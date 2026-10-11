use std::path::{Path, PathBuf};
use crate::domain::errors::DomainError;
use crate::domain::models::audio_asset::{AssetHash, AudioAsset};

pub trait AssetBlobStorage: Send + Sync {
    fn store_file(&self, source_path: &Path) -> Result<AudioAsset, DomainError>;
    fn exists(&self, hash: &AssetHash) -> Result<bool, DomainError>;
    fn get_file_path(&self, hash: &AssetHash) -> Result<Option<PathBuf>, DomainError>;
}
