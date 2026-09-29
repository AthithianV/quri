use crate::components::editor::editor::Editor;
use crate::theme;
use gpui::*;

pub struct MainPanel {
    _title: SharedString,
}

impl MainPanel {
    pub fn new(title: SharedString) -> Self {
        Self { _title: title }
    }
}

impl IntoElement for MainPanel {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .size_full()
            .p_2()
            .bg(theme::colors::CARD)
            .rounded_lg()
            .text_color(theme::colors::TEXT)
            .child(Editor::new("SELECT * FROM USERS".into()))
    }
}
