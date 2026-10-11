use std::path::Path;
use std::sync::Mutex;
use rusqlite::{params, Connection};

use crate::domain::errors::DomainError;
use crate::domain::models::{Project, ProjectId, ProjectSnapshot, SnapshotId};
use crate::domain::ports::SnapshotRepository;

pub struct SqliteSnapshotRepository {
    conn: Mutex<Connection>,
}

impl SqliteSnapshotRepository {
    pub fn new(db_path: &Path) -> Result<Self, DomainError> {
        let conn = Connection::open(db_path)
            .map_err(|e| DomainError::PersistenceError(format!("Erro abrindo SQLite {:?}: {}", db_path, e)))?;
        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.init_schema()?;
        Ok(repo)
    }

    pub fn new_in_memory() -> Result<Self, DomainError> {
        let conn = Connection::open_in_memory()
            .map_err(|e| DomainError::PersistenceError(format!("Erro abrindo SQLite em memória: {}", e)))?;
        let repo = Self {
            conn: Mutex::new(conn),
        };
        repo.init_schema()?;
        Ok(repo)
    }

    fn init_schema(&self) -> Result<(), DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );

            CREATE TABLE IF NOT EXISTS snapshots (
                id TEXT PRIMARY KEY,
                project_id TEXT NOT NULL,
                daw_name TEXT NOT NULL,
                daw_version TEXT NOT NULL,
                tempo_bpm REAL NOT NULL,
                snapshot_data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                FOREIGN KEY(project_id) REFERENCES projects(id)
            );

            CREATE INDEX IF NOT EXISTS idx_snapshots_project_id ON snapshots(project_id);
            ",
        )
        .map_err(|e| DomainError::PersistenceError(format!("Erro criando tabelas no SQLite: {}", e)))?;

        Ok(())
    }
}

impl SnapshotRepository for SqliteSnapshotRepository {
    fn save_project(&self, project: &Project) -> Result<(), DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        conn.execute(
            "INSERT OR REPLACE INTO projects (id, name, file_path, created_at, updated_at)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                project.id.0,
                project.name,
                project.file_path,
                project.created_at,
                project.updated_at
            ],
        )
        .map_err(|e| DomainError::PersistenceError(format!("Erro ao salvar projeto: {}", e)))?;

        Ok(())
    }

    fn get_project(&self, id: &ProjectId) -> Result<Option<Project>, DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        let mut stmt = conn
            .prepare("SELECT id, name, file_path, created_at, updated_at FROM projects WHERE id = ?1")
            .map_err(|e| DomainError::PersistenceError(format!("Erro preparando query de projeto: {}", e)))?;

        let mut rows = stmt
            .query(params![id.0])
            .map_err(|e| DomainError::PersistenceError(format!("Erro buscando projeto: {}", e)))?;

        if let Some(row) = rows.next().map_err(|e| DomainError::PersistenceError(e.to_string()))? {
            let pid: String = row.get(0).map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let name: String = row.get(1).map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let file_path: String = row.get(2).map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let created_at: i64 = row.get(3).map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let updated_at: i64 = row.get(4).map_err(|e| DomainError::PersistenceError(e.to_string()))?;

            let mut proj = Project::new(ProjectId(pid), name, file_path, created_at);
            proj.updated_at = updated_at;
            Ok(Some(proj))
        } else {
            Ok(None)
        }
    }

    fn save_snapshot(&self, project_id: &ProjectId, snapshot: &ProjectSnapshot) -> Result<(), DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        let snapshot_json = serde_json::to_string(snapshot)
            .map_err(|e| DomainError::PersistenceError(format!("Falha ao serializar snapshot: {}", e)))?;

        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| DomainError::PersistenceError(e.to_string()))?
            .as_secs() as i64;

        conn.execute(
            "INSERT OR REPLACE INTO snapshots (id, project_id, daw_name, daw_version, tempo_bpm, snapshot_data, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![
                snapshot.id.0,
                project_id.0,
                snapshot.daw_name,
                snapshot.daw_version,
                snapshot.tempo_bpm,
                snapshot_json,
                now
            ],
        )
        .map_err(|e| DomainError::PersistenceError(format!("Erro ao persistir snapshot: {}", e)))?;

        Ok(())
    }

    fn get_snapshot(&self, snapshot_id: &SnapshotId) -> Result<Option<ProjectSnapshot>, DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        let mut stmt = conn
            .prepare("SELECT snapshot_data FROM snapshots WHERE id = ?1")
            .map_err(|e| DomainError::PersistenceError(format!("Erro preparando query: {}", e)))?;

        let mut rows = stmt
            .query(params![snapshot_id.0])
            .map_err(|e| DomainError::PersistenceError(format!("Erro executando busca: {}", e)))?;

        if let Some(row) = rows.next().map_err(|e| DomainError::PersistenceError(e.to_string()))? {
            let json_str: String = row.get(0).map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let snapshot: ProjectSnapshot = serde_json::from_str(&json_str)
                .map_err(|e| DomainError::PersistenceError(format!("Falha ao desserializar snapshot: {}", e)))?;
            Ok(Some(snapshot))
        } else {
            Ok(None)
        }
    }

    fn list_snapshots(&self, project_id: &ProjectId) -> Result<Vec<ProjectSnapshot>, DomainError> {
        let conn = self.conn.lock().map_err(|_| {
            DomainError::PersistenceError("Falha ao adquirir lock do banco de dados".to_string())
        })?;

        let mut stmt = conn
            .prepare("SELECT snapshot_data FROM snapshots WHERE project_id = ?1 ORDER BY created_at DESC")
            .map_err(|e| DomainError::PersistenceError(format!("Erro preparando query de listagem: {}", e)))?;

        let rows = stmt
            .query_map(params![project_id.0], |row| {
                let json_str: String = row.get(0)?;
                Ok(json_str)
            })
            .map_err(|e| DomainError::PersistenceError(format!("Erro ao listar snapshots: {}", e)))?;

        let mut snapshots = Vec::new();
        for json_res in rows {
            let json_str = json_res.map_err(|e| DomainError::PersistenceError(e.to_string()))?;
            let snapshot: ProjectSnapshot = serde_json::from_str(&json_str)
                .map_err(|e| DomainError::PersistenceError(format!("Falha desserializando snapshot: {}", e)))?;
            snapshots.push(snapshot);
        }

        Ok(snapshots)
    }
}
