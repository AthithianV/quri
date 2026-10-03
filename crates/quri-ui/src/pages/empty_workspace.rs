use crate::components::layouts::activity_bar::ActivityBar;
use crate::components::layouts::main_panel::MainPanel;
use crate::components::layouts::sidebar::{RightSideBar, SideBar};
use crate::components::layouts::titlebar::TitleBar;
use crate::theme;
use gpui::*;

pub struct EmptyWorkspacePage {}

impl EmptyWorkspacePage {
    pub fn new() -> Self {}
}

impl Render for EmptyWorkspacePage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .flex()
            .justify_center()
            .items_center()
            .child(
                h1().text_4xl()
                    .font_bold()
                    .text_center()
                    .child("Create a new workspace"),
            )
    }
}
