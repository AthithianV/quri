use crate::{
    components::connection::connection_window::ConnectionWindow,
    entity::connection::{ConnectionConfig, Model as ConnectionModel},
    services::connection::ConnectionService,
    state::app_state::AppState,
    theme,
    utils::app_icon::AppIcon,
};
use gpui::prelude::FluentBuilder as _;
use gpui::*;
use gpui_component::Root;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    list::ListItem,
    ActiveTheme as _, Icon, IconName, StyledExt,
};

#[derive(Clone)]
struct ConnectionListItem {
    id: SharedString,
    name: SharedString,
    database_type: SharedString,
    summary: SharedString,
}

pub struct ConnectionSideBar {
    connections: Vec<ConnectionListItem>,
    is_loading: bool,
    error: Option<SharedString>,
}

impl ConnectionSideBar {
    pub fn new(_window: &mut Window, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            connections: Vec::new(),
            is_loading: false,
            error: None,
        };

        this.refresh(cx);
        this
    }

    fn refresh(&mut self, cx: &mut Context<Self>) {
        let Some(state) = cx.try_global::<AppState>().cloned() else {
            self.error = Some("App state is not initialized".into());
            return;
        };

        self.is_loading = true;
        self.error = None;
        cx.notify();

        cx.spawn(async move |this, cx| {
            let result = load_connections(state);

            _ = this.update(cx, |this, cx| {
                this.is_loading = false;

                match result {
                    Ok(connections) => {
                        this.connections = connections;
                        this.error = None;
                    }
                    Err(error) => {
                        this.connections.clear();
                        this.error = Some(error.into());
                    }
                }

                cx.notify();
            });
        })
        .detach();
    }

    fn render_connection(
        &self,
        ix: usize,
        connection: &ConnectionListItem,
        cx: &mut App,
    ) -> impl IntoElement {
        ListItem::new(ix)
            .w_full()
            .rounded(cx.theme().radius)
            .px_2()
            .py_1()
            .text_color(cx.theme().muted_foreground)
            .child(
                h_flex()
                    .gap_2()
                    .items_center()
                    .child(Icon::new(IconName::Folder))
                    .child(
                        div().flex().flex_col().overflow_hidden().child(
                            div()
                                .text_sm()
                                .font_semibold()
                                .text_color(theme::colors::TEXT)
                                .child(connection.name.clone()),
                        ), // .child(
                           //     h_flex()
                           //         .gap_2()
                           //         .text_xs()
                           //         .child(connection.database_type.clone())
                           //         .child(connection.summary.clone()),
                           // ),
                    ),
            )
            .on_click({
                let id = connection.id.clone();
                move |_, _, _| {
                    println!("Selected connection: {id}");
                }
            })
    }
}

impl Render for ConnectionSideBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .px_2()
            .py_1()
            .bg(theme::colors::CARD)
            .rounded_lg()
            .text_color(theme::colors::TEXT)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .size_full()
                    .gap_2()
                    .child(
                        div()
                            .py_1()
                            .text_color(cx.theme().muted_foreground)
                            .font_bold()
                            .border_b_1()
                            .border_color(cx.theme().muted)
                            .flex()
                            .justify_between()
                            .items_center()
                            .child("Databases")
                            .child(
                                h_flex()
                                    .gap_1()
                                    .child(
                                        Button::new("Refresh Connections")
                                            .ghost()
                                            .icon(Icon::new(AppIcon::Refresh))
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                this.refresh(cx);
                                            })),
                                    )
                                    .child(
                                        Button::new("Add New Connection")
                                            .icon(Icon::new(IconName::Plus))
                                            .ghost()
                                            .on_click(|_, _, cx| {
                                                cx.open_window(
                                                    WindowOptions {
                                                        window_bounds: Some(
                                                            WindowBounds::centered(
                                                                size(px(650.0), px(650.0)),
                                                                cx,
                                                            ),
                                                        ),
                                                        window_background:
                                                            WindowBackgroundAppearance::Transparent,
                                                        ..Default::default()
                                                    },
                                                    |window, cx| {
                                                        let view = cx.new(|cx| {
                                                            ConnectionWindow::new(window, cx)
                                                        });
                                                        cx.new(|cx| Root::new(view, window, cx))
                                                    },
                                                )
                                                .expect("Failed to open connection window");
                                            }),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .size_full()
                            .when(self.is_loading, |this| {
                                this.child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .text_sm()
                                        .text_color(cx.theme().muted_foreground)
                                        .child("Loading connections..."),
                                )
                            })
                            .when_some(self.error.clone(), |this, error| {
                                this.child(
                                    div()
                                        .px_2()
                                        .py_1()
                                        .text_sm()
                                        .text_color(cx.theme().danger)
                                        .child(error),
                                )
                            })
                            .when(
                                !self.is_loading
                                    && self.error.is_none()
                                    && self.connections.is_empty(),
                                |this| {
                                    this.child(
                                        div()
                                            .px_2()
                                            .py_1()
                                            .text_sm()
                                            .text_color(cx.theme().muted_foreground)
                                            .child("No connections yet"),
                                    )
                                },
                            )
                            .children(self.connections.iter().enumerate().map(
                                |(ix, connection)| self.render_connection(ix, connection, cx),
                            )),
                    ),
            )
    }
}

fn load_connections(state: AppState) -> Result<Vec<ConnectionListItem>, String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| format!("failed to create Tokio runtime: {error}"))?;

    runtime.block_on(async move {
        let db = state.get_app_db_connection().await?;
        let workspace = state.get_opened_workspace().await?;
        let connections = ConnectionService::fetch_connections_by_workspace_id(&db, workspace.id)
            .await
            .map_err(|error| error.to_string())?;

        Ok(connections.into_iter().map(connection_list_item).collect())
    })
}

fn connection_list_item(connection: ConnectionModel) -> ConnectionListItem {
    match connection.config() {
        Ok(config) => {
            let database_type = config.database_type().to_string();
            let summary = connection_summary(&config);
            let name = connection
                .connection_name
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| fallback_connection_name(&config));

            ConnectionListItem {
                id: connection.id.to_string().into(),
                name: name.into(),
                database_type: database_type.into(),
                summary: summary.into(),
            }
        }
        Err(_) => ConnectionListItem {
            id: connection.id.to_string().into(),
            name: connection
                .connection_name
                .unwrap_or_else(|| "Invalid connection".to_string())
                .into(),
            database_type: "unknown".into(),
            summary: "Invalid config".into(),
        },
    }
}

fn fallback_connection_name(config: &ConnectionConfig) -> String {
    match config {
        ConnectionConfig::Postgres(config) => config.database_name.clone(),
        ConnectionConfig::MySql(config) => config.database_name.clone(),
        ConnectionConfig::Sqlite(config) => config.file_path.clone(),
    }
}

fn connection_summary(config: &ConnectionConfig) -> String {
    match config {
        ConnectionConfig::Postgres(config) => {
            format!("{}:{}/{}", config.host, config.port, config.database_name)
        }
        ConnectionConfig::MySql(config) => {
            format!("{}:{}/{}", config.host, config.port, config.database_name)
        }
        ConnectionConfig::Sqlite(config) => config.file_path.clone(),
    }
}
