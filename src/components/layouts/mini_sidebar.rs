use crate::{theme, utils::app_icon::AppIcon};
use gpui::*;
use gpui_component::{
    button::{Button, ButtonVariants},
    ActiveTheme, Icon,
};

#[derive(IntoElement)]
pub struct MiniSidebar {}

impl MiniSidebar {
    pub fn new() -> Self {
        Self {}
    }
}

impl RenderOnce for MiniSidebar {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        div()
            .w_full()
            .px_1()
            .flex()
            .items_center()
            .justify_between()
            .bg(theme::colors::BACKGROUND)
            .text_xs()
            .text_color(theme::colors::TEXT_MUTED)
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .items_center()
                    .child(
                        Button::new("mini-sidebar-database")
                            .ghost()
                            .p_2()
                            .icon(Icon::new(AppIcon::Database)),
                    )
                    .child(
                        Button::new("mini-sidebar-history")
                            .ghost()
                            .p_2()
                            .icon(Icon::new(AppIcon::History)),
                    )
                    .child(
                        Button::new("mini-sidebar-saved-query")
                            .ghost()
                            .p_2()
                            .icon(Icon::new(AppIcon::Save)),
                    ),
            )
    }
}
