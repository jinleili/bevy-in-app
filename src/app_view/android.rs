use core::ffi::c_void;
use jni::sys::jobject;
use jni::JNIEnv;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, DisplayHandle, HandleError, HasDisplayHandle,
    HasWindowHandle, RawDisplayHandle, RawWindowHandle, WindowHandle,
};
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
pub struct AndroidViewObj {
    pub native_window: NativeWindow,
    pub scale_factor: f32,
}

#[derive(Debug)]
pub struct AppView {
    pub view_obj: super::SendSyncWrapper<AndroidViewObj>,
}

impl std::ops::Deref for AppView {
    type Target = AndroidViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj.0
    }
}

impl AppView {
    pub fn new(view_obj: AndroidViewObj) -> Self {
        Self {
            view_obj: super::SendSyncWrapper(view_obj),
        }
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        (
            self.get_width() as f32 / self.scale_factor,
            self.get_height() as f32 / self.scale_factor,
        )
    }

    fn get_width(&self) -> u32 {
        self.native_window.get_width()
    }

    fn get_height(&self) -> u32 {
        self.native_window.get_height()
    }
}

impl HasWindowHandle for AppView {
    fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
        unsafe {
            let a_native_window = self.native_window.a_native_window.lock().unwrap();
            let handle = AndroidNdkWindowHandle::new(
                std::ptr::NonNull::new(*a_native_window as *mut _ as *mut c_void).unwrap(),
            );
            Ok(WindowHandle::borrow_raw(RawWindowHandle::AndroidNdk(
                handle,
            )))
        }
    }
}

impl HasDisplayHandle for AppView {
    fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
        unsafe {
            Ok(DisplayHandle::borrow_raw(RawDisplayHandle::Android(
                AndroidDisplayHandle::new(),
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct NativeWindow {
    a_native_window: Arc<Mutex<*mut ndk_sys::ANativeWindow>>,
}

impl Default for NativeWindow {
    fn default() -> Self {
        Self {
            a_native_window: Arc::new(Mutex::new(std::ptr::null_mut())),
        }
    }
}

impl NativeWindow {
    pub fn new(env: *mut JNIEnv, surface: jobject) -> Self {
        let a_native_window = unsafe {
            // 获取与安卓端 surface 对象关联的 ANativeWindow，以便能通过 Rust 与之交互。
            // 此函数在返回 ANativeWindow 的同时会自动将其引用计数 +1，以防止该对象在安卓端被意外释放。
            ndk_sys::ANativeWindow_fromSurface(env as *mut _, surface as *mut _)
        };
        Self {
            a_native_window: Arc::new(Mutex::new(a_native_window)),
        }
    }

    fn get_width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(*self.a_native_window.lock().unwrap()) as u32 }
    }

    fn get_height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(*self.a_native_window.lock().unwrap()) as u32 }
    }
}

impl Drop for NativeWindow {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(*self.a_native_window.lock().unwrap());
        }
    }
}

unsafe impl Send for NativeWindow {}
unsafe impl Sync for NativeWindow {}
