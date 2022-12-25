use super::AppView;
use bevy::utils::HashMap;
use bevy::window::{RawHandleWrapper, Window, WindowDescriptor, WindowId};
use raw_window_handle::{HasRawDisplayHandle, HasRawWindowHandle};

#[derive(Debug, Default)]
pub struct AppViews {
    pub views: HashMap<WindowId, AppView>,
}

impl AppViews {
    pub fn create_window(
        &mut self,
        #[cfg(target_os = "ios")] view_obj: super::IOSViewObj,
        #[cfg(target_os = "android")] view_obj: super::AndroidViewObj,
        window_id: WindowId,
        window_descriptor: &WindowDescriptor,
    ) -> Window {
        let app_view = AppView::new(view_obj);
        let scale_factor = app_view.scale_factor;
        let inner_size = app_view.inner_size();
        let raw_handle = RawHandleWrapper {
            window_handle: app_view.raw_window_handle(),
            display_handle: app_view.raw_display_handle(),
        };
        self.views.insert(window_id, app_view);
        bevy::log::info!("----- size: {:?}, {}", inner_size, scale_factor);
        Window::new(
            window_id,
            window_descriptor,
            inner_size.0,
            inner_size.1,
            scale_factor.into(),
            None,
            Some(raw_handle),
        )
    }
}
