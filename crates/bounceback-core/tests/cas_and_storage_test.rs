use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

use bounceback_core::{
    AssetBlobStorage, Device, DeviceKind, LocalBlobStore, MixerSettings, Project, ProjectId,
    ProjectSnapshot, SnapshotId, SnapshotRepository, SqliteSnapshotRepository, Track, TrackType,
};

#[test]
fn test_cas_store_and_deduplicate_asset() {
    let temp_dir = tempdir().expect("Falha ao criar tempdir");
    let cas_dir = temp_dir.path().join(".bounceback").join("objects");
    let cas = LocalBlobStore::new(&cas_dir).expect("Falha ao inicializar CAS");

    // Cria arquivo de áudio falso 1
    let audio_path1 = temp_dir.path().join("kick_808.wav");
    {
        let mut file = File::create(&audio_path1).unwrap();
        file.write_all(b"RIFF....WAVEfmt ....data...kick_sample_bytes_12345").unwrap();
    }

    let asset1 = cas.store_file(&audio_path1).expect("Falha ao armazenar asset 1");
    assert_eq!(asset1.file_name, "kick_808.wav");
    assert_eq!(asset1.size_bytes, audio_path1.metadata().unwrap().len());
    assert!(cas.exists(&asset1.hash).unwrap());

    // Cria arquivo com o mesmo conteúdo exato em outro caminho (duplicado)
    let audio_path2 = temp_dir.path().join("kick_copy.wav");
    {
        let mut file = File::create(&audio_path2).unwrap();
        file.write_all(b"RIFF....WAVEfmt ....data...kick_sample_bytes_12345").unwrap();
    }

    let asset2 = cas.store_file(&audio_path2).expect("Falha ao armazenar asset 2");
    // O hash deve ser IDÊNTICO (Content-Addressable Storage)
    assert_eq!(asset1.hash, asset2.hash, "Hashes de conteúdos idênticos devem coincidir");

    // Valida que o caminho do blob físico existe
    let blob_path = cas.get_file_path(&asset1.hash).unwrap().expect("Blob deve existir");
    assert!(blob_path.exists());

    // Cria arquivo com conteúdo diferente
    let snare_path = temp_dir.path().join("snare.wav");
    {
        let mut file = File::create(&snare_path).unwrap();
        file.write_all(b"RIFF....WAVEfmt ....data...snare_sample_different").unwrap();
    }
    let asset3 = cas.store_file(&snare_path).expect("Falha ao armazenar snare");
    assert_ne!(asset1.hash, asset3.hash, "Hashes de conteúdos distintos devem ser diferentes");
}

#[test]
fn test_sqlite_project_and_snapshot_persistence() {
    let repo = SqliteSnapshotRepository::new_in_memory().expect("Falha ao criar repositório SQLite");

    let project_id = ProjectId("proj-ableton-1".to_string());
    let project = Project::new(
        project_id.clone(),
        "Meu Hit de Verão",
        "/caminho/projeto.als",
        1700000000,
    );

    repo.save_project(&project).expect("Falha ao salvar projeto");

    let loaded_proj = repo
        .get_project(&project_id)
        .expect("Query falhou")
        .expect("Projeto deve existir");
    assert_eq!(loaded_proj.name, "Meu Hit de Verão");
    assert_eq!(loaded_proj.file_path, "/caminho/projeto.als");

    // Cria e salva um snapshot
    let snap_id = SnapshotId("snap-commit-123".to_string());
    let track = Track::new(
        "track-1",
        "Bass Track",
        TrackType::Audio,
        MixerSettings {
            volume_db: -1.5,
            pan: 0.1,
        },
        vec![Device::new("dev-1", "FabFilter Saturn 2", DeviceKind::Vst3)],
    );

    let snapshot = ProjectSnapshot::new(
        snap_id.clone(),
        "Ableton Live",
        "11.3",
        128.0,
        vec![track],
        vec![],
    );

    repo.save_snapshot(&project_id, &snapshot).expect("Falha ao salvar snapshot");

    let loaded_snap = repo
        .get_snapshot(&snap_id)
        .expect("Query de snapshot falhou")
        .expect("Snapshot deve existir");
    assert_eq!(loaded_snap.daw_name, "Ableton Live");
    assert_eq!(loaded_snap.tempo_bpm, 128.0);
    assert_eq!(loaded_snap.tracks.len(), 1);
    assert_eq!(loaded_snap.tracks[0].name, "Bass Track");
    assert_eq!(loaded_snap.tracks[0].mixer.volume_db, -1.5);
    assert!(loaded_snap.tracks[0].has_device("FabFilter Saturn 2"));

    // Listagem
    let list = repo.list_snapshots(&project_id).expect("Listagem falhou");
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].id, snap_id);
}
