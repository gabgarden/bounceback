use serde::{Deserialize, Serialize};
use super::device::Device;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TrackType {
    Audio,
    Midi,
    Return,
    Master,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MixerSettings {
    pub volume_db: f64,
    pub pan: f64,
}

impl Default for MixerSettings {
    fn default() -> Self {
        Self {
            volume_db: 0.0,
            pan: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Track {
    pub id: String,
    pub name: String,
    pub track_type: TrackType,
    pub mixer: MixerSettings,
    pub devices: Vec<Device>,
    pub is_muted: bool,
    pub is_soloed: bool,
}

impl Track {
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        track_type: TrackType,
        mixer: MixerSettings,
        devices: Vec<Device>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            track_type,
            mixer,
            devices,
            is_muted: false,
            is_soloed: false,
        }
    }

    pub fn has_device(&self, device_name: &str) -> bool {
        self.devices.iter().any(|d| d.name.eq_ignore_ascii_case(device_name))
    }
}
