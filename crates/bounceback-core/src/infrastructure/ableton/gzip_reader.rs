use std::fs::File;
use std::io::Read;
use std::path::Path;
use flate2::read::GzDecoder;
use crate::domain::errors::DomainError;

pub struct AbletonGzipReader;

impl AbletonGzipReader {
    pub fn read_to_string(file_path: &Path) -> Result<String, DomainError> {
        let file = File::open(file_path)
            .map_err(|e| DomainError::IoError(format!("Erro ao abrir arquivo {:?}: {}", file_path, e)))?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = String::new();
        decoder
            .read_to_string(&mut buffer)
            .map_err(|e| DomainError::DecompressionFailed(format!("Falha ao descompactar Gzip: {}", e)))?;
        Ok(buffer)
    }

    pub fn read_bytes(file_path: &Path) -> Result<Vec<u8>, DomainError> {
        let file = File::open(file_path)
            .map_err(|e| DomainError::IoError(format!("Erro ao abrir arquivo {:?}: {}", file_path, e)))?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = Vec::new();
        decoder
            .read_to_end(&mut buffer)
            .map_err(|e| DomainError::DecompressionFailed(format!("Falha ao descompactar Gzip: {}", e)))?;
        Ok(buffer)
    }
}
