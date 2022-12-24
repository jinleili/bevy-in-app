use crate::app_view::{AndroidViewObj, AppView};
use bevy::input::ButtonState;
use bevy::prelude::*;
use jni::objects::JClass;
use jni::sys::{jfloat, jint, jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;

#[link(name = "c++_shared")]
extern "C" {}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn create_bevy_app(
    env: *mut JNIEnv,
    _: JClass,
    surface: jobject,
    scale_factor: jfloat,
) -> jlong {
    log_panics::init();
    let mut bevy_app = crate::create_breakout_app();
    let android_obj = AndroidViewObj {
        native_window: AppView::get_native_window(env, surface),
        scale_factor: scale_factor as _,
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
pub fn device_motion(_env: *mut JNIEnv, _: JClass, obj: jlong, x: jfloat, _y: jfloat, _z: jfloat) {
    let app = unsafe { &mut *(obj as *mut App) };
    let x: f32 = x as _;
    if x < -0.2 {
        crate::change_input(app, KeyCode::Left, ButtonState::Released);
        crate::change_input(app, KeyCode::Right, ButtonState::Pressed);
    } else if x > 0.2 {
        crate::change_input(app, KeyCode::Right, ButtonState::Released);
        crate::change_input(app, KeyCode::Left, ButtonState::Pressed);
    } else {
        crate::change_input(app, KeyCode::Left, ButtonState::Released);
        crate::change_input(app, KeyCode::Right, ButtonState::Released);
    }
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn release_bevy_app(_env: *mut JNIEnv, _: JClass, obj: jlong) {
    let app: Box<App> = unsafe { Box::from_raw(obj as *mut _) };
    crate::close_bevy_window(app);
}
