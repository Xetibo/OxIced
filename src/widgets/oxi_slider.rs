use iced_core::Theme;
use iced_widget::{
    slider::{Handle, HandleShape, Rail, Status, Style},
    Slider,
};

pub fn slider_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let style = Style {
        rail: Rail {
            backgrounds: (
                iced_core::Background::Color(palette.primary.base.color),
                iced_core::Background::Color(palette.background.weak.color),
            ),
            width: 20.0,
            // TODO
            border: Default::default(),
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 8.0 },
            border_width: 2.0,
            border_color: palette.primary.base.text,
            background: iced_core::Background::Color(palette.primary.base.text),
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
    iced_widget::slider(range, value, on_change).style(slider_style)
}
