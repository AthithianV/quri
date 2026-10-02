use std::sync::Arc;

use uuid::Uuid;

use crate::{
    error::QuriResult,
    model::connection_model::{ConnectionModel, CreateConnectionModel, UpdateConnectionModel},
    ports::connection::ConnectionRepository,
};

pub struct ConnectionService {
    repository: Arc<dyn ConnectionRepository>,
}

impl ConnectionService {
    pub fn new(repository: Arc<dyn ConnectionRepository>) -> Self {
        Self { repository }
    }

    async fn create_connection(&self, input: CreateConnectionModel) -> QuriResult<ConnectionModel> {
        self.repository.create_connection(input).await
    }

    async fn update_connection(
        &self,
        id: Uuid,
        input: UpdateConnectionModel,
    ) -> QuriResult<Option<ConnectionModel>> {
        self.repository.update_connection(id, input).await
    }

    async fn delete_connection(&self, id: Uuid) -> QuriResult<bool> {
        self.repository.delete_connection(id).await
    }

    async fn fetch_connection_by_id(&self, id: Uuid) -> QuriResult<Option<ConnectionModel>> {
        self.repository.fetch_connection_by_id(id).await
    }

    async fn fetch_connections_by_workspace_id(
        &self,
        workspace_id: Uuid,
    ) -> QuriResult<Vec<ConnectionModel>> {
        self.repository
            .fetch_connections_by_workspace_id(workspace_id)
            .await
    }
}
