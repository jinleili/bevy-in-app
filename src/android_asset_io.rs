use bevy::{
    asset::{AssetIo, AssetIoError, ChangeWatcher, Metadata},
    prelude::*,
    utils::BoxedFuture,
};
use ndk::asset::AssetManager;
use std::{
    convert::TryFrom,
    ffi::CString,
    path::{Path, PathBuf},
};

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
            .world
            .remove_non_send_resource::<AndroidAssetManager>()
            .unwrap();
        let asset_manager = unsafe {
            AssetManager::from_ptr(std::ptr::NonNull::new(android_asset_manager.0).unwrap())
        };
        // create the custom AndroidAssetIo instance
        let asset_io = AndroidAssetIo::new("assets".to_string(), asset_manager);

        // the asset server is constructed and added the resource manager
        app.insert_resource(AssetServer::new(asset_io));
    }
}

struct AndroidAssetIo {
    root_path: PathBuf,
    asset_manager: AssetManager,
}

impl AndroidAssetIo {
    pub fn new<P: AsRef<Path>>(path: P, asset_manager: AssetManager) -> Self {
        AndroidAssetIo {
            root_path: path.as_ref().to_owned(),
            asset_manager,
        }
    }
}

impl AssetIo for AndroidAssetIo {
    fn load_path<'a>(&'a self, path: &'a Path) -> BoxedFuture<'a, Result<Vec<u8>, AssetIoError>> {
        Box::pin(async move {
            let mut opened_asset = self
                .asset_manager
                .open(&CString::new(path.to_str().unwrap()).unwrap())
                .ok_or(AssetIoError::NotFound(path.to_path_buf()))?;
            let bytes = opened_asset.get_buffer()?;
            Ok(bytes.to_vec())
        })
    }

    fn read_directory(
        &self,
        _path: &Path,
    ) -> Result<Box<dyn Iterator<Item = PathBuf>>, AssetIoError> {
        Ok(Box::new(std::iter::empty::<PathBuf>()))
    }

    fn watch_path_for_changes(
        &self,
        _to_watch: &Path,
        _to_reload: Option<PathBuf>,
    ) -> Result<(), AssetIoError> {
        Ok(())
    }

    fn watch_for_changes(&self, _configuration: &ChangeWatcher) -> Result<(), AssetIoError> {
        Ok(())
    }

    fn get_metadata(&self, path: &Path) -> Result<Metadata, AssetIoError> {
        let full_path = self.root_path.join(path);
        full_path
            .metadata()
            .and_then(Metadata::try_from)
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AssetIoError::NotFound(full_path)
                } else {
                    e.into()
                }
            })
    }
}
