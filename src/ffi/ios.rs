use crate::app_view::{IOSViewObj, create_bevy_window};
use bevy::{
    ecs::system::SystemState,
    input::{
        ButtonState,
        touch::{TouchInput, TouchPhase},
    },
    prelude::*,
};

#[unsafe(no_mangle)]
pub fn create_bevy_app(view: *mut objc::runtime::Object, scale_factor: f32) -> *mut libc::c_void {
    let mut bevy_app = crate::create_app();
    let ios_obj = IOSViewObj { view, scale_factor };
    bevy_app.insert_non_send_resource(ios_obj);

    create_bevy_window(&mut bevy_app);

    info!("Bevy App created!");
    let box_obj = Box::new(bevy_app);
    // into_raw 返回指针的同时，将此对象的内存管理权转交给调用方
    Box::into_raw(box_obj) as *mut libc::c_void
}

#[unsafe(no_mangle)]
pub fn enter_frame(obj: *mut libc::c_void) {
    // 获取到指针指代的 Rust 对象的可变借用
    let app = unsafe { &mut *(obj as *mut App) };
    app.update();
}

#[unsafe(no_mangle)]
pub fn touch_started(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Started, Vec2::new(x, y));
}

#[unsafe(no_mangle)]
pub fn touch_moved(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Moved, Vec2::new(x, y));
}

#[unsafe(no_mangle)]
pub fn touch_ended(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Ended, Vec2::new(x, y));
}

#[unsafe(no_mangle)]
pub fn touch_cancelled(obj: *mut libc::c_void, x: f32, y: f32) {
    touched(obj, TouchPhase::Canceled, Vec2::new(x, y));
}

fn touched(obj: *mut libc::c_void, phase: TouchPhase, position: Vec2) {
    let app = unsafe { &mut *(obj as *mut App) };
    let mut windows_system_state: SystemState<Query<(Entity, &Window)>> =
        SystemState::from_world(app.world_mut());
    let (entity, _) = windows_system_state.get(app.world_mut()).single().unwrap();

    let touch = TouchInput {
        window: entity,
        phase,
        position,
        force: None,
        id: 0,
    };

    app.world_mut().write_message(touch);
}

#[unsafe(no_mangle)]
pub fn gyroscope_motion(_obj: *mut libc::c_void, _x: f32, _y: f32, _z: f32) {
    // TODO:
}

#[unsafe(no_mangle)]
pub fn accelerometer_motion(_obj: *mut libc::c_void, _x: f32, _y: f32, _z: f32) {
    // TODO:
}

#[unsafe(no_mangle)]
pub fn device_motion(obj: *mut libc::c_void, x: f32, _y: f32, _z: f32) {
    let app = unsafe { &mut *(obj as *mut App) };
    if x > 0.005 {
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Pressed);
    } else if x < -0.005 {
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Pressed);
    } else {
        crate::change_input(app, KeyCode::ArrowLeft, ButtonState::Released);
        crate::change_input(app, KeyCode::ArrowRight, ButtonState::Released);
    }
}

#[unsafe(no_mangle)]
pub fn release_bevy_app(obj: *mut libc::c_void) {
    // 将指针转换为其指代的实际 Rust 对象，同时也拿回此对象的内存管理权
    let app: Box<App> = unsafe { Box::from_raw(obj as *mut _) };
    crate::close_bevy_window(app);
}
