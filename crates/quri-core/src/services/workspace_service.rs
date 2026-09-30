use std::sync::Arc;

use crate::ports::workspace::WorkspaceRepository;

struct Service {
    respository: Arc<dyn WorkspaceRepository>,
}
