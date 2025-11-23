use bevy::prelude::*;

#[cfg(any(target_os = "android", target_os = "ios"))]
use bevy::ecs::{
    entity::Entity,
    system::{Commands, Query, SystemState},
};
#[cfg(any(target_os = "android", target_os = "ios"))]
use bevy::input::{
    ButtonState,
    keyboard::{Key, KeyboardInput},
};

#[cfg(any(target_os = "android", target_os = "ios"))]
mod app_view;

#[cfg(any(target_os = "android", target_os = "ios"))]
mod ffi;
#[cfg(any(target_os = "android", target_os = "ios"))]
pub use ffi::*;

#[cfg(target_os = "android")]
mod android_asset_io;

mod simple_demo;

/// Creates and configures a Bevy app for cross-platform use (desktop, Android, iOS).
#[allow(unused_variables)]
pub fn create_app(
    #[cfg(target_os = "android")] android_asset_manager: android_asset_io::AndroidAssetManager,
) -> App {
    let mut app = App::new();
    
    // Insert Android asset manager resource if on Android
    #[cfg(target_os = "android")]
    app.insert_non_send_resource(android_asset_manager);
    
    // Configure plugins for the target platform
    let plugins = configure_plugins();
    
    app.insert_resource(ClearColor(Color::srgb(0.1, 0.1, 0.1)))
        .add_plugins(plugins);

    // Add platform-specific plugins
    #[cfg(any(target_os = "android", target_os = "ios"))]
    app.add_plugins(app_view::AppViewPlugin);

    // Add the demo plugin
    app.add_plugins(simple_demo::SimpleDemoPlugin);

    // In this scenario, need to call the setup() of the plugins that have been registered
    // in the App manually.
    // https://github.com/bevyengine/bevy/issues/7576
    // bevy 0.11 changed: https://github.com/bevyengine/bevy/pull/8336
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        use bevy::app::PluginsState;
        if app.plugins_state() == PluginsState::Ready {}
        app.finish();
        app.cleanup();
    }

    app
}

/// Configures Bevy plugins based on the target platform.
#[allow(unused_mut)]
fn configure_plugins() -> bevy::app::PluginGroupBuilder {
    #[allow(unused_imports)]
    use bevy::winit::WinitPlugin;

    let mut plugins = DefaultPlugins.build();

    // Disable winit on mobile platforms (we use native windowing)
    #[cfg(any(target_os = "android", target_os = "ios"))]
    {
        plugins = plugins
            .disable::<WinitPlugin>()
            .set(WindowPlugin::default());
    }

    // Configure Android-specific settings
    #[cfg(target_os = "android")]
    {
        use bevy::render::{
            RenderPlugin,
            settings::{RenderCreation, WgpuSettings},
        };
        
        plugins = plugins.set(RenderPlugin {
            render_creation: RenderCreation::Automatic(WgpuSettings {
                backends: Some(wgpu::Backends::VULKAN),
                ..default()
            }),
            ..default()
        });

        // The custom asset IO plugin must be inserted before AssetPlugin.
        // This is required because Bevy's default AssetPlugin relies on winit,
        // which we don't use on Android.
        plugins = plugins
            .add_before::<bevy::asset::AssetPlugin>(android_asset_io::AndroidAssetIoPlugin);
    }

    plugins
}


#[cfg(any(target_os = "android", target_os = "ios"))]
pub(crate) fn change_input(app: &mut App, key_code: KeyCode, state: ButtonState) {
    let mut windows_system_state: SystemState<Query<(Entity, &mut Window)>> =
        SystemState::from_world(app.world_mut());
    let windows = windows_system_state.get_mut(app.world_mut());
    if let Ok((entity, _)) = windows.single() {
        let input = KeyboardInput {
            logical_key: if key_code == KeyCode::ArrowLeft {
                Key::ArrowLeft
            } else {
                Key::ArrowRight
            },
            state,
            key_code: key_code,
            window: entity,
            repeat: false,
            text: None,
        };
        app.world_mut().write_message(input);
    }
}

#[cfg(any(target_os = "android", target_os = "ios"))]
#[allow(clippy::type_complexity)]
pub(crate) fn close_bevy_window(mut app: Box<App>) {
    let mut windows_state: SystemState<(
        Commands,
        Query<(Entity, &mut Window)>,
        MessageWriter<AppExit>,
    )> = SystemState::from_world(app.world_mut());
    let (mut commands, windows, mut app_exit_events) = windows_state.get_mut(app.world_mut());
    for (window, _focus) in windows.iter() {
        commands.entity(window).despawn();
    }
    app_exit_events.write(AppExit::Success);
    windows_state.apply(app.world_mut());

    app.update();
}
