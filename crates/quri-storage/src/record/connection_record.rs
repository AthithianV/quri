use chrono::NaiveDateTime;
use quri_core::model::connection_model::{
    ConnectionModel, CreateConnectionModel, UpdateConnectionModel,
};
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
pub struct CreateConnectionRecord {
    pub workspace_id: Uuid,
    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct UpdateConnectionRecord {
    pub connection_name: Option<Option<String>>,
    pub connection_config: Option<Value>,
    pub last_connected_at: Option<Option<NaiveDateTime>>,
}

impl From<ConnectionRecord> for ConnectionModel {
    fn from(record: ConnectionRecord) -> Self {
        Self {
            id: record.id,
            workspace_id: record.workspace_id,
            connection_name: record.connection_name,
            connection_config: record.connection_config,
            last_connected_at: record.last_connected_at,
            last_introspect_at: record.last_introspect_at,
        }
    }
}

impl From<CreateConnectionModel> for CreateConnectionRecord {
    fn from(model: CreateConnectionModel) -> Self {
        Self {
            workspace_id: model.workspace_id,
            connection_name: model.connection_name,
            connection_config: model.connection_config,
            last_connected_at: model.last_connected_at,
        }
    }
}

impl From<UpdateConnectionModel> for UpdateConnectionRecord {
    fn from(record: UpdateConnectionModel) -> Self {
        Self {
            connection_name: record.connection_name,
            connection_config: record.connection_config,
            last_connected_at: record.last_connected_at,
        }
    }
}
