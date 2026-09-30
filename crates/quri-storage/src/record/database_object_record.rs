use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct DatebaseObjectRecord {
    pub id: Uuid,
    pub database_id: Uuid,
    pub parent_id: Uuid,
    pub schema_name: Option<String>,
    pub name: String,
    pub qualified_name: Option<String>,
    pub object_type: String,
    pub definition: Option<String>,
    pub metadata: Option<Value>,
    pub created_at: NaiveDateTime,
}
