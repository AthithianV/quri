use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct QueryHistoryRecord {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub workspace_id: Uuid,
    pub query_text: String,
    pub executed_at: NaiveDateTime,
    pub execution_time_ms: i32,
    pub rows_returned: i32,
    pub status: String,
    pub error_message: String,
    pub created_at: NaiveDateTime,
}
