use bevy::app::{App, CoreStage, Plugin};
use bevy::ecs::{
    event::{Events, ManualEventReader},
    prelude::*,
    world::World,
};
use bevy::math::Vec2;
use bevy::window::{
    CreateWindow, ModifiesWindows, WindowClosed, WindowCreated, WindowResized,
    WindowScaleFactorChanged, Windows,
};

#[cfg_attr(target_os = "ios", path = "ios.rs")]
#[cfg_attr(target_os = "android", path = "android.rs")]
mod app_view;
pub use app_view::*;

mod app_views;
use app_views::AppViews;

#[derive(Default, Resource)]
struct AppCreateWindowReader(ManualEventReader<CreateWindow>);

pub struct AppViewPlugin;
impl Plugin for AppViewPlugin {
    fn build(&self, app: &mut App) {
        app.init_non_send_resource::<AppViews>()
            .add_system_to_stage(CoreStage::PostUpdate, change_window.label(ModifiesWindows));

        let create_window_reader = AppCreateWindowReader::default();
        app.insert_resource(create_window_reader);
    }
}

fn change_window(
    mut _app_views: NonSendMut<AppViews>,
    mut _windows: ResMut<Windows>,
    mut _window_dpi_changed_events: EventWriter<WindowScaleFactorChanged>,
    mut _window_close_events: EventWriter<WindowClosed>,
) {
}

#[allow(unused)]
pub fn app_runner(app: &mut App) {
    let mut create_window_event_reader = app
        .world
        .remove_resource::<AppCreateWindowReader>()
        .unwrap()
        .0;
    handle_create_window_events(&mut app.world, &mut create_window_event_reader);
    let size = if let Some(a_window) = app.world.cell().resource_mut::<Windows>().get_focused() {
        Vec2::new(a_window.width(), a_window.height())
    } else {
        Vec2::ZERO
    };
    app.insert_resource(crate::AppWindowSize { size });
}

fn handle_create_window_events(
    world: &mut World,
    create_window_event_reader: &mut ManualEventReader<CreateWindow>,
) {
    #[cfg(target_os = "ios")]
    let view_obj = world.remove_non_send_resource::<IOSViewObj>().unwrap();
    #[cfg(target_os = "android")]
    let view_obj = world.remove_non_send_resource::<AndroidViewObj>().unwrap();

    let world = world.cell();
    let mut app_views = world.non_send_resource_mut::<AppViews>();
    let mut windows = world.resource_mut::<Windows>();
    let create_window_events = world.resource::<Events<CreateWindow>>();
    for create_window_event in create_window_event_reader.iter(&create_window_events) {
        let window = app_views.create_window(
            view_obj,
            create_window_event.id,
            &create_window_event.descriptor,
        );
        world.send_event(WindowResized {
            id: create_window_event.id,
            width: window.width(),
            height: window.height(),
        });
        windows.add(window);

        world.send_event(WindowCreated {
            id: create_window_event.id,
        });
    }
}
