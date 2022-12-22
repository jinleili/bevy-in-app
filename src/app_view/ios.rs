use bevy_window::{RawHandleWrapper, Window, WindowDescriptor, WindowId};
use core_graphics::geometry::CGRect;
use objc::{runtime::Object, *};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, UiKitDisplayHandle,
    UiKitWindowHandle,
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct IOSViewObj {
    pub view: *mut Object,
    pub scale_factor: f32,
}

impl Default for IOSViewObj {
    fn default() -> Self {
        Self {
            view: std::ptr::null_mut(),
            scale_factor: 1.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppView {
    pub view_obj: IOSViewObj,
}

impl AppView {
    pub fn new(view_obj: IOSViewObj) -> Self {
        Self { view_obj }
    }

    pub fn scale_factor(&self) -> f32 {
        self.view_obj.scale_factor
    }

    pub fn inner_size(&self) -> (u32, u32) {
        let logical_res = self.logical_resolution();
        (
            (logical_res.0 * self.view_obj.scale_factor) as u32,
            (logical_res.1 * self.view_obj.scale_factor) as u32,
        )
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        let s: CGRect = unsafe { msg_send![self.view_obj.view, frame] };
        (s.size.width as f32, s.size.height as f32)
    }
}

unsafe impl HasRawWindowHandle for AppView {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = UiKitWindowHandle::empty();
        handle.ui_view = self.view_obj.view as _;
        RawWindowHandle::UiKit(handle)
    }
}

unsafe impl HasRawDisplayHandle for AppView {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::UiKit(UiKitDisplayHandle::empty())
    }
}

impl super::AppViews {
    pub fn create_window(
        &mut self,
        view_obj: IOSViewObj,
        window_id: WindowId,
        window_descriptor: &WindowDescriptor,
    ) -> Window {
        let app_view = AppView::new(view_obj);
        let scale_factor = app_view.scale_factor();
        let inner_size = app_view.inner_size();
        let raw_handle = RawHandleWrapper {
            window_handle: app_view.raw_window_handle(),
            display_handle: app_view.raw_display_handle(),
        };
        self.views.insert(window_id, app_view);

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
