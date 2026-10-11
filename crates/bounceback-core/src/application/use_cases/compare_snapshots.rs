use crate::domain::models::{
    DeviceDiff, MixerDiff, ProjectDiff, ProjectSnapshot, Track, TrackDiff,
};

pub struct DiffService;

impl DiffService {
    pub fn calculate(old: &ProjectSnapshot, new: &ProjectSnapshot) -> ProjectDiff {
        let mut diff = ProjectDiff::new();

        // 1. Detectar mudanças de tempo/BPM
        if (old.tempo_bpm - new.tempo_bpm).abs() > 0.001 {
            diff.tempo_change = Some((old.tempo_bpm, new.tempo_bpm));
        }

        // 2. Comparar Tracks
        for new_track in &new.tracks {
            if let Some(old_track) = old.find_track_by_id(&new_track.id) {
                // Track existe em ambos -> verificar modificações
                if let Some(track_diff) = Self::diff_track(old_track, new_track) {
                    diff.modified_tracks.push(track_diff);
                }
            } else {
                // Track adicionada
                diff.added_tracks.push(new_track.clone());
            }
        }

        // 3. Detectar Tracks Removidas
        for old_track in &old.tracks {
            if new.find_track_by_id(&old_track.id).is_none() {
                diff.removed_tracks.push(old_track.clone());
            }
        }

        diff
    }

    fn diff_track(old: &Track, new: &Track) -> Option<TrackDiff> {
        let mixer_diff = Self::diff_mixer(old, new);
        let device_diffs = Self::diff_devices(old, new);

        if mixer_diff.is_some() || !device_diffs.is_empty() {
            Some(TrackDiff {
                track_id: new.id.clone(),
                track_name: new.name.clone(),
                mixer_diff,
                device_diffs,
            })
        } else {
            None
        }
    }

    fn diff_mixer(old: &Track, new: &Track) -> Option<MixerDiff> {
        let m = MixerDiff {
            volume_before: old.mixer.volume_db,
            volume_after: new.mixer.volume_db,
            pan_before: old.mixer.pan,
            pan_after: new.mixer.pan,
        };
        if m.has_changes() {
            Some(m)
        } else {
            None
        }
    }

    fn diff_devices(old: &Track, new: &Track) -> Vec<DeviceDiff> {
        let mut diffs = Vec::new();

        // Dispositivos adicionados no novo track
        for new_dev in &new.devices {
            if !old.has_device(&new_dev.name) {
                diffs.push(DeviceDiff::Added(new_dev.clone()));
            }
        }

        // Dispositivos removidos
        for old_dev in &old.devices {
            if !new.has_device(&old_dev.name) {
                diffs.push(DeviceDiff::Removed(old_dev.clone()));
            }
        }

        diffs
    }
}
