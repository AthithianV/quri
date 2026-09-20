use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct Model {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub workspace_id: Uuid,
    pub title: String,
    pub query_text: String,
    pub cursor_position: i32,
    pub is_pinned: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
