use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct ConnectionRecord {
    pub id: Uuid,
    pub workspace_id: Uuid,

    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
    pub last_introspect_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CreateConnection {
    pub workspace_id: Uuid,
    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct UpdateConnection {
    pub connection_name: Option<Option<String>>,
    pub connection_config: Option<Value>,
    pub last_connected_at: Option<Option<NaiveDateTime>>,
}
