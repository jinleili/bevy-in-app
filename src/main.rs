#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    use bevy::prelude::*;
    use bevy_in_app::{create_breakout_app, AppWindowSize};

    let window_size = AppWindowSize {
        size: Vec2::new(1280.0, 768.0),
    };

    let mut bevy_app = create_breakout_app();
    bevy_app.insert_resource(window_size).run();
}
