use std::path::PathBuf;
use std::sync::Arc;
use bounceback_core::{
    AbletonParser, CreateSnapshotUseCase, DiffService, ProjectId, ProjectParser,
    SnapshotRepository, SqliteSnapshotRepository,
};

fn get_fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push("ableton");
    path.push(name);
    if !path.exists() {
        // Fallback para execução a partir da raiz do workspace
        let mut root_path = PathBuf::from("tests/fixtures/ableton");
        root_path.push(name);
        return root_path;
    }
    path
}

#[test]
fn test_detect_track_gain_and_plugin_change() {
    let parser = AbletonParser::new();
    let snap_a_path = get_fixture_path("minimal_v11.als");
    let snap_b_path = get_fixture_path("minimal_v11_modified.als");

    let snap_a = parser.parse(&snap_a_path).expect("Falha ao ler snapshot A");
    let snap_b = parser.parse(&snap_b_path).expect("Falha ao ler snapshot B");

    let diff = DiffService::calculate(&snap_a, &snap_b);

    assert_eq!(diff.modified_tracks.len(), 1, "Deveria haver exatamente 1 track modificada");
    let mod_track = &diff.modified_tracks[0];
    assert_eq!(mod_track.track_name, "Audio 1");

    // Valida alteração do mixer (0.0 dB -> 3.0 dB)
    let mixer_diff = mod_track.mixer_diff.as_ref().expect("Deveria conter alteração de mixer");
    assert!((mixer_diff.volume_before - 0.0).abs() < 0.001);
    assert!((mixer_diff.volume_after - 3.0).abs() < 0.001);

    // Valida substituição de plugins
    assert!(diff.has_device_added("FabFilter Pro-C 2"), "Deveria detectar adição do FabFilter Pro-C 2");
    assert!(diff.has_device_removed("FabFilter Pro-Q 3"), "Deveria detectar remoção do FabFilter Pro-Q 3");
}

#[test]
fn test_identical_projects_produce_empty_diff() {
    let parser = AbletonParser::new();
    let snap_path = get_fixture_path("minimal_v11.als");

    let snap_a = parser.parse(&snap_path).expect("Falha ao ler snapshot");
    let snap_b = parser.parse(&snap_path).expect("Falha ao ler snapshot novamente");

    let diff = DiffService::calculate(&snap_a, &snap_b);

    assert!(diff.is_empty(), "Diff entre snapshots idênticos deve ser vazio");
}

#[test]
fn test_end_to_end_snapshot_creation_and_diff() {
    let repo = Arc::new(SqliteSnapshotRepository::new_in_memory().expect("Falha ao criar SQLite"));
    let parser = Arc::new(AbletonParser::new());

    let use_case = CreateSnapshotUseCase::new(parser, repo.clone());
    let project_id = ProjectId("proj-ableton-v11".to_string());
    let path_v1 = get_fixture_path("project_v1.als");
    let path_v2 = get_fixture_path("project_v2.als");

    // Salva o projeto pai no repositório
    let project = bounceback_core::Project::new(
        project_id.clone(),
        "Meu Projeto Ableton",
        path_v1.to_string_lossy(),
        1700000000,
    );
    repo.save_project(&project).expect("Falha salvando projeto pai");

    // Cria e persiste snapshot 1
    let snap1 = use_case.execute(&project_id, &path_v1).expect("Falha criando snapshot 1");
    // Cria e persiste snapshot 2
    let snap2 = use_case.execute(&project_id, &path_v2).expect("Falha criando snapshot 2");

    // Valida persistência no repositório
    let stored_list = repo.list_snapshots(&project_id).expect("Falha listando snapshots");
    assert_eq!(stored_list.len(), 2, "Deveriam existir 2 snapshots no histórico");

    // Calcula diff entre os snapshots persistidos
    let diff = DiffService::calculate(&snap1, &snap2);
    assert_eq!(diff.modified_tracks.len(), 1);
    assert!(diff.has_device_added("FabFilter Pro-C 2"));
    assert!(diff.has_device_removed("FabFilter Pro-Q 3"));
}
