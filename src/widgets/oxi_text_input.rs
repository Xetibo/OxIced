use iced::{
    border::Radius,
    widget::text_input::{Status, Style},
    Border, Theme,
};

use super::common::darken_color;

pub fn text_input_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced::Background::Color(palette.background.weak.color),
        border: Border {
            color: palette.background.strong.color,
            width: 0.0,
            radius: Radius::from(10),
        },
        icon: palette.background.base.text,
        placeholder: palette.primary.weak.text,
        value: palette.background.base.text,
        selection: palette.primary.strong.color,
    };
    match status {
        Status::Active => style,
        Status::Hovered => {
            // TODO: really? double darken
            style.background = iced::Background::Color(darken_color(darken_color(
                palette.background.strong.color,
            )));
            style
        }
        Status::Focused => {
            style.background = iced::Background::Color(darken_color(darken_color(
                palette.background.strong.color,
            )));
            style
        }
        Status::Disabled => {
            style.background = iced::Background::Color(palette.background.base.color);
            style
        }
    }
}

pub fn text_input<'a, M>(
    placeholder: &str,
    value: &str,
    on_text_changed: impl Fn(String) -> M + 'a,
) -> iced::widget::TextInput<'a, M>
where
    M: Clone,
{
    iced::widget::text_input(placeholder, value)
        .padding(10)
        .on_input(on_text_changed)
        .style(text_input_style)
}
