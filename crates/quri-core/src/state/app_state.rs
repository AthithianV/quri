use std::sync::Arc;
use tokio::sync::RwLock;

use crate::{model::workspace_model::WorkspaceModel, services::Services};

/// AppState is shared across the application
#[derive(Clone)]
pub struct AppState {
    pub inner: Arc<AppStateInner>,
}

pub struct AppStateInner {
    pub services: Services,
    pub active_workspace: RwLock<Option<WorkspaceModel>>,
}

impl AppState {
    pub fn new(services: Services) -> Self {
        Self {
            inner: Arc::new(AppStateInner {
                services,
                active_workspace: RwLock::new(None),
            }),
        }
    }

    #[allow(dead_code)]
    pub async fn get_active_workspace(&self) -> Result<WorkspaceModel, String> {
        let read_guard = self.inner.active_workspace.read().await;
        read_guard
            .clone()
            .ok_or_else(|| "Opened workspace not initialized yet".to_string())
    }

    pub async fn set_active_workspace(&self, workspace: WorkspaceModel) {
        let mut write_guard = self.inner.active_workspace.write().await;
        *write_guard = Some(workspace);
    }
}
