use async_trait::async_trait;
use uuid::Uuid;

use crate::{error::QuriResult, model::workspace_model::WorkspaceModel};

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn get_opened_workspaces(&self) -> QuriResult<Vec<WorkspaceModel>>;

    async fn mark_workspace_opened(&self, id: Uuid) -> QuriResult<WorkspaceModel>;

    async fn fetch_by_id(&self, id: Uuid) -> QuriResult<Option<WorkspaceModel>>;

    async fn fetch_all(&self) -> QuriResult<Vec<WorkspaceModel>>;
}
