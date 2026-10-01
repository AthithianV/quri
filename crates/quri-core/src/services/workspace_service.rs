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

    pub async fn get_opened_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.repository.get_opened_workspaces().await
    }

    pub async fn open_workspace(&self, id: uuid::Uuid) -> QuriResult<WorkspaceModel> {
        self.repository.mark_workspace_opened(id).await
    }

    pub async fn get_workspace(&self, id: uuid::Uuid) -> QuriResult<Option<WorkspaceModel>> {
        self.repository.fetch_by_id(id).await
    }

    pub async fn get_all_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>> {
        self.repository.fetch_all().await
    }
}
