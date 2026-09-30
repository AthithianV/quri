use crate::record::workspace_record::WorkspaceRecord;
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct StorageWorkspaceRepository {
    pool: SqlitePool,
}

impl StorageWorkspaceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_opened_workspaces(&self) -> Result<Vec<WorkspaceRecord>, sqlx::Error> {
        let mut transaction = self.pool.begin().await?;

        // This will make sure, if there is not active WS, make recently opened as active.
        sqlx::query(
            r#"
            WITH selected_workspace AS (
                SELECT COALESCE(
                    (
                        SELECT id
                        FROM workspace
                        WHERE is_active = 1
                          AND is_opened = 1
                        ORDER BY last_opened DESC
                        LIMIT 1
                    ),
                    (
                        SELECT id
                        FROM workspace
                        WHERE is_opened = 1
                        ORDER BY last_opened DESC
                        LIMIT 1
                    )
                ) AS id
            )
            UPDATE workspace
            SET is_active = CASE
                WHEN workspace.id = (SELECT id FROM selected_workspace)
                THEN 1
                ELSE 0
            END
            "#,
        )
        .execute(&mut *transaction)
        .await?;

        let workspaces = sqlx::query_as::<_, WorkspaceRecord>(
            "SELECT id, name, is_opened, is_active, last_opened, created_at, updated_at
             FROM workspace WHERE is_opened = 1
             ORDER BY last_opened DESC",
        )
        .fetch_all(&mut *transaction)
        .await?;

        transaction.commit().await?;

        Ok(workspaces)
    }

    async fn mark_workspace_opened(&self, id: Uuid) -> Result<WorkspaceRecord, sqlx::Error> {
        let now = chrono::Utc::now().naive_utc();
        sqlx::query(
            "UPDATE workspace
             SET is_opened = CASE WHEN id = ? THEN 1 ELSE is_opened END,
                 is_active = CASE WHEN id = ? THEN 1 ELSE 0 END,
                 last_opened = ?,
                 updated_at = ?",
        )
        .bind(id)
        .bind(id)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.fetch_by_id(id).await?.ok_or(sqlx::Error::RowNotFound)
    }

    async fn fetch_by_id(&self, id: Uuid) -> Result<Option<WorkspaceRecord>, sqlx::Error> {
        sqlx::query_as::<_, WorkspaceRecord>(
            "SELECT id, name, is_opened, is_active, last_opened, created_at, updated_at
             FROM workspace WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn fetch_all(&self) -> Result<Vec<WorkspaceRecord>, sqlx::Error> {
        sqlx::query_as::<_, WorkspaceRecord>(
            "SELECT id, name, is_opened, is_active, last_opened, created_at, updated_at
             FROM workspace",
        )
        .fetch_all(&self.pool)
        .await
    }
}
