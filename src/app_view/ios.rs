use core_graphics::geometry::CGRect;
use objc::{runtime::Object, *};
use raw_window_handle::{
    HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle, UiKitDisplayHandle,
    UiKitWindowHandle,
};

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

impl std::ops::Deref for AppView {
    type Target = IOSViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj
    }
}

impl AppView {
    pub fn new(view_obj: IOSViewObj) -> Self {
        Self { view_obj }
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        let s: CGRect = unsafe { msg_send![self.view, frame] };
        (s.size.width as f32, s.size.height as f32)
    }
}

unsafe impl HasRawWindowHandle for AppView {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = UiKitWindowHandle::empty();
        handle.ui_view = self.view as _;
        RawWindowHandle::UiKit(handle)
    }
}

unsafe impl HasRawDisplayHandle for AppView {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::UiKit(UiKitDisplayHandle::empty())
    }
}
