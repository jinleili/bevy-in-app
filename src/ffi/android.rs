use crate::app_view::{AndroidViewObj, AppView};
use bevy::input::{
    keyboard::KeyboardInput,
    touch::{TouchInput, TouchPhase},
    ButtonState,
};
use bevy::prelude::*;
use jni::objects::JClass;
use jni::sys::{jint, jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;

#[link(name = "c++_shared")]
extern "C" {}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn create_bevy_app(env: *mut JNIEnv, _: JClass, surface: jobject) -> jlong {
    log_panics::init();
    let mut bevy_app = crate::create_breakout_app();

    let android_obj = AndroidViewObj {
        native_window: AppView::get_native_window(env, surface),
        scale_factor: 2.0,
    };

    bevy_app.insert_non_send_resource(android_obj);

    crate::app_view::app_runner(&mut bevy_app);

    info!("Bevy App created!");
    Box::into_raw(Box::new(bevy_app)) as jlong
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn enter_frame(_env: *mut JNIEnv, _: JClass, obj: jlong) {
    let obj = unsafe { &mut *(obj as *mut App) };
    obj.update();
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn release_bevy_app(_env: *mut JNIEnv, _: JClass, obj: jlong) {
    let mut app: Box<App> = unsafe { Box::from_raw(obj as *mut _) };
    let mut windows = app.world.resource_mut::<Windows>();
    if let Some(window) = windows.get_focused_mut() {
        window.close();
    }
}
