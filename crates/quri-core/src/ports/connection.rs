use async_trait::async_trait;
use uuid::Uuid;

use crate::{
    error::QuriResult,
    model::connection_model::{ConnectionModel, CreateConnectionModel, UpdateConnectionModel},
};

#[async_trait]
pub trait ConnectionRepository: Send + Sync {
    async fn create_connection(input: CreateConnectionModel) -> QuriResult<ConnectionModel>;

    async fn update_connection(
        id: Uuid,
        input: UpdateConnectionModel,
    ) -> QuriResult<Option<ConnectionModel>>;

    async fn delete_connection(id: Uuid) -> QuriResult<bool>;

    async fn fetch_connection_by_id(id: Uuid) -> QuriResult<Option<ConnectionModel>>;

    async fn fetch_connections_by_workspace_id(
        workspace_id: Uuid,
    ) -> QuriResult<Vec<ConnectionModel>>;
}
