mod components;
mod theme;
mod utils;
mod window;

use gpui::*;
use gpui_component::Root;
use image::RgbaImage;
use std::sync::Arc;
use window::root_window::RootWindow;

use crate::utils::app_icon::{Assets, CombinedAssets};

fn load_icon() -> Arc<RgbaImage> {
    let image = image::open("assets/logo.png")
        .expect("Failed to load logo")
        .to_rgba8();

    Arc::new(image)
}

// async fn init_db(state: AppState) {
//     let runtime = match tokio::runtime::Builder::new_current_thread()
//         .enable_all()
//         .build()
//     {
//         Ok(runtime) => runtime,
//         Err(error) => {
//             eprintln!("Failed to create Tokio runtime for local database: {error}");
//             return;
//         }
//     };
//
//     match runtime.block_on(initialize_local_db()) {
//         Ok(db_conn) => {
//             match runtime.block_on(WorkspaceService::get_or_create_opened_workspace(&db_conn)) {
//                 Ok(workspace) => runtime.block_on(state.set_opened_workspace(workspace)),
//                 Err(error) => eprintln!("Failed to fetch opened workspace: {error:?}"),
//             }
//
//             runtime.block_on(state.set_app_db_connection(db_conn));
//         }
//         Err(error) => eprintln!("Failed to initialize local database: {error:?}"),
//     }
// }

fn main() {
    let app = gpui_platform::application()
        .with_assets(CombinedAssets(Assets, gpui_component_assets::Assets));

    // let state = AppState::new();
    // let state_for_db = state.clone();

    app.run(move |cx| {
        gpui_component::init(cx);
        theme::apply_component_theme(cx);

        // Initialize AppState as a Global
        //         cx.set_global(state);

        // Spawn task to initialize the database
        //         cx.background_executor()
        //             .spawn(init_db(state_for_db.clone()))
        //             .detach();

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
