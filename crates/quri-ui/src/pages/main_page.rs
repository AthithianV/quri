use crate::components::layouts::activity_bar::ActivityBar;
use crate::components::layouts::main_panel::MainPanel;
use crate::components::layouts::sidebar::{RightSideBar, SideBar};
use crate::components::layouts::titlebar::TitleBar;
use crate::theme;
use gpui::*;

pub struct MainPage {}

impl MainPage {}

impl Render for MainPage {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .gap_1()
                    .p_1()
                    .pt_0()
                    .child(div().w_1_4().child(SideBar::new()))
                    .child(
                        div()
                            .flex_1()
                            .min_w_0()
                            .child(MainPanel::new(self.title.clone())),
                    )
                    .child(div().w_1_4().child(RightSideBar::new())),
            )
            .child(ActivityBar::new())
    }
}
