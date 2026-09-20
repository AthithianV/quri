use gpui::*;
use gpui_component::{
    menu::AppMenuBar, ActiveTheme as _, StyledExt, TitleBar as ComponentTitleBar,
};

use crate::theme;

#[derive(IntoElement)]
pub struct TitleBar {
    title: SharedString,
}

impl TitleBar {
    pub fn new(title: SharedString) -> Self {
        Self { title }
    }
}

impl RenderOnce for TitleBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        ComponentTitleBar::new()
            .bg(theme::colors::BACKGROUND)
            .h_9()
            .border_color(cx.theme().border)
            .child(
                div()
                    .text_color(cx.theme().primary_foreground)
                    .font_bold()
                    .italic()
                    .child(div().text_sm().font_bold().child(self.title)),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .text_color(cx.theme().primary_foreground)
                    .child(AppMenuBar::new(cx)),
            )
    }
}
