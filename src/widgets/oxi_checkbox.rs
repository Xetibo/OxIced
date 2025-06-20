use iced::{
    Border, Theme,
    border::Radius,
    widget::{
        Checkbox,
        checkbox::{Status, Style},
        text::LineHeight,
    },
};

use crate::theme::theme::OXITHEME;

pub fn checkbox_style(_: &Theme, status: Status) -> Style {
    let palette = OXITHEME;
    let mut style = Style {
        background: iced::Background::Color(palette.tertiary_bg),
        text_color: Some(palette.text),
        border: Border {
            color: palette.border_color_weak,
            width: 1.0,
            radius: Radius::from(4),
        },
        icon_color: palette.text,
    };
    let mut checked_style = Style {
        background: iced::Background::Color(palette.primary),
        border: Border {
            color: palette.primary_contrast,
            ..style.border
        },
        icon_color: palette.primary_contrast,
        ..style
    };
    match status {
        Status::Active {
            is_checked: checked,
        } => match checked {
            true => {
                checked_style.background = iced::Background::Color(palette.primary_active);
                checked_style
            }
            false => style,
        },
        Status::Hovered {
            is_checked: checked,
        } => match checked {
            true => {
                checked_style.background = iced::Background::Color(palette.primary_hover);
                checked_style
            }
            false => {
                style.background = iced::Background::Color(palette.tertiary_bg_hover);
                style
            }
        },
        Status::Disabled {
            is_checked: checked,
        } => match checked {
            true => {
                style.background = iced::Background::Color(palette.primary_bg);
                style.icon_color = palette.secondary_bg;
                style
            }
            false => {
                style.background = iced::Background::Color(palette.primary_bg);
                style
            }
        },
    }
}

pub fn checkbox<'a, M>(
    label: impl Into<String>,
    is_checked: bool,
    user_on_toggle: impl Fn(bool) -> M + 'a,
) -> Checkbox<'a, M> {
    iced::widget::checkbox(label, is_checked)
        .size(25)
        .spacing(OXITHEME.padding_lg)
        .width(OXITHEME.padding_lg)
        .style(checkbox_style)
        .text_line_height(LineHeight::Relative(2.0))
        .on_toggle(user_on_toggle)
}
