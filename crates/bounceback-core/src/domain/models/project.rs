use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ProjectId(pub String);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub file_path: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Project {
    pub fn new(
        id: ProjectId,
        name: impl Into<String>,
        file_path: impl Into<String>,
        created_at: i64,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            file_path: file_path.into(),
            created_at,
            updated_at: created_at,
        }
    }
}
