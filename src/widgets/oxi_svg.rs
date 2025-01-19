use std::path::PathBuf;

use iced::widget::svg::{Status, Style};
use iced::Theme;

pub enum SvgStyleVariant {
    Primary,
    Secondary,
}

pub fn svg_style(variant: &SvgStyleVariant, theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let color = match variant {
        SvgStyleVariant::Primary => palette.primary.base.text,
        SvgStyleVariant::Secondary => palette.secondary.base.text,
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
