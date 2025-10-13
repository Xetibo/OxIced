use std::path::PathBuf;

use iced::Theme;
use iced::widget::svg::{Status, Style};

use crate::theme::theme_impl::OXITHEME;

pub enum SvgStyleVariant {
    Primary,
    Secondary,
}

pub fn svg_style(variant: &SvgStyleVariant, _: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let color = match variant {
        SvgStyleVariant::Primary => palette.primary,
        SvgStyleVariant::Secondary => palette.secondary,
    };
    let style = Style { color: Some(color) };
    match status {
        // TODO differentiate perhaps?
        Status::Hovered => style,
        Status::Idle => style,
    }
}

pub fn svg_from_path<'a>(
    variant: SvgStyleVariant,
    svg_path: impl Into<PathBuf>,
) -> iced::widget::Svg<'a> {
    let handle = iced::widget::svg::Handle::from_path(svg_path);
    iced::widget::svg(handle).style(move |theme, status| svg_style(&variant, theme, status))
}
