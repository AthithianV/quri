use sqlx::SqlitePool;
use uuid::Uuid;

use crate::record::connection_record::{
    ConnectionRecord, CreateConnectionRecord, UpdateConnectionRecord,
};

pub struct StorageConnectionRepository {
    pool: SqlitePool,
}

impl StorageConnectionRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_connection(
        &self,
        input: CreateConnectionRecord,
    ) -> Result<ConnectionRecord, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();

        sqlx::query(
            "INSERT INTO connection
             (id, workspace_id, connection_name, connection_config,
              last_connected_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(input.workspace_id)
        .bind(input.connection_name)
        .bind(input.connection_config)
        .bind(input.last_connected_at)
        .bind(now)
        .bind(now)
        .execute(&self.pool)
        .await?;

        self.fetch_connection_by_id(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_connection(
        &self,
        id: Uuid,
        input: UpdateConnectionRecord,
    ) -> Result<Option<ConnectionRecord>, sqlx::Error> {
        let Some(current) = self.fetch_connection_by_id(id).await? else {
            return Ok(None);
        };

        let connection_name = input.connection_name.unwrap_or(current.connection_name);
        let connection_config = match input.connection_config {
            Some(config) => config,
            None => current.connection_config,
        };
        let last_connected_at = input.last_connected_at.unwrap_or(current.last_connected_at);
        let updated_at = chrono::Utc::now().naive_utc();

        sqlx::query(
            "UPDATE connection
             SET connection_name = ?, connection_config = ?,
                 last_connected_at = ?, updated_at = ?
             WHERE id = ?",
        )
        .bind(connection_name)
        .bind(connection_config)
        .bind(last_connected_at)
        .bind(updated_at)
        .bind(id)
        .execute(&self.pool)
        .await?;

        self.fetch_connection_by_id(id).await
    }

    pub async fn delete_connection(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM connection WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn fetch_connection_by_id(
        &self,
        id: Uuid,
    ) -> Result<Option<ConnectionRecord>, sqlx::Error> {
        sqlx::query_as::<_, ConnectionRecord>(
            "SELECT id, workspace_id, connection_name, connection_config,
                    last_connected_at,
                    last_introspected_at AS last_introspect_at,
                    created_at, updated_at
             FROM connection WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
    }

    pub async fn fetch_connections_by_workspace_id(
        &self,
        workspace_id: Uuid,
    ) -> Result<Vec<ConnectionRecord>, sqlx::Error> {
        sqlx::query_as::<_, ConnectionRecord>(
            "SELECT id, workspace_id, connection_name, connection_config,
                    last_connected_at,
                    last_introspected_at AS last_introspect_at,
                    created_at, updated_at
             FROM connection
             WHERE workspace_id = ?
             ORDER BY connection_name ASC, created_at ASC",
        )
        .bind(workspace_id)
        .fetch_all(&self.pool)
        .await
    }
}
