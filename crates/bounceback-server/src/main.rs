use std::net::SocketAddr;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use bounceback_core::{
    AbletonParser, DiffService, ProjectDiff, ProjectParser, ProjectSnapshot,
    SqliteSnapshotRepository,
};

#[derive(Clone)]
struct AppState {
    parser: Arc<AbletonParser>,
    #[allow(dead_code)]
    repo: Arc<SqliteSnapshotRepository>,
}

#[derive(Debug, Serialize)]
struct HealthResponse {
    status: &'static str,
    name: &'static str,
    version: &'static str,
}

#[derive(Debug, Deserialize)]
struct ParseRequest {
    file_path: String,
}

#[derive(Debug, Deserialize)]
struct CompareRequest {
    path_a: String,
    path_b: String,
}

#[derive(Debug, Serialize)]
struct DemoResponse {
    project_name: String,
    snapshot_a: ProjectSnapshot,
    snapshot_b: ProjectSnapshot,
    diff: ProjectDiff,
}

#[tokio::main]
async fn main() {
    let db_path = PathBuf::from(".bounceback/bounceback.db");
    if let Some(parent) = db_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    let repo = Arc::new(
        SqliteSnapshotRepository::new(&db_path)
            .unwrap_or_else(|_| SqliteSnapshotRepository::new_in_memory().unwrap()),
    );
    let parser = Arc::new(AbletonParser::new());

    let state = AppState { parser, repo };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/api/health", get(health_handler))
        .route("/api/snapshots/parse", post(parse_snapshot_handler))
        .route("/api/diff/compare", post(compare_handler))
        .route("/api/demo", get(demo_handler))
        .layer(cors)
        .with_state(state);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
    println!("🚀 BounceBack Server rodando em http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Falha ao abrir porta 3001");
    axum::serve(listener, app).await.expect("Erro no servidor Axum");
}

async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        name: "BounceBack DAW Version Control Core",
        version: "0.1.0",
    })
}

async fn parse_snapshot_handler(
    State(state): State<AppState>,
    Json(req): Json<ParseRequest>,
) -> Result<Json<ProjectSnapshot>, (StatusCode, String)> {
    let path = PathBuf::from(&req.file_path);
    state
        .parser
        .parse(&path)
        .map(Json)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))
}

async fn compare_handler(
    State(state): State<AppState>,
    Json(req): Json<CompareRequest>,
) -> Result<Json<ProjectDiff>, (StatusCode, String)> {
    let path_a = PathBuf::from(&req.path_a);
    let path_b = PathBuf::from(&req.path_b);

    let snap_a = state
        .parser
        .parse(&path_a)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let snap_b = state
        .parser
        .parse(&path_b)
        .map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;

    let diff = DiffService::calculate(&snap_a, &snap_b);
    Ok(Json(diff))
}

async fn demo_handler(
    State(state): State<AppState>,
) -> Result<Json<DemoResponse>, (StatusCode, String)> {
    // Procura as fixtures no ambiente
    let candidates = [
        "tests/fixtures/ableton",
        "crates/bounceback-core/tests/fixtures/ableton",
    ];

    let mut base_dir = PathBuf::from("tests/fixtures/ableton");
    for cand in candidates {
        if Path::new(cand).join("project_v1.als").exists() {
            base_dir = PathBuf::from(cand);
            break;
        }
    }

    let p1 = base_dir.join("project_v1.als");
    let p2 = base_dir.join("project_v2.als");

    let snap_a = state
        .parser
        .parse(&p1)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro lendo v1: {}", e)))?;

    let snap_b = state
        .parser
        .parse(&p2)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, format!("Erro lendo v2: {}", e)))?;

    let diff = DiffService::calculate(&snap_a, &snap_b);

    Ok(Json(DemoResponse {
        project_name: "Ableton Studio Session".to_string(),
        snapshot_a: snap_a,
        snapshot_b: snap_b,
        diff,
    }))
}
