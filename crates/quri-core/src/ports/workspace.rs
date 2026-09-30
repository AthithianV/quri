use async_trait::async_trait;
use quri_storage::record::workspace_record::WorkspaceRecord;
use uuid::Uuid;

use crate::error::QuriResult;

#[async_trait]
pub trait WorkspaceRepository: Send + Sync {
    async fn get_opened_workspaces(&self) -> QuriResult<Vec<WorkspaceRecord>>;

    async fn mark_workspace_opened(&self, id: Uuid) -> QuriResult<WorkspaceRecord>;

    async fn fetch_by_id(&self, id: Uuid) -> QuriResult<Option<WorkspaceRecord>>;

    async fn fetch_all(&self, id: Uuid) -> QuriResult<Option<WorkspaceRecord>>;
}
