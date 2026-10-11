export type DeviceKind = 'Vst3' | 'Vst2' | 'Au' | 'Native' | 'Unknown';

export interface Device {
  id: string;
  name: string;
  kind: DeviceKind;
  is_enabled: boolean;
}

export type TrackType = 'Audio' | 'Midi' | 'Return' | 'Master';

export interface MixerSettings {
  volume_db: number;
  pan: number;
}

export interface Track {
  id: string;
  name: string;
  track_type: TrackType;
  mixer: MixerSettings;
  devices: Device[];
  is_muted: boolean;
  is_soloed: boolean;
}

export interface AudioAsset {
  hash: { 0: string };
  original_path: string;
  file_name: string;
  size_bytes: number;
  sample_rate?: number;
}

export interface ProjectSnapshot {
  id: { 0: string };
  daw_name: string;
  daw_version: string;
  tempo_bpm: number;
  tracks: Track[];
  assets: AudioAsset[];
}

export interface MixerDiff {
  volume_before: number;
  volume_after: number;
  pan_before: number;
  pan_after: number;
}

export type DeviceDiff =
  | { Added: Device }
  | { Removed: Device }
  | { Modified: { device_id: string; name: string; change: string } };

export interface TrackDiff {
  track_id: string;
  track_name: string;
  mixer_diff?: MixerDiff;
  device_diffs: DeviceDiff[];
}

export interface ProjectDiff {
  added_tracks: Track[];
  removed_tracks: Track[];
  modified_tracks: TrackDiff[];
  tempo_change?: [number, number];
}

export interface DemoResponse {
  project_name: string;
  snapshot_a: ProjectSnapshot;
  snapshot_b: ProjectSnapshot;
  diff: ProjectDiff;
}
