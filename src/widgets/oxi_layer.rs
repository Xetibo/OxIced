use iced::{Alignment, Element, Length, Theme, widget::container, widget::container::Style};

use crate::theme::theme::OXITHEME;

fn box_style(theme: &Theme) -> Style {
    let palette = OXITHEME;
    Style {
        background: Some(iced::Background::Color(palette.base)),
        border: iced::border::color(palette.primary)
            .width(3)
            .rounded(palette.border_radius),
        ..container::rounded_box(theme)
    }
}

pub fn rounded_layer<'a, T: 'a>(
    content: impl Into<Element<'a, T>>,
    max_size: (u32, u32),
) -> Element<'a, T> {
    let palette = OXITHEME;
    container(content)
        .style(box_style)
        .align_x(Alignment::Center)
        .padding(palette.padding_xl + palette.padding_xl)
        .max_width(max_size.0 as u16)
        .max_height(max_size.1 as u16)
        .width(Length::Fill)
        .into()
}

pub fn layer_theme() -> iced_layershell::Appearance {
    let palette = OXITHEME;
    iced_layershell::Appearance {
        background_color: iced::Color::TRANSPARENT,
        text_color: palette.text,
    }
}
