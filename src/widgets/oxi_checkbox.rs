use iced::{
    border::Radius,
    widget::{
        checkbox::{Status, Style},
        text::LineHeight,
        Checkbox,
    },
    Border, Theme,
};

use crate::Message;

pub fn checkbox_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced::Background::Color(palette.primary.base.color),
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
            style.background = iced::Background::Color(palette.primary.strong.color);
            style
        }
        Status::Disabled { is_checked: _ } => {
            style.background = iced::Background::Color(palette.primary.weak.color);
            style
        }
    }
}

pub fn checkbox<'a>(
    label: impl Into<String>,
    is_checked: bool,
    user_on_toggle: impl Fn(bool) -> Message + 'a,
) -> Checkbox<'a, Message> {
    iced::widget::checkbox(label, is_checked)
        .size(22)
        .spacing(10)
        .width(10)
        .style(checkbox_style)
        .text_line_height(LineHeight::Relative(2.0))
        .on_toggle(user_on_toggle)
}
