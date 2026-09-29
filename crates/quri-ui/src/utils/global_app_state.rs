use gpui::Global;
use quri_core::state::app_state::AppState;

#[derive(Clone)]
pub struct GlobalAppState(AppState);

impl Global for GlobalAppState {}

impl GlobalAppState {
    pub fn new() -> Self {
        Self(AppState::new())
    }

    pub fn state(&self) -> AppState {
        self.0.clone()
    }
}
