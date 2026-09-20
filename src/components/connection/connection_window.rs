use crate::{
    components::layouts::titlebar::TitleBar,
    entity::connection::{ConnectionConfig, MySqlConfig, PostgresConfig, SqliteConfig},
    services::connection::{ConnectionService, CreateConnection},
    state::app_state::AppState,
    theme,
};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants as _},
    h_flex,
    input::{Input, InputState},
    select::{SearchableVec, Select, SelectEvent, SelectState},
    v_flex, ActiveTheme as _, Icon, IconName, IndexPath, StyledExt,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum DatabaseType {
    Postgres,
    MySql,
    Sqlite,
}

impl DatabaseType {
    fn label(self) -> &'static str {
        match self {
            Self::Postgres => "Postgres",
            Self::MySql => "MySQL",
            Self::Sqlite => "Sqlite",
        }
    }
}

#[derive(Clone)]
struct DatabaseTypeOption {
    label: SharedString,
    value: DatabaseType,
}

impl DatabaseTypeOption {
    fn new(value: DatabaseType) -> Self {
        Self {
            label: value.label().into(),
            value,
        }
    }
}

impl gpui_component::searchable_list::SearchableListItem for DatabaseTypeOption {
    type Value = DatabaseType;

    fn title(&self) -> SharedString {
        self.label.clone()
    }

    fn value(&self) -> &Self::Value {
        &self.value
    }
}

pub struct ConnectionWindow {
    database_type: DatabaseType,
    database_type_select: Entity<SelectState<SearchableVec<DatabaseTypeOption>>>,
    name: Entity<InputState>,
    host: Entity<InputState>,
    port: Entity<InputState>,
    username: Entity<InputState>,
    password: Entity<InputState>,
    database: Entity<InputState>,
    sqlite_path: Entity<InputState>,
    _subscriptions: Vec<Subscription>,
}

impl ConnectionWindow {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        let database_type_select = cx.new(|cx| {
            SelectState::new(
                SearchableVec::new(vec![
                    DatabaseTypeOption::new(DatabaseType::Postgres),
                    DatabaseTypeOption::new(DatabaseType::MySql),
                    DatabaseTypeOption::new(DatabaseType::Sqlite),
                ]),
                Some(IndexPath::default().row(0)),
                window,
                cx,
            )
        });

        let name = cx.new(|cx| InputState::new(window, cx).placeholder("Connection name"));
        let host = cx.new(|cx| InputState::new(window, cx).placeholder("localhost"));
        let port = cx.new(|cx| InputState::new(window, cx).placeholder("5432"));
        let username = cx.new(|cx| InputState::new(window, cx).placeholder("Username"));
        let password = cx.new(|cx| InputState::new(window, cx).placeholder("Password"));
        let database = cx.new(|cx| InputState::new(window, cx).placeholder("Database name"));
        let sqlite_path =
            cx.new(|cx| InputState::new(window, cx).placeholder("/path/to/database.sqlite"));

        let _subscriptions = vec![cx.subscribe_in(&database_type_select, window, {
            let port = port.clone();
            move |this, _, event, window, cx| {
                let SelectEvent::Confirm(selected) = event;
                let Some(database_type) = selected else {
                    return;
                };

                this.database_type = *database_type;
                if matches!(database_type, DatabaseType::Postgres) {
                    port.update(cx, |state, cx| state.set_value("5432", window, cx));
                } else if matches!(database_type, DatabaseType::MySql) {
                    port.update(cx, |state, cx| state.set_value("3306", window, cx));
                }
                cx.notify();
            }
        })];

        Self {
            database_type: DatabaseType::Postgres,
            database_type_select,
            name,
            host,
            port,
            username,
            password,
            database,
            sqlite_path,
            _subscriptions,
        }
    }

    fn labeled_input(label: &'static str, input: &Entity<InputState>) -> Div {
        v_flex()
            .gap_1()
            .child(div().text_sm().font_semibold().child(label))
            .child(
                Input::new(input)
                    .w_full()
                    .py_5()
                    .px_3()
                    .text_color(theme::colors::TEXT)
                    .bg(theme::colors::BACKGROUND),
            )
    }

    fn sqlite_path_input(input: &Entity<InputState>) -> Div {
        let input_for_picker = input.clone();

        v_flex()
            .gap_1()
            .flex()
            .child(div().text_sm().font_semibold().child("File path"))
            .child(
                div()
                    .flex()
                    .gap_3()
                    .child(
                        Input::new(input)
                            .w_full()
                            .py_5()
                            .px_3()
                            .bg(theme::colors::BACKGROUND),
                    )
                    .child(
                        Button::new("choose-sqlite-file")
                            .icon(Icon::new(IconName::File))
                            .secondary()
                            .py_5()
                            .w(px(200.0))
                            .label("Choose")
                            .on_click(move |_, window, cx| {
                                let paths = cx.prompt_for_paths(PathPromptOptions {
                                    files: true,
                                    directories: false,
                                    multiple: false,
                                    prompt: Some("Choose SQLite database".into()),
                                });
                                let input = input_for_picker.clone();

                                window
                                    .spawn(cx, async move |cx| {
                                        let Ok(Ok(Some(paths))) = paths.await else {
                                            return;
                                        };
                                        let Some(path) = paths.into_iter().next() else {
                                            return;
                                        };
                                        let path = path.to_string_lossy().into_owned();

                                        cx.update(|window, cx| {
                                            let _ = input.update(cx, |state, cx| {
                                                state.set_value(path, window, cx);
                                            });
                                        })
                                        .ok();
                                    })
                                    .detach();
                            }),
                    ),
            )
    }

    fn input_value(input: &Entity<InputState>, cx: &App) -> String {
        input.read(cx).value().trim().to_string()
    }

    fn optional_input_value(input: &Entity<InputState>, cx: &App) -> Option<String> {
        let value = Self::input_value(input, cx);
        if value.is_empty() {
            None
        } else {
            Some(value)
        }
    }

    fn connection_config_from_form(
        database_type: DatabaseType,
        host: String,
        port: String,
        username: String,
        password: Option<String>,
        database_name: String,
        sqlite_path: String,
    ) -> Result<ConnectionConfig, String> {
        match database_type {
            DatabaseType::Postgres => Ok(ConnectionConfig::Postgres(PostgresConfig {
                host,
                port: port
                    .parse()
                    .map_err(|_| "Postgres port must be a number".to_string())?,
                database_name,
                username,
                password,
                ssl_mode: None,
                extra_params: None,
            })),
            DatabaseType::MySql => Ok(ConnectionConfig::MySql(MySqlConfig {
                host,
                port: port
                    .parse()
                    .map_err(|_| "MySQL port must be a number".to_string())?,
                database_name,
                username,
                password,
                extra_params: None,
            })),
            DatabaseType::Sqlite => {
                if sqlite_path.is_empty() {
                    return Err("SQLite file path is required".to_string());
                }

                Ok(ConnectionConfig::Sqlite(SqliteConfig {
                    file_path: sqlite_path,
                    extra_params: None,
                }))
            }
        }
    }
}

impl Render for ConnectionWindow {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let database_type = self.database_type;
        let name = self.name.clone();
        let host = self.host.clone();
        let port = self.port.clone();
        let username = self.username.clone();
        let password = self.password.clone();
        let database = self.database.clone();
        let sqlite_path = self.sqlite_path.clone();

        let connection_fields = match self.database_type {
            DatabaseType::Sqlite => v_flex()
                .gap_6()
                .child(
                    Self::sqlite_path_input(&self.sqlite_path)
                        .text_color(cx.theme().muted_foreground),
                )
                .into_any_element(),
            DatabaseType::Postgres | DatabaseType::MySql => v_flex()
                .gap_6()
                .child(
                    h_flex()
                        .gap_5()
                        .child(
                            Self::labeled_input("Host", &self.host)
                                .w_full()
                                .text_color(cx.theme().muted_foreground),
                        )
                        .child(
                            Self::labeled_input("Port", &self.port)
                                .w(px(250.0))
                                .text_color(cx.theme().muted_foreground),
                        ),
                )
                .child(
                    h_flex()
                        .gap_5()
                        .child(
                            Self::labeled_input("Username", &self.username)
                                .w_full()
                                .text_color(cx.theme().muted_foreground),
                        )
                        .child(
                            Self::labeled_input("Password", &self.password)
                                .w_full()
                                .text_color(cx.theme().muted_foreground),
                        ),
                )
                .child(
                    Self::labeled_input("Database", &self.database)
                        .text_color(cx.theme().muted_foreground),
                )
                .into_any_element(),
        };

        v_flex()
            .size_full()
            .rounded_lg()
            .bg(theme::colors::CARD)
            .overflow_hidden()
            .child(TitleBar::new("Connection".into()))
            .child(
                div()
                    .p_5()
                    .size_full()
                    .text_color(theme::colors::TEXT)
                    .rounded_lg()
                    .child(
                        v_flex()
                            .gap_6()
                            .child(
                                v_flex()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_semibold()
                                            .child("Database type")
                                            .text_color(cx.theme().muted_foreground),
                                    )
                                    .child(
                                        Select::new(&self.database_type_select)
                                            .placeholder("Choose database type")
                                            .bg(theme::colors::BACKGROUND)
                                            .py_5()
                                            .px_3()
                                            .w_full(),
                                    ),
                            )
                            .child(
                                Self::labeled_input("Connection name", &self.name)
                                    .text_color(cx.theme().muted_foreground),
                            )
                            .child(connection_fields),
                    )
                    .child(div().flex_1())
                    .child(
                        h_flex()
                            .justify_end()
                            .gap_2()
                            .py_5()
                            .child(Button::new("Cancel").secondary().label("Cancel").on_click(
                                |_, window, _| {
                                    window.remove_window();
                                },
                            ))
                            .child(
                                Button::new("Save Connection")
                                    .primary()
                                    .label("Save")
                                    .on_click(move |_, window, cx| {
                                        let Some(state) = cx.try_global::<AppState>().cloned()
                                        else {
                                            eprintln!("AppState is not initialized");
                                            return;
                                        };

                                        let connection_name =
                                            ConnectionWindow::optional_input_value(&name, cx);
                                        let host = ConnectionWindow::input_value(&host, cx);
                                        let port = ConnectionWindow::input_value(&port, cx);
                                        let username = ConnectionWindow::input_value(&username, cx);
                                        let password =
                                            ConnectionWindow::optional_input_value(&password, cx);
                                        let database_name =
                                            ConnectionWindow::input_value(&database, cx);
                                        let sqlite_path =
                                            ConnectionWindow::input_value(&sqlite_path, cx);

                                        let connection_config =
                                            match ConnectionWindow::connection_config_from_form(
                                                database_type,
                                                host,
                                                port,
                                                username,
                                                password,
                                                database_name,
                                                sqlite_path,
                                            ) {
                                                Ok(config) => config,
                                                Err(error) => {
                                                    eprintln!("Invalid connection form: {error}");
                                                    return;
                                                }
                                            };

                                        window
                                            .spawn(cx, async move |cx| {
                                                let result = save_connection(
                                                    state,
                                                    connection_name,
                                                    connection_config,
                                                );

                                                if let Err(error) = result {
                                                    eprintln!("Failed to save connection: {error}");
                                                    return;
                                                }

                                                cx.update(|window, _| window.remove_window()).ok();
                                            })
                                            .detach();
                                    }),
                            ),
                    ),
            )
    }
}

fn save_connection(
    state: AppState,
    connection_name: Option<String>,
    connection_config: ConnectionConfig,
) -> Result<(), String> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| format!("failed to create Tokio runtime: {error}"))?;

    runtime.block_on(async move {
        let db = state.get_app_db_connection().await?;
        let workspace = state.get_opened_workspace().await?;

        ConnectionService::create_connection(
            &db,
            CreateConnection {
                workspace_id: workspace.id,
                connection_name,
                connection_config,
                last_connected_at: None,
            },
        )
        .await
        .map(|_| ())
        .map_err(|error| error.to_string())
    })
}
