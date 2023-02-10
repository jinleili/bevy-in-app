use jni::sys::jobject;
use jni::JNIEnv;
use raw_window_handle::{
    AndroidDisplayHandle, AndroidNdkWindowHandle, HasRawDisplayHandle, HasRawWindowHandle,
    RawDisplayHandle, RawWindowHandle,
};

#[derive(Debug, Copy, Clone)]
pub struct AndroidViewObj {
    pub native_window: *mut ndk_sys::ANativeWindow,
    pub scale_factor: f32,
}

impl Default for AndroidViewObj {
    fn default() -> Self {
        Self {
            native_window: std::ptr::null_mut(),
            scale_factor: 2.0,
        }
    }
}

#[derive(Debug, Default)]
pub struct AppView {
    pub view_obj: AndroidViewObj,
}

impl std::ops::Deref for AppView {
    type Target = AndroidViewObj;
    fn deref(&self) -> &Self::Target {
        &self.view_obj
    }
}

impl AppView {
    pub fn get_native_window(env: *mut JNIEnv, surface: jobject) -> *mut ndk_sys::ANativeWindow {
        unsafe {
            // 获取与安卓端 surface 对象关联的 ANativeWindow，以便能通过 Rust 与之交互。
            // 此函数在返回 ANativeWindow 的同时会自动将其引用计数 +1，以防止该对象在安卓端被意外释放。
            ndk_sys::ANativeWindow_fromSurface(env as _, surface as _)
        }
    }

    pub fn new(view_obj: AndroidViewObj) -> Self {
        Self { view_obj }
    }

    pub fn logical_resolution(&self) -> (f32, f32) {
        (
            self.get_width() as f32 / self.scale_factor,
            self.get_height() as f32 / self.scale_factor,
        )
    }

    fn get_width(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getWidth(self.native_window) as u32 }
    }

    fn get_height(&self) -> u32 {
        unsafe { ndk_sys::ANativeWindow_getHeight(self.native_window) as u32 }
    }
}

unsafe impl HasRawWindowHandle for AppView {
    fn raw_window_handle(&self) -> RawWindowHandle {
        let mut handle = AndroidNdkWindowHandle::empty();
        handle.a_native_window = self.native_window as _;
        RawWindowHandle::AndroidNdk(handle)
    }
}

unsafe impl HasRawDisplayHandle for AppView {
    fn raw_display_handle(&self) -> RawDisplayHandle {
        RawDisplayHandle::Android(AndroidDisplayHandle::empty())
    }
}

impl Drop for AppView {
    fn drop(&mut self) {
        unsafe {
            ndk_sys::ANativeWindow_release(self.native_window);
        }
    }
}
