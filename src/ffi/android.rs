use crate::android_asset_io::AndroidAssetManager;
use crate::app_view::{AndroidViewObj, NativeWindow};
use android_logger::Config;
use bevy::input::ButtonState;
use bevy::prelude::*;
use jni::sys::{jfloat, jlong, jobject};
use jni::JNIEnv;
use jni_fn::jni_fn;
use log::LevelFilter;

#[link(name = "c++_shared")]
extern "C" {}

#[no_mangle]
pub fn android_main(_android_app: bevy::winit::android_activity::AndroidApp) {
    // This maybe a bevy issue
    // `android_main` empty function is currently required, otherwise, a panic will occur:
    //
    // java.lang.UnsatisfiedLinkError: dlopen failed: cannot locate symbol "android_main"
    // referenced by "/data/app/~~hebB-d3x4YdYjuFlqiJT3w==/name.jinleili.bevy.debug-j2uCKW7h8U7-_YzEOO48Dg==/base.apk!/lib/arm64-v8a/libbevy_in_app.so"...
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn init_ndk_context(env: JNIEnv, _: jobject, context: jobject) {
    log_panics::init();
    android_logger::init_once(Config::default().with_max_level(LevelFilter::Info));
    let java_vm = env.get_java_vm().unwrap();
    unsafe {
        ndk_context::initialize_android_context(java_vm.get_java_vm_pointer() as _, context as _);
    }
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn create_bevy_app(
    env: *mut JNIEnv,
    _: jobject,
    asset_manager: jobject,
    surface: jobject,
    scale_factor: jfloat,
) -> jlong {
    let a_asset_manager = unsafe { ndk_sys::AAssetManager_fromJava(env as _, asset_manager) };
    let android_obj = AndroidViewObj {
        native_window: NativeWindow::new(env, surface),
        scale_factor: scale_factor as _,
    };

    let mut bevy_app = crate::create_bevy_app(AndroidAssetManager(a_asset_manager));
    bevy_app.insert_non_send_resource(android_obj);
    crate::app_view::create_bevy_window(&mut bevy_app);
    log::info!("Bevy App created!");

    Box::into_raw(Box::new(bevy_app)) as jlong
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn is_preparation_completed(_env: *mut JNIEnv, _: jobject, obj: jlong) -> u32 {
    let bevy_app = unsafe { &mut *(obj as *mut App) };
    crate::is_preparation_completed(bevy_app)
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn enter_frame(_env: *mut JNIEnv, _: jobject, obj: jlong) {
    let bevy_app = unsafe { &mut *(obj as *mut App) };
    bevy_app.update();
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn device_motion(_env: *mut JNIEnv, _: jobject, obj: jlong, x: jfloat, _y: jfloat, _z: jfloat) {
    let app = unsafe { &mut *(obj as *mut App) };
    let x: f32 = x as _;
    if x < -0.2 {
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Pressed);
    } else if x > 0.2 {
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Pressed);
    } else {
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Released);
    }
}

#[no_mangle]
#[jni_fn("name.jinleili.bevy.RustBridge")]
pub fn release_bevy_app(_env: *mut JNIEnv, _: jobject, obj: jlong) {
    let app: Box<App> = unsafe { Box::from_raw(obj as *mut _) };
    crate::close_bevy_window(app);
}
