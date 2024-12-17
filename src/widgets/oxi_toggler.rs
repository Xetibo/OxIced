use iced_core::Theme;
use iced_widget::{
    text::LineHeight,
    toggler::{Status, Style},
    Toggler,
};

pub fn toggler_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: palette.primary.base.color,
        background_border_width: 2.0,
        background_border_color: palette.primary.strong.color,
        foreground: palette.background.base.text,
        foreground_border_width: 2.0,
        foreground_border_color: palette.background.base.text,
    };
    match status {
        Status::Active { is_toggled: _ } => {
            style.background = palette.primary.weak.color;
            style
        }
        Status::Hovered { is_toggled: _ } => {
            style.background = palette.primary.weak.color;
            style
        }
        // TODO
        Status::Disabled => style,
    }
}

pub fn toggler<'a, M>(is_checked: bool) -> Toggler<'a, M> {
    // TODO label and on toggle
    iced_widget::toggler(is_checked)
        .text_line_height(LineHeight::Relative(4.0))
        .size(30)
        .style(toggler_style)
}
