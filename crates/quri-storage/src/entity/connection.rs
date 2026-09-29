use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
use uuid::Uuid;

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "config", rename_all = "snake_case")]
pub enum ConnectionConfig {
    Postgres(PostgresConfig),
    #[serde(rename = "mysql")]
    MySql(MySqlConfig),
    Sqlite(SqliteConfig),
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostgresConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: Option<String>,
    pub ssl_mode: Option<String>,
    pub extra_params: Option<Value>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct MySqlConfig {
    pub host: String,
    pub port: u16,
    pub database_name: String,
    pub username: String,
    pub password: Option<String>,
    pub extra_params: Option<Value>,
}

#[allow(dead_code)]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SqliteConfig {
    pub file_path: String,
    pub extra_params: Option<Value>,
}

#[allow(dead_code)]
impl ConnectionConfig {
    pub fn database_type(&self) -> &'static str {
        match self {
            Self::Postgres(_) => "postgres",
            Self::MySql(_) => "mysql",
            Self::Sqlite(_) => "sqlite",
        }
    }

    pub fn into_json(self) -> Result<Value, serde_json::Error> {
        serde_json::to_value(self)
    }

    pub fn from_json(value: Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub workspace_id: Uuid,

    pub connection_name: Option<String>,
    pub connection_config: Value,
    pub last_connected_at: Option<NaiveDateTime>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

#[allow(dead_code)]
impl Model {
    pub fn config(&self) -> Result<ConnectionConfig, serde_json::Error> {
        ConnectionConfig::from_json(self.connection_config.clone())
    }
}
