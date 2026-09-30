use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Clone, Debug, PartialEq, Eq, FromRow, Serialize, Deserialize)]
pub struct DatabaseRecord {
    pub id: Uuid,
    pub connection_id: Uuid,
    pub provider: String,
    pub name: String,
    pub fetched_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
