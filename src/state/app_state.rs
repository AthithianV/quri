use crate::entity::workspace;
use gpui::Global;
use sqlx::SqlitePool;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum DbType {
    Postgres,
    MySQL,
    SQLite,
}

#[derive(Clone)]
#[allow(dead_code)]
pub enum DbPoolType {
    Postgres(sqlx::Pool<sqlx::Postgres>),
    MySQL(sqlx::Pool<sqlx::MySql>),
    SQLite(sqlx::Pool<sqlx::Sqlite>),
}

/// AppState is shared across the application
#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub app_db: RwLock<Option<SqlitePool>>,
    pub opened_workspace: RwLock<Option<workspace::Model>>,
    #[allow(dead_code)]
    pub connection_pools: RwLock<HashMap<i64, ConnectionPool>>,
}

impl Global for AppState {}

impl AppState {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                app_db: RwLock::new(None),
                opened_workspace: RwLock::new(None),
                connection_pools: RwLock::new(HashMap::new()),
            }),
        }
    }

    #[allow(dead_code)]
    pub async fn add_connection_pool(&self, pool: ConnectionPool) {
        let mut pools = self.inner.connection_pools.write().await;
        pools.insert(pool.connection_id, pool);
    }

    #[allow(dead_code)]
    pub async fn remove_connection_pool(&self, connection_id: i64) {
        let mut pools = self.inner.connection_pools.write().await;
        pools.remove(&connection_id);
    }

    #[allow(dead_code)]
    pub async fn get_connection_pool(&self, connection_id: i64) -> Option<ConnectionPool> {
        let pools = self.inner.connection_pools.read().await;
        pools.get(&connection_id).cloned()
    }

    #[allow(dead_code)]
    pub async fn get_app_db_connection(&self) -> Result<SqlitePool, String> {
        let read_guard = self.inner.app_db.read().await;
        read_guard
            .clone()
            .ok_or_else(|| "App DB not initialized yet".to_string())
    }

    pub async fn set_app_db_connection(&self, db: SqlitePool) {
        let mut write_guard = self.inner.app_db.write().await;
        *write_guard = Some(db);
    }

    #[allow(dead_code)]
    pub async fn get_opened_workspace(&self) -> Result<workspace::Model, String> {
        let read_guard = self.inner.opened_workspace.read().await;
        read_guard
            .clone()
            .ok_or_else(|| "Opened workspace not initialized yet".to_string())
    }

    pub async fn set_opened_workspace(&self, workspace: workspace::Model) {
        let mut write_guard = self.inner.opened_workspace.write().await;
        *write_guard = Some(workspace);
    }
}

/// ConnectionPool is a wrapper around a connection pool
#[derive(Clone)]
#[allow(dead_code)]
pub struct ConnectionPool {
    pub connection_id: i64,
    pub name: String,
    pub db_type: DbType,
    pub pool: DbPoolType,
}

#[allow(dead_code)]
impl ConnectionPool {
    /// Helper to create a new UserConnection instance
    pub fn new(connection_id: i64, name: String, db_type: DbType, pool: DbPoolType) -> Self {
        Self {
            connection_id,
            name,
            db_type,
            pool,
        }
    }
}
