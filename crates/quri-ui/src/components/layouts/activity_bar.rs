use crate::{theme, utils::app_icon::AppIcon};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    ActiveTheme, Icon, Sizable,
};

#[derive(IntoElement)]
pub struct ActivityBar {}

impl ActivityBar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for ActivityBar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .px_2()
            .py_0p5()
            .flex()
            .items_center()
            .justify_between()
            .gap_1()
            .bg(theme::colors::BACKGROUND)
            .text_xs()
            .text_color(theme::colors::TEXT_MUTED)
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("activity-left-database").ghost().small().icon(
                            Icon::new(AppIcon::Database).text_color(cx.theme().muted_foreground),
                        ),
                    )
                    .child(
                        Button::new("activity-left-history").ghost().small().icon(
                            Icon::new(AppIcon::History).text_color(cx.theme().muted_foreground),
                        ),
                    )
                    .child(
                        Button::new("activity-left-saved-query")
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::Save).text_color(cx.theme().muted_foreground)),
                    ),
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap_2()
                    .child(
                        Button::new("activity-right-code")
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::Code).text_color(cx.theme().muted_foreground)),
                    )
                    .child(
                        Button::new("activity-right-ai")
                            .ghost()
                            .small()
                            .icon(Icon::new(AppIcon::Ai).text_color(cx.theme().muted_foreground)),
                    ),
            )
    }
}
