use crate::{
    components::connection::connection_window::ConnectionWindow, theme, utils::app_icon::AppIcon,
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

        this
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
                                                println!("Refreshed");
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
