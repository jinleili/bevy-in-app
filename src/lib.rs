#[cfg(any(target_os = "android", target_os = "ios"))]
mod app_view;

// #[cfg_attr(target_os = "ios", path = "ffi/ios.rs")]
// #[cfg_attr(target_os = "android", path = "ffi/android.rs", allow(non_snake_case))]
#[cfg(any(target_os = "android", target_os = "ios"))]
mod ffi;
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use ffi::*;

#[derive(Resource)]
pub struct AppWindowSize {
    pub size: Vec2,
}

impl std::ops::Deref for AppWindowSize {
    type Target = Vec2;
    fn deref(&self) -> &Self::Target {
        &self.size
    }
}

use bevy::prelude::*;
#[allow(unused_imports)]
use bevy::winit::WinitPlugin;

mod breakout;
pub fn create_breakout_app() -> App {
    use bevy::time::FixedTimestep;
    use breakout::*;
    let mut bevy_app = App::new();
    #[allow(unused_mut)]
    let mut default_plugins = DefaultPlugins.build();

    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        default_plugins = default_plugins.disable::<WinitPlugin>().set(WindowPlugin {
            window: WindowDescriptor {
                resizable: false,
                mode: WindowMode::BorderlessFullscreen,
                ..default()
            },
            ..default()
        });

        if cfg!(target_os = "android") {
            default_plugins = default_plugins.disable::<bevy::audio::AudioPlugin>();
        }
    }
    bevy_app
        .insert_resource(ClearColor(Color::rgb(0.8, 0.4, 0.6)))
        .add_plugins(default_plugins);

    #[cfg(any(target_os = "android", target_os = "ios"))]
    bevy_app.add_plugin(app_view::AppViewPlugin);
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    bevy_app.add_system(bevy::window::close_on_esc);

    let mut system_set = SystemSet::new();
    system_set = system_set
        .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
        .with_system(check_for_collisions)
        .with_system(move_paddle.before(check_for_collisions))
        .with_system(apply_velocity.before(check_for_collisions));
    if cfg!(not(target_os = "android")) {
        system_set = system_set.with_system(play_collision_sound.after(check_for_collisions));
    }

    bevy_app
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(system_set)
        .add_system(update_scoreboard);

    bevy_app
}
