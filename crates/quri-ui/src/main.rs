mod components;
mod theme;
mod utils;
mod window;

// quri
use quri_core::utils::logging::setup_logging;

// gpui
use gpui::*;
use gpui_component::Root;
use window::root_window::RootWindow;

use crate::utils::{
    app_icon::{load_icon, Assets, CombinedAssets},
    global_app_state::GlobalAppState,
};

fn main() {
    let _log_guard = setup_logging().expect("failed to initialize logging");
    tracing::info!("Quri starting");

    let app = gpui_platform::application()
        .with_assets(CombinedAssets(Assets, gpui_component_assets::Assets));

    let state = GlobalAppState::new();

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::apply_component_theme(cx);

        // Initialize AppState as a Global
        cx.set_global(state);

        // Open the root window
        cx.open_window(
            WindowOptions {
                icon: Some(load_icon()),
                window_background: WindowBackgroundAppearance::Transparent,
                ..Default::default()
            },
            |window, cx| {
                let view = cx.new(|_| RootWindow::new());
                cx.new(|cx| Root::new(view, window, cx).window_shadow_size(px(0.0)))
            },
        )
        .expect("Failed to open window");
    });
}
