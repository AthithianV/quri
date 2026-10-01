use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkspaceModel {
    pub id: Uuid,
    pub name: String,
    pub is_opened: Option<bool>,
    pub is_active: Option<bool>,
    pub last_opened: Option<NaiveDateTime>,
}
