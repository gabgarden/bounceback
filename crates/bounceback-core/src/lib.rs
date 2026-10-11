pub mod application;
pub mod domain;
pub mod infrastructure;

// Re-exports convenientes
pub use application::{CreateSnapshotUseCase, DiffService};
pub use domain::errors::DomainError;
pub use domain::models::{
    AssetHash, AudioAsset, Device, DeviceKind, DeviceDiff, MixerDiff, MixerSettings, Project, ProjectDiff,
    ProjectId, ProjectSnapshot, SnapshotId, Track, TrackDiff, TrackType,
};
pub use domain::ports::{AssetBlobStorage, ProjectParser, SnapshotRepository};
pub use infrastructure::{
    AbletonGzipReader, AbletonParser, LocalBlobStore, SqliteSnapshotRepository,
};
