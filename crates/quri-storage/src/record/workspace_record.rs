use chrono::NaiveDateTime;
use quri_core::model::workspace_model::WorkspaceModel;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct WorkspaceRecord {
    pub id: Uuid,
    pub name: String,
    pub is_opened: Option<bool>,
    pub is_active: Option<bool>,
    pub last_opened: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl From<WorkspaceRecord> for WorkspaceModel {
    fn from(record: WorkspaceRecord) -> Self {
        Self {
            id: record.id,
            name: record.name,
            is_opened: record.is_opened,
            is_active: record.is_active,
            last_opened: record.last_opened,
        }
    }
}
