use super::{AppView, AppViewWindow};
use bevy::ecs::entity::Entity;
use bevy_platform::collections::HashMap;
use bevy::window::WindowWrapper;

#[derive(Debug, Default)]
pub struct AppViews {
    views: HashMap<super::WindowId, AppViewWindow>,
    entity_to_window_id: HashMap<Entity, super::WindowId>,
}

impl AppViews {
    pub fn create_window(
        &mut self,
        #[cfg(target_os = "ios")] view_obj: super::IOSViewObj,
        #[cfg(target_os = "android")] view_obj: super::AndroidViewObj,
        entity: Entity,
    ) -> &AppViewWindow {
        let app_view = AppViewWindow(WindowWrapper::new(AppView::new(view_obj)));
        let window_id = super::WindowId::new();
        self.entity_to_window_id.insert(entity, window_id);

        self.views.entry(window_id).insert(app_view).into_mut()
    }

    /// Get the AppView that is associated with our entity.
    pub fn get_view(&self, entity: Entity) -> Option<&AppViewWindow> {
        self.entity_to_window_id
            .get(&entity)
            .and_then(|window_id| self.views.get(window_id))
    }

    /// This should mostly just be called when the window is closing.
    pub fn remove_view(&mut self, entity: Entity) -> Option<AppViewWindow> {
        let window_id = self.entity_to_window_id.remove(&entity)?;
        self.views.remove(&window_id)
    }
}
