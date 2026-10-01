use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConnectionModel {
    pub id: Uuid,
    pub workspace_id: Uuid,

    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
    pub last_introspect_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CreateConnectionModel {
    pub workspace_id: Uuid,
    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct UpdateConnectionModel {
    pub connection_name: Option<Option<String>>,
    pub connection_config: Option<Value>,
    pub last_connected_at: Option<Option<NaiveDateTime>>,
}
