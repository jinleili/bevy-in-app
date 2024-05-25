use core_graphics::geometry::CGRect;
use objc::{runtime::Object, *};
use raw_window_handle::{
    DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
    RawWindowHandle, UiKitDisplayHandle, UiKitWindowHandle, WindowHandle,
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

#[derive(Debug)]
pub struct AppView {
    pub view_obj: super::SendSyncWrapper<IOSViewObj>,
}

impl std::ops::Deref for AppView {
    type Target = IOSViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj.0
    }
}

impl AppView {
    pub fn new(view_obj: IOSViewObj) -> Self {
        Self {
            view_obj: super::SendSyncWrapper(view_obj),
        }
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        let s: CGRect = unsafe { msg_send![self.view, frame] };
        (s.size.width as f32, s.size.height as f32)
    }
}

impl HasWindowHandle for AppView {
    fn window_handle(&self) -> Result<WindowHandle, HandleError> {
        Ok(unsafe {
            WindowHandle::borrow_raw(RawWindowHandle::UiKit(UiKitWindowHandle::new({
                let ui_view = self.view as _;
                std::ptr::NonNull::new(ui_view).unwrap()
            })))
        })
    }
}

impl HasDisplayHandle for AppView {
    fn display_handle(&self) -> Result<DisplayHandle, HandleError> {
        Ok(
            unsafe {
                DisplayHandle::borrow_raw(RawDisplayHandle::UiKit(UiKitDisplayHandle::new()))
            },
        )
    }
}
