use crate::entity::connection::{self, ConnectionConfig};
use chrono::NaiveDateTime;
use sqlx::SqlitePool;
use uuid::Uuid;

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub struct CreateConnection {
    pub workspace_id: Uuid,
    pub connection_name: Option<String>,
    pub connection_config: ConnectionConfig,
    pub last_connected_at: Option<NaiveDateTime>,
}

#[derive(Clone, Debug, Default)]
#[allow(dead_code)]
pub struct UpdateConnection {
    pub connection_name: Option<Option<String>>,
    pub connection_config: Option<ConnectionConfig>,
    pub last_connected_at: Option<Option<NaiveDateTime>>,
}

#[allow(dead_code)]
pub struct ConnectionService;

#[allow(dead_code)]
impl ConnectionService {
    pub async fn create_connection(
        db: &SqlitePool,
        input: CreateConnection,
    ) -> Result<connection::Model, sqlx::Error> {
        let id = Uuid::new_v4();
        let now = chrono::Utc::now().naive_utc();
        let config = input
            .connection_config
            .into_json()
            .map_err(|error| sqlx::Error::Encode(Box::new(error)))?;

        sqlx::query(
            "INSERT INTO connection
             (id, workspace_id, connection_name, connection_config,
              last_connected_at, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(id)
        .bind(input.workspace_id)
        .bind(input.connection_name)
        .bind(config)
        .bind(input.last_connected_at)
        .bind(now)
        .bind(now)
        .execute(db)
        .await?;

        Self::fetch_connection_by_id(db, id)
            .await?
            .ok_or(sqlx::Error::RowNotFound)
    }

    pub async fn update_connection(
        db: &SqlitePool,
        id: Uuid,
        input: UpdateConnection,
    ) -> Result<Option<connection::Model>, sqlx::Error> {
        let Some(current) = Self::fetch_connection_by_id(db, id).await? else {
            return Ok(None);
        };

        let connection_name = input.connection_name.unwrap_or(current.connection_name);
        let connection_config = match input.connection_config {
            Some(config) => config
                .into_json()
                .map_err(|error| sqlx::Error::Encode(Box::new(error)))?,
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
        .execute(db)
        .await?;

        Self::fetch_connection_by_id(db, id).await
    }

    pub async fn delete_connection(db: &SqlitePool, id: Uuid) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM connection WHERE id = ?")
            .bind(id)
            .execute(db)
            .await?;
        Ok(result.rows_affected() > 0)
    }

    pub async fn fetch_connection_by_id(
        db: &SqlitePool,
        id: Uuid,
    ) -> Result<Option<connection::Model>, sqlx::Error> {
        sqlx::query_as::<_, connection::Model>(
            "SELECT id, workspace_id, connection_name, connection_config,
                    last_connected_at, created_at, updated_at
             FROM connection WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(db)
        .await
    }

    pub async fn fetch_connections_by_workspace_id(
        db: &SqlitePool,
        workspace_id: Uuid,
    ) -> Result<Vec<connection::Model>, sqlx::Error> {
        sqlx::query_as::<_, connection::Model>(
            "SELECT id, workspace_id, connection_name, connection_config,
                    last_connected_at, created_at, updated_at
             FROM connection
             WHERE workspace_id = ?
             ORDER BY connection_name ASC, created_at ASC",
        )
        .bind(workspace_id)
        .fetch_all(db)
        .await
    }
}
