use ndk::asset::AssetManager;

pub struct AndroidAssetManager(pub *mut ndk_sys::AAssetManager);

impl Default for AndroidAssetManager {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}