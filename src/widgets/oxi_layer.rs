use iced::{
    Alignment, Element, Length, Theme, theme,
    widget::container::{self, Container, Style},
};

use crate::theme::theme_impl::OXITHEME;

fn box_style(theme: &Theme) -> Style {
    let palette = &OXITHEME;
    Style {
        background: Some(iced::Background::Color(palette.mantle)),
        border: iced::border::color(palette.primary)
            .width(3)
            .rounded(palette.border_radius as u16),
        ..container::rounded_box(theme)
    }
}

pub fn rounded_layer<'a, T: 'a>(
    content: impl Into<Element<'a, T>>,
    max_size: (u32, u32),
) -> Element<'a, T> {
    let palette = &OXITHEME;
    Container::new(content)
        .style(box_style)
        .align_x(Alignment::Center)
        .padding(palette.padding_xl + palette.padding_xl)
        .max_width(max_size.0)
        .max_height(max_size.1)
        .width(Length::Fill)
        .into()
}

pub fn layer_theme() -> theme::Style {
    let palette = &OXITHEME;
    theme::Style {
        background_color: iced::Color::TRANSPARENT,
        text_color: palette.text,
    }
}
