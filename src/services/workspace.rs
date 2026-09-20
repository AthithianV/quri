use crate::entity::workspace;
use sqlx::SqlitePool;
use uuid::Uuid;

const PRIMARY_WORKSPACE_ID: Uuid =
    Uuid::from_bytes([0, 0, 0, 0, 0, 0, 0x40, 0, 0x80, 0, 0, 0, 0, 0, 0, 1]);
const PRIMARY_WORKSPACE_NAME: &str = "Primary";

#[allow(dead_code)]
pub struct WorkspaceService;

#[allow(dead_code)]
impl WorkspaceService {
    pub async fn get_or_create_opened_workspace(
        db: &SqlitePool,
    ) -> Result<workspace::Model, sqlx::Error> {
        Self::repair_primary_workspace_id(db).await?;

        if let Some(workspace) = sqlx::query_as::<_, workspace::Model>(
            "SELECT id, name, is_opened, last_opened, created_at, updated_at
             FROM workspace WHERE is_opened = 1
             ORDER BY last_opened DESC LIMIT 1",
        )
        .fetch_optional(db)
        .await?
        {
            return Ok(workspace);
        }

        if let Some(workspace) = sqlx::query_as::<_, workspace::Model>(
            "SELECT id, name, is_opened, last_opened, created_at, updated_at
             FROM workspace WHERE name = ? LIMIT 1",
        )
        .bind(PRIMARY_WORKSPACE_NAME)
        .fetch_optional(db)
        .await?
        {
            return Self::mark_workspace_opened(db, workspace.id).await;
        }

        let now = chrono::Utc::now().naive_utc();
        sqlx::query(
            "INSERT INTO workspace
             (id, name, is_opened, last_opened, created_at, updated_at)
             VALUES (?, ?, 1, ?, ?, ?)",
        )
        .bind(PRIMARY_WORKSPACE_ID)
        .bind(PRIMARY_WORKSPACE_NAME)
        .bind(now)
        .bind(now)
        .bind(now)
        .execute(db)
        .await?;

        Self::fetch_by_id(db, PRIMARY_WORKSPACE_ID)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn mark_workspace_opened(
        db: &SqlitePool,
        id: Uuid,
    ) -> Result<workspace::Model, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workspace
             SET is_opened = 1, last_opened = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(now)
        .bind(now)
        .bind(id)
        .execute(db)
        .await?;

        Self::fetch_by_id(db, id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn fetch_by_id(
        db: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<workspace::Model>, sqlx::Error> {
        sqlx::query_as::<_, workspace::Model>(
            "SELECT id, name, is_opened, last_opened, created_at, updated_at
             FROM workspace WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    async fn repair_primary_workspace_id(db: &SqlitePool) -> Result<(), sqlx::Error> {
        sqlx::query(
            "UPDATE workspace SET id = ?
             WHERE name = ? AND typeof(id) = 'text'",
        )
        .bind(PRIMARY_WORKSPACE_ID)
        .bind(PRIMARY_WORKSPACE_NAME)
        .execute(db)
        .await
        .map(|_| ())
    }
}
