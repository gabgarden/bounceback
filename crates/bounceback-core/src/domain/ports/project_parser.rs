use std::path::Path;
use crate::domain::errors::DomainError;
use crate::domain::models::ProjectSnapshot;

pub trait ProjectParser: Send + Sync {
    fn parse(&self, file_path: &Path) -> Result<ProjectSnapshot, DomainError>;
    fn supported_extensions(&self) -> &'static [&'static str];
}
