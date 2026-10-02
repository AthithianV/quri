pub mod connection_service;
pub mod workspace_service;

use std::sync::Arc;

use connection_service::ConnectionService;
use workspace_service::WorkspaceService;

pub struct Services {
    pub workspace: Arc<WorkspaceService>,
    pub connection: Arc<ConnectionService>,
}
