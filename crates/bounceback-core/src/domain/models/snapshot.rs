use serde::{Deserialize, Serialize};
use super::track::Track;
use super::audio_asset::AudioAsset;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SnapshotId(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectSnapshot {
    pub id: SnapshotId,
    pub daw_name: String,
    pub daw_version: String,
    pub tempo_bpm: f64,
    pub tracks: Vec<Track>,
    pub assets: Vec<AudioAsset>,
}

impl ProjectSnapshot {
    pub fn new(
        id: SnapshotId,
        daw_name: impl Into<String>,
        daw_version: impl Into<String>,
        tempo_bpm: f64,
        tracks: Vec<Track>,
        assets: Vec<AudioAsset>,
    ) -> Self {
        Self {
            id,
            daw_name: daw_name.into(),
            daw_version: daw_version.into(),
            tempo_bpm,
            tracks,
            assets,
        }
    }

    pub fn find_track_by_id(&self, id: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.id == id)
    }

    pub fn find_track_by_name(&self, name: &str) -> Option<&Track> {
        self.tracks.iter().find(|t| t.name.eq_ignore_ascii_case(name))
    }
}
