use iced::{
    border::Radius,
    widget::button::{Status, Style},
    Border, Element, Shadow, Theme, Vector,
};

use crate::Message;

pub fn button_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: Some(iced::Background::Color(palette.primary.base.color)),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.strong.color,
            width: 3.0,
            radius: Radius::from(10),
        },
        shadow: Shadow {
            color: theme.extended_palette().primary.weak.color,
            offset: Vector { x: 0.0, y: 0.0 },
            blur_radius: 2.0,
        },
    };
    match status {
        Status::Active => style,
        Status::Hovered => {
            style.background = Some(iced::Background::Color(palette.primary.strong.color));
            style
        }
        Status::Pressed => {
            style.background = Some(iced::Background::Color(palette.primary.weak.color));
            style
        }
        Status::Disabled => {
            style.background = Some(iced::Background::Color(palette.primary.weak.color));
            style
        }
    }
}

pub fn button<'a>(content: impl Into<Element<'a, Message>>) -> iced::widget::Button<'a, Message> {
    iced::widget::button(content)
        .padding(10)
        .style(button_style)
}
