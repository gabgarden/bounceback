use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Falha ao descompactar arquivo: {0}")]
    DecompressionFailed(String),

    #[error("Formato de arquivo ou XML inválido: {0}")]
    InvalidFormat(String),

    #[error("Erro de I/O no disco: {0}")]
    IoError(String),

    #[error("Erro de persistência de dados: {0}")]
    PersistenceError(String),

    #[error("Erro no cálculo de diff: {0}")]
    DiffError(String),
}
