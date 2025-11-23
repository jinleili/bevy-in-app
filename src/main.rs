#[cfg(any(target_os = "android", target_os = "ios"))]
fn main() {}

#[cfg(not(any(target_os = "android", target_os = "ios")))]
fn main() {
    let mut bevy_app = bevy_in_app::create_app();
    bevy_app.run();
}
