use bevy::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod app_view;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod ffi;
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use ffi::*;

#[cfg(target_os = "android")]
mod android_asset_io;

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

mod breakout;
#[allow(unused_variables)]
pub fn create_breakout_app(
    #[cfg(target_os = "android")] android_asset_manager: android_asset_io::AndroidAssetManager,
) -> App {
    use bevy::time::FixedTimestep;
    #[allow(unused_imports)]
    use bevy::winit::WinitPlugin;
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
    }

    #[cfg(target_os = "android")]
    {
        bevy_app.insert_non_send_resource(android_asset_manager);

        use bevy::render::{settings::WgpuSettings, RenderPlugin};
        default_plugins = default_plugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                backends: Some(wgpu::Backends::VULKAN),
                ..default()
            },
        });
        // the custom asset io plugin must be inserted in-between the
        // `CorePlugin' and `AssetPlugin`. It needs to be after the
        // CorePlugin, so that the IO task pool has already been constructed.
        // And it must be before the `AssetPlugin` so that the asset plugin
        // doesn't create another instance of an asset server. In general,
        // the AssetPlugin should still run so that other aspects of the
        // asset system are initialized correctly.
        default_plugins = default_plugins
            .add_before::<bevy::asset::AssetPlugin, _>(android_asset_io::AndroidAssetIoPlugin);
    }
    bevy_app
        .insert_resource(ClearColor(Color::rgb(0.8, 0.4, 0.6)))
        .add_plugins(default_plugins);

    #[cfg(any(target_os = "android", target_os = "ios"))]
    bevy_app.add_plugin(app_view::AppViewPlugin);
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    bevy_app.add_system(bevy::window::close_on_esc);

    bevy_app
        .insert_resource(Scoreboard { score: 0 })
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_startup_system(setup)
        .add_event::<CollisionEvent>()
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(TIME_STEP as f64))
                .with_system(check_for_collisions)
                .with_system(move_paddle.before(check_for_collisions))
                .with_system(apply_velocity.before(check_for_collisions))
                .with_system(play_collision_sound.after(check_for_collisions)),
        )
        .add_system(update_scoreboard);

    bevy_app
}

use bevy::input::{keyboard::KeyboardInput, ButtonState};

#[allow(unused)]
pub(crate) fn change_input(app: &mut App, key_code: KeyCode, state: ButtonState) {
    let input = KeyboardInput {
        scan_code: if key_code == KeyCode::Left { 123 } else { 124 },
        state,
        key_code: Some(key_code),
    };
    app.world.cell().send_event(input);
}

#[allow(unused)]
pub(crate) fn close_bevy_window(mut app: Box<App>) {
    let mut windows = app.world.resource_mut::<Windows>();
    if let Some(window) = windows.get_focused_mut() {
        window.close();
    }
}
