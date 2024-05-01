use bevy::{
    asset::io::{
        AssetReader, AssetReaderError, AssetSource, AssetSourceId, PathStream, Reader, VecReader,
    },
    prelude::*,
    utils::BoxedFuture,
};
use ndk::asset::AssetManager;
use std::{
    ffi::CString,
    path::{Path, PathBuf},
};

/// *mut ndk_sys::AAssetManager 无法实现 send
pub static ASSET_MANAGER: std::sync::OnceLock<AssetManager> = std::sync::OnceLock::new();

pub struct AndroidAssetManager(pub *mut ndk_sys::AAssetManager);

impl Default for AndroidAssetManager {
    fn default() -> Self {
        Self(std::ptr::null_mut())
    }
}

pub struct AndroidAssetIoPlugin;

impl Plugin for AndroidAssetIoPlugin {
    fn build(&self, app: &mut App) {
        let android_asset_manager = app
            .world_mut()
            .remove_non_send_resource::<AndroidAssetManager>()
            .unwrap();
        let asset_manager = unsafe {
            AssetManager::from_ptr(std::ptr::NonNull::new(android_asset_manager.0).unwrap())
        };
        let _ = ASSET_MANAGER.set(asset_manager);

        // override bevy default asset reader
        // https://github.com/bevyengine/bevy/pull/9885
        app.register_asset_source(
            AssetSourceId::Default,
            AssetSource::build()
                .with_reader(|| Box::new(AndroidAssetIo::new("assets".to_string()))),
        );
    }
}

#[allow(dead_code)]
struct AndroidAssetIo {
    root_path: PathBuf,
}

impl AndroidAssetIo {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        AndroidAssetIo {
            root_path: path.as_ref().to_owned(),
        }
    }
}

impl AssetReader for AndroidAssetIo {
    fn read<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let mut opened_asset = ASSET_MANAGER
                .get()
                .unwrap()
                .open(&CString::new(path.to_str().unwrap()).unwrap())
                .ok_or(AssetReaderError::NotFound(path.to_path_buf()))?;
            let bytes = opened_asset.get_buffer()?;
            let reader: Box<Reader> = Box::new(VecReader::new(bytes.to_vec()));
            Ok(reader)
        })
    }

    fn read_directory<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<PathStream>, AssetReaderError>> {
        error!("Reading directories is not supported with the AndroidAssetReader");
        Box::pin(async move { Err(AssetReaderError::NotFound(path.to_path_buf())) })
    }

    fn read_meta<'a>(
        &'a self,
        path: &'a Path,
    ) -> BoxedFuture<'a, Result<Box<Reader<'a>>, AssetReaderError>> {
        Box::pin(async move {
            let meta_path = get_meta_path(path);
            let mut opened_asset = ASSET_MANAGER
                .get()
                .unwrap()
                .open(&CString::new(meta_path.to_str().unwrap()).unwrap())
                .ok_or(AssetReaderError::NotFound(meta_path))?;
            let bytes = opened_asset.get_buffer()?;
            let reader: Box<Reader> = Box::new(VecReader::new(bytes.to_vec()));
            Ok(reader)
        })
    }

    fn is_directory<'a>(
        &'a self,
        _path: &'a Path,
    ) -> BoxedFuture<'a, std::result::Result<bool, AssetReaderError>> {
        error!("Reading directories is not supported with the AndroidAssetReader");
        Box::pin(async move { Ok(false) })
    }
}

/// Appends `.meta` to the given path.
pub(crate) fn get_meta_path(path: &Path) -> PathBuf {
    let mut meta_path = path.to_path_buf();
    let mut extension = path
        .extension()
        .expect("asset paths must have extensions")
        .to_os_string();
    extension.push(".meta");
    meta_path.set_extension(extension);
    meta_path
}
