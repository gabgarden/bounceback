use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AssetHash(pub String);

impl AssetHash {
    pub fn new(hash: impl Into<String>) -> Self {
        Self(hash.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AudioAsset {
    pub hash: AssetHash,
    pub original_path: String,
    pub file_name: String,
    pub size_bytes: u64,
    pub sample_rate: Option<u32>,
}

impl AudioAsset {
    pub fn new(
        hash: AssetHash,
        original_path: impl Into<String>,
        file_name: impl Into<String>,
        size_bytes: u64,
        sample_rate: Option<u32>,
    ) -> Self {
        Self {
            hash,
            original_path: original_path.into(),
            file_name: file_name.into(),
            size_bytes,
            sample_rate,
        }
    }
}
