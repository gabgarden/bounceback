use serde::{Deserialize, Serialize};
use super::device::Device;
use super::track::Track;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum DeviceDiff {
    Added(Device),
    Removed(Device),
    Modified {
        device_id: String,
        name: String,
        change: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixerDiff {
    pub volume_before: f64,
    pub volume_after: f64,
    pub pan_before: f64,
    pub pan_after: f64,
}

impl MixerDiff {
    pub fn has_changes(&self) -> bool {
        (self.volume_before - self.volume_after).abs() > 0.0001
            || (self.pan_before - self.pan_after).abs() > 0.0001
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrackDiff {
    pub track_id: String,
    pub track_name: String,
    pub mixer_diff: Option<MixerDiff>,
    pub device_diffs: Vec<DeviceDiff>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectDiff {
    pub added_tracks: Vec<Track>,
    pub removed_tracks: Vec<Track>,
    pub modified_tracks: Vec<TrackDiff>,
    pub tempo_change: Option<(f64, f64)>,
}

impl ProjectDiff {
    pub fn new() -> Self {
        Self {
            added_tracks: Vec::new(),
            removed_tracks: Vec::new(),
            modified_tracks: Vec::new(),
            tempo_change: None,
        }
    }

    pub fn has_device_added(&self, device_name: &str) -> bool {
        self.modified_tracks.iter().any(|t| {
            t.device_diffs.iter().any(|d| match d {
                DeviceDiff::Added(dev) => dev.name.eq_ignore_ascii_case(device_name),
                _ => false,
            })
        })
    }

    pub fn has_device_removed(&self, device_name: &str) -> bool {
        self.modified_tracks.iter().any(|t| {
            t.device_diffs.iter().any(|d| match d {
                DeviceDiff::Removed(dev) => dev.name.eq_ignore_ascii_case(device_name),
                _ => false,
            })
        })
    }

    pub fn is_empty(&self) -> bool {
        self.added_tracks.is_empty()
            && self.removed_tracks.is_empty()
            && self.modified_tracks.is_empty()
            && self.tempo_change.is_none()
    }
}

impl Default for ProjectDiff {
    fn default() -> Self {
        Self::new()
    }
}
