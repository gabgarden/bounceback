use std::path::Path;
use std::sync::Arc;

use crate::domain::errors::DomainError;
use crate::domain::models::{ProjectId, ProjectSnapshot};
use crate::domain::ports::{ProjectParser, SnapshotRepository};

pub struct CreateSnapshotUseCase {
    parser: Arc<dyn ProjectParser>,
    repository: Arc<dyn SnapshotRepository>,
}

impl CreateSnapshotUseCase {
    pub fn new(
        parser: Arc<dyn ProjectParser>,
        repository: Arc<dyn SnapshotRepository>,
    ) -> Self {
        Self { parser, repository }
    }

    pub fn execute(
        &self,
        project_id: &ProjectId,
        file_path: &Path,
    ) -> Result<ProjectSnapshot, DomainError> {
        // 1. Parsing semântico via trait ProjectParser (DIP / OCP)
        let snapshot = self.parser.parse(file_path)?;

        // 2. Persistência no repositório de snapshots
        self.repository.save_snapshot(project_id, &snapshot)?;

        Ok(snapshot)
    }
}
