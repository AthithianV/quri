use std::sync::Arc;

use crate::{
    error::QuriResult, model::workspace_model::WorkspaceModel,
    ports::workspace::WorkspaceRepository,
};

pub struct WorkspaceService {
    repository: Arc<dyn WorkspaceRepository>,
}

impl WorkspaceService {
    pub fn new(repository: Arc<dyn WorkspaceRepository>) -> Self {
        Self { repository }
    }

    #[tracing::instrument(skip(self, name))]
    pub async fn create_workspace(&self, name: String) -> QuriResult<WorkspaceModel> {
        self.repository.create_workspace(name).await
    }

    #[tracing::instrument(skip(self, name), fields(workspace_id = %id))]
    pub async fn update_workspace(
        &self,
        id: uuid::Uuid,
        name: String,
    ) -> QuriResult<Option<WorkspaceModel>> {
        self.repository.update_workspace(id, name).await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %id))]
    pub async fn delete_workspace(&self, id: uuid::Uuid) -> QuriResult<bool> {
        self.repository.delete_workspace(id).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_opened_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.repository.get_opened_workspaces().await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %id))]
    pub async fn open_workspace(&self, id: uuid::Uuid) -> QuriResult<WorkspaceModel> {
        self.repository.mark_workspace_opened(id).await
    }

    #[tracing::instrument(skip(self), fields(workspace_id = %id))]
    pub async fn get_workspace(&self, id: uuid::Uuid) -> QuriResult<Option<WorkspaceModel>> {
        self.repository.fetch_by_id(id).await
    }

    #[tracing::instrument(skip(self))]
    pub async fn get_all_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.repository.fetch_all().await
    }
}
