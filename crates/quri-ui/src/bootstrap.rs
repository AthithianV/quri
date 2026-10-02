// quri-ui/src/bootstrap.rs

use std::sync::Arc;

use quri_core::{
    error::{QuriError, QuriResult},
    services::{
        connection_service::ConnectionService, workspace_service::WorkspaceService, Services,
    },
};
use quri_storage::initialize_repositories;

pub async fn bootstrap() -> QuriResult<Services> {
    let repositories = initialize_repositories()
        .await
        .map_err(|error| QuriError::Storage(anyhow::Error("unable to initiate Repositories")))?;

    Ok(Services {
        workspace: Arc::new(WorkspaceService::new(repositories.workspace)),
        connection: Arc::new(ConnectionService::new(repositories.connection)),
    })
}
