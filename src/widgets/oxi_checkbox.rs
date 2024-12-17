use iced_core::{border::Radius, widget::text::LineHeight, Border, Theme};

use iced_widget::{
    checkbox::{Status, Style},
    Checkbox,
};

pub fn checkbox_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced_core::Background::Color(palette.primary.base.color),
        text_color: Some(palette.background.base.text),
        border: Border {
            color: palette.primary.strong.color,
            width: 1.0,
            radius: Radius::from(4),
        },
        icon_color: palette.background.base.text,
    };
    match status {
        Status::Active { is_checked: _ } => style,
        Status::Hovered { is_checked: _ } => {
            style.background = iced_core::Background::Color(palette.primary.strong.color);
            style
        }
        Status::Disabled { is_checked: _ } => {
            style.background = iced_core::Background::Color(palette.primary.weak.color);
            style
        }
    }
}

pub fn checkbox<'a, M>(
    label: impl Into<String>,
    is_checked: bool,
    user_on_toggle: impl Fn(bool) -> M + 'a,
) -> Checkbox<'a, M> {
    iced_widget::checkbox(label, is_checked)
        .size(22)
        .spacing(10)
        .width(10)
        .style(checkbox_style)
        .text_line_height(LineHeight::Relative(2.0))
        .on_toggle(user_on_toggle)
}
