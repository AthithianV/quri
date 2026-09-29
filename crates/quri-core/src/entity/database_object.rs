use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub database_id: Uuid,
    pub name: String,
    pub object_type: String,
    pub definition: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: NaiveDateTime,
}
