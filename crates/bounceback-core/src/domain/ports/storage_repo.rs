use crate::domain::errors::DomainError;
use crate::domain::models::{Project, ProjectId, ProjectSnapshot, SnapshotId};

pub trait SnapshotRepository: Send + Sync {
    fn save_project(&self, project: &Project) -> Result<(), DomainError>;
    fn get_project(&self, id: &ProjectId) -> Result<Option<Project>, DomainError>;
    fn save_snapshot(&self, project_id: &ProjectId, snapshot: &ProjectSnapshot) -> Result<(), DomainError>;
    fn get_snapshot(&self, snapshot_id: &SnapshotId) -> Result<Option<ProjectSnapshot>, DomainError>;
    fn list_snapshots(&self, project_id: &ProjectId) -> Result<Vec<ProjectSnapshot>, DomainError>;
}
