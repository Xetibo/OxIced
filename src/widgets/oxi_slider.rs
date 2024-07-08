use iced::{
    border::Radius,
    widget::{
        slider::{Handle, HandleShape, Rail, Status, Style},
        Slider,
    },
    Theme,
};

use crate::Message;

pub fn slider_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let style = Style {
        rail: Rail {
            colors: (palette.primary.strong.color, palette.primary.weak.color),
            width: 20.0,
            border_radius: Radius::from(10),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 8.0 },
            color: palette.primary.base.text,
            border_width: 2.0,
            border_color: palette.primary.base.text,
        },
    };
    match status {
        Status::Active => style,
        Status::Hovered => style,
        Status::Dragged => style,
    }
}

pub fn slider<'a, V>(
    range: std::ops::RangeInclusive<V>,
    value: V,
    on_change: impl Fn(V) -> Message + 'a,
) -> Slider<'a, V, Message>
where
    V: Copy + From<u8> + std::cmp::PartialOrd,
{
    iced::widget::slider(range, value, on_change).style(slider_style)
}
