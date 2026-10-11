use std::fs::{self, File};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use crate::domain::errors::DomainError;
use crate::domain::models::audio_asset::{AssetHash, AudioAsset};
use crate::domain::ports::AssetBlobStorage;

/// Content-Addressable Storage (CAS) local para assets de áudio.
/// Organiza blobs usando o hash BLAKE3 em shards (estilo Git: ab/cdef1234...).
#[derive(Debug, Clone)]
pub struct LocalBlobStore {
    root_dir: PathBuf,
}

impl LocalBlobStore {
    pub fn new(root_dir: impl Into<PathBuf>) -> Result<Self, DomainError> {
        let root = root_dir.into();
        fs::create_dir_all(&root)
            .map_err(|e| DomainError::IoError(format!("Falha ao criar diretório CAS {:?}: {}", root, e)))?;
        Ok(Self { root_dir: root })
    }

    fn blob_path_for_hash(&self, hash_str: &str) -> PathBuf {
        if hash_str.len() >= 4 {
            let (prefix, suffix) = hash_str.split_at(2);
            self.root_dir.join(prefix).join(suffix)
        } else {
            self.root_dir.join(hash_str)
        }
    }

    fn compute_hash_and_size(source_path: &Path) -> Result<(String, u64), DomainError> {
        let file = File::open(source_path)
            .map_err(|e| DomainError::IoError(format!("Erro ao abrir arquivo {:?}: {}", source_path, e)))?;
        let mut reader = BufReader::with_capacity(64 * 1024, file);
        let mut hasher = blake3::Hasher::new();
        let mut buffer = [0u8; 64 * 1024];
        let mut total_bytes = 0u64;

        loop {
            let bytes_read = reader
                .read(&mut buffer)
                .map_err(|e| DomainError::IoError(format!("Erro lendo {:?}: {}", source_path, e)))?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
            total_bytes += bytes_read as u64;
        }

        Ok((hasher.finalize().to_hex().to_string(), total_bytes))
    }
}

impl AssetBlobStorage for LocalBlobStore {
    fn store_file(&self, source_path: &Path) -> Result<AudioAsset, DomainError> {
        if !source_path.exists() {
            return Err(DomainError::IoError(format!("Arquivo fonte não existe: {:?}", source_path)));
        }

        let (hash_hex, size_bytes) = Self::compute_hash_and_size(source_path)?;
        let destination = self.blob_path_for_hash(&hash_hex);

        // Deduplicação: se o blob já existe com este hash, não duplicamos bytes em disco
        if !destination.exists() {
            if let Some(parent) = destination.parent() {
                fs::create_dir_all(parent).map_err(|e| {
                    DomainError::IoError(format!("Falha ao criar diretório shard {:?}: {}", parent, e))
                })?;
            }

            fs::copy(source_path, &destination).map_err(|e| {
                DomainError::IoError(format!(
                    "Falha ao copiar {:?} para CAS {:?}: {}",
                    source_path, destination, e
                ))
            })?;
        }

        let file_name = source_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unnamed_audio")
            .to_string();

        let asset_hash = AssetHash::new(hash_hex);
        Ok(AudioAsset::new(
            asset_hash,
            source_path.to_string_lossy(),
            file_name,
            size_bytes,
            None,
        ))
    }

    fn exists(&self, hash: &AssetHash) -> Result<bool, DomainError> {
        let path = self.blob_path_for_hash(hash.as_str());
        Ok(path.exists())
    }

    fn get_file_path(&self, hash: &AssetHash) -> Result<Option<PathBuf>, DomainError> {
        let path = self.blob_path_for_hash(hash.as_str());
        if path.exists() {
            Ok(Some(path))
        } else {
            Ok(None)
        }
    }
}
