use iced::{
    Border, Color, Theme,
    border::Radius,
    widget::{
        Slider,
        slider::{Handle, HandleShape, Rail, Status, Style},
    },
};

use crate::theme::theme_impl::OXITHEME;

pub fn slider_style(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let style = Style {
        rail: Rail {
            backgrounds: (
                iced::Background::Color(palette.primary),
                iced::Background::Color(palette.secondary_bg),
            ),
            width: 8.0,
            border: Border {
                color: Color::from_rgba(0.0, 0.0, 0.0, 0.0),
                width: 0.0,
                radius: Radius::new(palette.border_radius),
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 8.0 },
            border_width: 2.0,
            border_color: palette.text,
            background: iced::Background::Color(palette.text),
        },
    };
    match status {
        Status::Active => style,
        Status::Hovered => style,
        Status::Dragged => style,
    }
}

pub fn slider<'a, V, M>(
    range: std::ops::RangeInclusive<V>,
    value: V,
    on_change: impl Fn(V) -> M + 'a,
) -> Slider<'a, V, M>
where
    V: Copy + From<u8> + std::cmp::PartialOrd,
    M: Clone,
{
    iced::widget::slider(range, value, on_change).style(slider_style)
}
