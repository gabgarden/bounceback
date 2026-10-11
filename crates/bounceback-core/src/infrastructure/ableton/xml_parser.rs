use std::path::Path;
use quick_xml::events::Event;
use quick_xml::reader::Reader;

use crate::domain::errors::DomainError;
use crate::domain::models::{
    Device, DeviceKind, MixerSettings, ProjectSnapshot, SnapshotId, Track, TrackType,
};
use crate::domain::ports::ProjectParser;
use super::gzip_reader::AbletonGzipReader;

pub struct AbletonParser;

impl AbletonParser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse_xml_str(xml_str: &str) -> Result<ProjectSnapshot, DomainError> {
        let mut reader = Reader::from_str(xml_str);
        reader.config_mut().trim_text(true);

        let mut creator = String::from("Ableton Live");
        let mut version = String::from("11.0");
        let mut tempo_bpm = 120.0;
        let mut tracks: Vec<Track> = Vec::new();

        // Estado transiente durante o streaming do XML
        let mut current_track_type: Option<TrackType> = None;
        let mut current_track_id = String::new();
        let mut current_track_name = String::new();
        let mut current_mixer = MixerSettings::default();
        let mut current_devices: Vec<Device> = Vec::new();

        let mut in_name_tag = false;
        let mut in_mixer_volume = false;
        let mut in_mixer_pan = false;
        let mut in_tempo = false;
        let mut current_device_id: Option<String> = None;
        let mut current_device_name: Option<String> = None;
        let mut current_device_kind = DeviceKind::Unknown;

        let mut buf = Vec::new();

        loop {
            match reader.read_event_into(&mut buf) {
                Ok(Event::Start(ref e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref());

                    match name_str.as_ref() {
                        "Ableton" => {
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"Creator" {
                                    creator = String::from_utf8_lossy(&attr.value).to_string();
                                } else if attr.key.as_ref() == b"MinorVersion" {
                                    version = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        "AudioTrack" | "MidiTrack" | "ReturnTrack" => {
                            current_track_type = Some(match name_str.as_ref() {
                                "MidiTrack" => TrackType::Midi,
                                "ReturnTrack" => TrackType::Return,
                                _ => TrackType::Audio,
                            });
                            current_track_name = String::from("Track");
                            current_mixer = MixerSettings::default();
                            current_devices.clear();

                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"Id" {
                                    current_track_id = String::from_utf8_lossy(&attr.value).to_string();
                                }
                            }
                        }
                        "Name" if current_track_type.is_some() && current_device_id.is_none() => {
                            in_name_tag = true;
                        }
                        "Volume" if current_track_type.is_some() => {
                            in_mixer_volume = true;
                        }
                        "Pan" if current_track_type.is_some() => {
                            in_mixer_pan = true;
                        }
                        "Tempo" => {
                            in_tempo = true;
                        }
                        "PluginDevice" => {
                            current_device_kind = DeviceKind::Vst3;
                            for attr in e.attributes().flatten() {
                                if attr.key.as_ref() == b"Id" {
                                    current_device_id = Some(String::from_utf8_lossy(&attr.value).to_string());
                                }
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Empty(ref e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref());

                    if (name_str == "EffectiveName" || name_str == "UserName") && in_name_tag {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"Value" {
                                current_track_name = String::from_utf8_lossy(&attr.value).to_string();
                            }
                        }
                    } else if name_str == "Manual" && in_mixer_volume {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"Value" {
                                if let Ok(val) = String::from_utf8_lossy(&attr.value).parse::<f64>() {
                                    current_mixer.volume_db = val;
                                }
                            }
                        }
                    } else if name_str == "Manual" && in_mixer_pan {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"Value" {
                                if let Ok(val) = String::from_utf8_lossy(&attr.value).parse::<f64>() {
                                    current_mixer.pan = val;
                                }
                            }
                        }
                    } else if name_str == "Manual" && in_tempo {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"Value" {
                                if let Ok(val) = String::from_utf8_lossy(&attr.value).parse::<f64>() {
                                    tempo_bpm = val;
                                }
                            }
                        }
                    } else if name_str == "Name" && current_device_id.is_some() {
                        for attr in e.attributes().flatten() {
                            if attr.key.as_ref() == b"Value" {
                                current_device_name = Some(String::from_utf8_lossy(&attr.value).to_string());
                            }
                        }
                    }
                }
                Ok(Event::End(ref e)) => {
                    let name = e.name();
                    let name_str = String::from_utf8_lossy(name.as_ref());

                    match name_str.as_ref() {
                        "Name" => in_name_tag = false,
                        "Volume" => in_mixer_volume = false,
                        "Pan" => in_mixer_pan = false,
                        "Tempo" => in_tempo = false,
                        "PluginDevice" => {
                            if let (Some(dev_id), Some(dev_name)) = (current_device_id.take(), current_device_name.take()) {
                                current_devices.push(Device::new(dev_id, dev_name, current_device_kind.clone()));
                            }
                        }
                        "AudioTrack" | "MidiTrack" | "ReturnTrack" => {
                            if let Some(track_type) = current_track_type.take() {
                                tracks.push(Track::new(
                                    current_track_id.clone(),
                                    current_track_name.clone(),
                                    track_type,
                                    current_mixer.clone(),
                                    current_devices.clone(),
                                ));
                            }
                        }
                        _ => {}
                    }
                }
                Ok(Event::Eof) => break,
                Err(e) => return Err(DomainError::InvalidFormat(format!("Erro ao fazer parse do XML: {}", e))),
                _ => {}
            }
            buf.clear();
        }

        let snapshot_id = SnapshotId(blake3::hash(xml_str.as_bytes()).to_hex().to_string());
        Ok(ProjectSnapshot::new(
            snapshot_id,
            creator,
            version,
            tempo_bpm,
            tracks,
            Vec::new(),
        ))
    }
}

impl Default for AbletonParser {
    fn default() -> Self {
        Self::new()
    }
}

impl ProjectParser for AbletonParser {
    fn parse(&self, file_path: &Path) -> Result<ProjectSnapshot, DomainError> {
        let xml_str = AbletonGzipReader::read_to_string(file_path)?;
        Self::parse_xml_str(&xml_str)
    }

    fn supported_extensions(&self) -> &'static [&'static str] {
        &["als"]
    }
}
