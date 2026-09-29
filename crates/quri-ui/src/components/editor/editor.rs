use crate::theme;
use gpui::*;
use gpui_component::{
    input::{Input, InputState},
    StyledExt,
};

#[derive(IntoElement)]
pub struct Editor {
    content: SharedString,
}

impl Editor {
    pub fn new(content: SharedString) -> Self {
        Self { content }
    }
}

impl RenderOnce for Editor {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        let state = window.use_keyed_state("main-editor", cx, |window, cx| {
            InputState::new(window, cx)
                .code_editor("sql")
                .searchable(true)
                .show_whitespaces(false)
                .default_value(self.content)
        });

        div()
            .size_full()
            .text_color(theme::colors::TEXT)
            .bg(theme::colors::CARD)
            .child(
                Input::new(&state)
                    .h_full()
                    .bordered(false)
                    .focus_bordered(false)
                    .text_color(theme::colors::TEXT)
                    .bg(theme::colors::CARD)
                    .text_lg()
                    .font_semibold()
                    .font_family("JetBrains Mono")
                    .size_full(),
            )
    }
}
