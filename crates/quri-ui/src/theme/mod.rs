pub mod colors;
pub mod heights;

use gpui::*;
use gpui_component::{
    highlighter::SyntaxColors,
    theme::{Theme, ThemeMode},
};

pub fn apply_component_theme(cx: &mut App) {
    Theme::change(ThemeMode::Dark, None, cx);

    let component_theme = Theme::global_mut(cx);

    component_theme.background = colors::BACKGROUND.into();
    component_theme.foreground = colors::TEXT.into();
    component_theme.primary_foreground = colors::TEXT.into();
    component_theme.muted_foreground = colors::TEXT_MUTED.into();
    component_theme.input = colors::CARD.into();
    component_theme.border = Hsla::transparent_black();
    component_theme.drag_border = colors::BACKGROUND.into();

    let highlight_style = &mut std::sync::Arc::make_mut(&mut component_theme.highlight_theme).style;

    highlight_style.editor_background = Some(colors::CARD.into());
    highlight_style.editor_foreground = Some(colors::TEXT.into());
    highlight_style.editor_line_number = Some(colors::TEXT_MUTED.into());
    highlight_style.editor_active_line_number = Some(colors::TEXT.into());
    highlight_style.editor_active_line = Some(colors::CARD.into());
    highlight_style.syntax = vscode_dark_syntax_colors();
}

fn vscode_dark_syntax_colors() -> SyntaxColors {
    serde_json::from_str(
        r##"{
            "comment": { "color": "#6A9955", "font_style": "italic" },
            "comment.doc": { "color": "#6A9955", "font_style": "italic" },
            "keyword": { "color": "#569CD6" },
            "operator": { "color": "#D4D4D4" },
            "boolean": { "color": "#569CD6" },
            "constant": { "color": "#4FC1FF" },
            "number": { "color": "#B5CEA8" },
            "string": { "color": "#CE9178" },
            "string.escape": { "color": "#D7BA7D" },
            "string.special": { "color": "#D7BA7D" },
            "function": { "color": "#DCDCAA" },
            "constructor": { "color": "#4EC9B0" },
            "type": { "color": "#4EC9B0" },
            "property": { "color": "#9CDCFE" },
            "variable": { "color": "#9CDCFE" },
            "variable.special": { "color": "#C586C0" },
            "attribute": { "color": "#9CDCFE" },
            "label": { "color": "#C8C8C8" },
            "punctuation": { "color": "#D4D4D4" },
            "punctuation.bracket": { "color": "#FFD700" },
            "punctuation.delimiter": { "color": "#D4D4D4" },
            "punctuation.special": { "color": "#D7BA7D" },
            "embedded": { "color": "#D4D4D4" },
            "primary": { "color": "#D4D4D4" },
            "preproc": { "color": "#C586C0" },
            "tag": { "color": "#569CD6" },
            "title": { "color": "#DCDCAA" }
        }"##,
    )
    .expect("valid VS Code syntax colors")
}
