use crate::record::workspace_record::WorkspaceRecord;
use async_trait::async_trait;
use quri_core::{
    error::QuriError, error::QuriResult, model::workspace_model::WorkspaceModel,
    ports::workspace::WorkspaceRepository,
};
use sqlx::SqlitePool;
use uuid::Uuid;

pub struct StorageWorkspaceRepository {
    pool: SqlitePool,
}

impl StorageWorkspaceRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl WorkspaceRepository for StorageWorkspaceRepository {
    async fn get_opened_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.fetch_opened_records()
            .await
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn mark_workspace_opened(&self, id: Uuid) -> QuriResult<WorkspaceModel> {
        self.mark_opened_record(id)
            .await
            .map(Into::into)
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn fetch_by_id(&self, id: Uuid) -> QuriResult<Option<WorkspaceModel>> {
        self.fetch_record_by_id(id)
            .await
            .map(|record| record.map(Into::into))
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn fetch_all(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.fetch_all_records()
            .await
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(|error| QuriError::Storage(error.into()))
    }
}

impl StorageWorkspaceRepository {
    async fn fetch_opened_records(&self) -> Result<Vec<WorkspaceRecord>, sqlx::Error> {
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

    async fn mark_opened_record(&self, id: Uuid) -> Result<WorkspaceRecord, sqlx::Error> {
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

        self.fetch_record_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn fetch_record_by_id(&self, id: Uuid) -> Result<Option<WorkspaceRecord>, sqlx::Error> {
        sqlx::query_as::<_, WorkspaceRecord>(
            "SELECT id, name, is_opened, is_active, last_opened, created_at, updated_at
             FROM workspace WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    async fn fetch_all_records(&self) -> Result<Vec<WorkspaceRecord>, sqlx::Error> {
        sqlx::query_as::<_, WorkspaceRecord>(
            "SELECT id, name, is_opened, is_active, last_opened, created_at, updated_at
             FROM workspace",
        )
        .fetch_all(&self.pool)
        .await
    }
}
