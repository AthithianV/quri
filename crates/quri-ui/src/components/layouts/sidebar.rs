use crate::components::connection::connection_sidebar::ConnectionSideBar;
use crate::theme;
use gpui::*;

#[derive(IntoElement)]
pub struct SideBar {}

impl SideBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for SideBar {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let connection_sidebar = window.use_keyed_state("connection-sidebar", cx, |window, cx| {
            ConnectionSideBar::new(window, cx)
        });

        div().size_full().child(connection_sidebar)
    }
}

#[derive(IntoElement)]
pub struct RightSideBar {}

impl RightSideBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for RightSideBar {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        div()
            .size_full()
            .p_3()
            .bg(theme::colors::CARD)
            .rounded_lg()
            .text_color(theme::colors::TEXT_MUTED)
            .child("Select a tool from the right activity group")
    }
}
