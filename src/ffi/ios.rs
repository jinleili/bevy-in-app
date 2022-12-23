use crate::app_view::{app_runner, IOSViewObj};
use bevy::input::{
    keyboard::KeyboardInput,
    touch::{TouchInput, TouchPhase},
    ButtonState,
};
use bevy::prelude::*;

#[no_mangle]
pub fn create_bevy_app(view: *mut objc::runtime::Object, scale_factor: f32) -> *mut libc::c_void {
    println!("create_bevy_app ---- 0");
    let mut bevy_app = crate::create_breakout_app();
    println!("create_breakout_app ----");

    let ios_obj = IOSViewObj { view, scale_factor };
    bevy_app.insert_non_send_resource(ios_obj);
    println!("insert_non_send_resource ----");

    app_runner(&mut bevy_app);

    let box_obj = Box::new(bevy_app);
    // into_raw 返回指针的同时，将此对象的内存管理权转交给调用方
    Box::into_raw(box_obj) as *mut libc::c_void
}

#[no_mangle]
pub fn enter_frame(obj: *mut libc::c_void) {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(obj as *mut App) };
    app.update();
}

#[no_mangle]
pub fn touch_started(obj: *mut libc::c_void, x: f32, y: f32) {
    // 使用逻辑像素位置
    touched(obj, TouchPhase::Started, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_moved(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Moved, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_ended(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Ended, Vec2::new(x, y));
}

#[no_mangle]
pub fn touch_cancelled(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Cancelled, Vec2::new(x, y));
}

fn touched(obj: *mut libc::c_void, phase: TouchPhase, position: Vec2) {
    let touch = TouchInput {
        phase,
        position,
        force: None,
        id: 0,
    };
    let app = unsafe { &mut *(obj as *mut App) };
    app.world.cell().send_event(touch);
}

#[no_mangle]
pub fn gyroscope_motion(_obj: *mut libc::c_void, _x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub fn accelerometer_motion(_obj: *mut libc::c_void, _x: f32, _y: f32, _z: f32) {}

#[no_mangle]
pub fn device_motion(obj: *mut libc::c_void, x: f32, _y: f32, _z: f32) {
    let mut input = KeyboardInput {
        scan_code: 0,
        state: ButtonState::Pressed,
        key_code: None,
    };
    let app = unsafe { &mut *(obj as *mut App) };
    if x > 0.005 {
        release_input(app, KeyCode::Left);
        input.scan_code = 124;
        input.key_code = Some(KeyCode::Right);
    } else if x < -0.005 {
        release_input(app, KeyCode::Right);
        input.scan_code = 123;
        input.key_code = Some(KeyCode::Left);
    } else {
        release_input(app, KeyCode::Left);
        release_input(app, KeyCode::Right);
    }
    if input.key_code.is_some() {
        app.world.cell().send_event(input);
    }
}

fn release_input(app: &mut App, key_code: KeyCode) {
    let input = KeyboardInput {
        scan_code: if key_code == KeyCode::Left { 123 } else { 124 },
        state: ButtonState::Released,
        key_code: Some(key_code),
    };
    app.world.cell().send_event(input);
}

#[no_mangle]
pub fn release_bevy_app(obj: *mut libc::c_void) {
    // 将指针转换为其指代的实际 Rust 对象，同时也拿回此对象的内存管理权
    let mut app: Box<App> = unsafe { Box::from_raw(obj as *mut _) };
    let mut windows = app.world.resource_mut::<Windows>();
    if let Some(window) = windows.get_focused_mut() {
        window.close();
    }
}
