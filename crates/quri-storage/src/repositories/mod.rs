pub mod connection_repository;
pub mod workspace_repository;

use std::sync::Arc;

use connection_repository::StorageConnectionRepository;
use sqlx::SqlitePool;
use workspace_repository::StorageWorkspaceRepository;

pub struct StorageRepositories {
    pub workspace: Arc<StorageWorkspaceRepository>,
    pub connection: Arc<StorageConnectionRepository>,
}

impl StorageRepositories {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            workspace: Arc::new(StorageWorkspaceRepository::new(pool.clone())),
            connection: Arc::new(StorageConnectionRepository::new(pool)),
        }
    }
}
