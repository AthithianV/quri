use async_trait::async_trait;
use quri_core::{
    error::{QuriError, QuriResult},
    model::connection_model::{ConnectionModel, CreateConnectionModel, UpdateConnectionModel},
    ports::connection::ConnectionRepository,
};
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
}

#[async_trait]
impl ConnectionRepository for StorageConnectionRepository {
    async fn create_connection(
        &self,
        new_connection: CreateConnectionModel,
    ) -> QuriResult<ConnectionModel> {
        self.create_connection_record(new_connection.into())
            .await
            .map(Into::into)
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn update_connection(
        &self,
        id: Uuid,
        input: UpdateConnectionModel,
    ) -> QuriResult<Option<ConnectionModel>> {
        self.update_connection_record(id, input.into())
            .await
            .map(|record| record.map(Into::into))
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn delete_connection(&self, id: Uuid) -> QuriResult<bool> {
        self.delete_connection_record(id)
            .await
            .map_err(|err| QuriError::Storage(err.into()))
    }

    async fn fetch_connection_by_id(&self, id: Uuid) -> QuriResult<Option<ConnectionModel>> {
        self.fetch_connection_by_id_record(id)
            .await
            .map(|record| record.map(Into::into))
            .map_err(|error| QuriError::Storage(error.into()))
    }

    async fn fetch_connections_by_workspace_id(
        &self,
        workspace_id: Uuid,
    ) -> QuriResult<Vec<ConnectionModel>> {
        self.fetch_connections_by_workspace_id_record(workspace_id)
            .await
            .map(|records| records.into_iter().map(Into::into).collect())
            .map_err(|error| QuriError::Storage(error.into()))
    }
}

impl StorageConnectionRepository {
    async fn create_connection_record(
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

        self.fetch_connection_by_id_record(id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    async fn update_connection_record(
        &self,
        id: Uuid,
        input: UpdateConnectionRecord,
    ) -> Result<Option<ConnectionRecord>, sqlx::Error> {
        let Some(current) = self.fetch_connection_by_id_record(id).await? else {
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

        self.fetch_connection_by_id_record(id).await
    }

    async fn delete_connection_record(&self, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM connection WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    async fn fetch_connection_by_id_record(
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

    async fn fetch_connections_by_workspace_id_record(
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
