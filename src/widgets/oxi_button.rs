use iced::{
    border::Radius,
    widget::button::{Status, Style},
    Border, Element, Shadow, Theme, Vector,
};

use super::common::{lighten_color, StylingCategory};

pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
}

fn styled(palette: &impl StylingCategory) -> Style {
    Style {
        background: Some(iced::Background::Color(palette.base().color)),
        text_color: palette.base().text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(10),
        },
        shadow: Shadow {
            color: palette.weak().color,
            offset: Vector { x: 0.2, y: 0.2 },
            blur_radius: 2.0,
        },
    }
}

fn disabled(style: Style) -> Style {
    Style {
        background: style
            .background
            .map(|background| background.scale_alpha(0.5)),
        text_color: style.text_color.scale_alpha(0.5),
        ..style
    }
}

fn states(status: Status, base: Style, palette: &impl StylingCategory) -> Style {
    match status {
        Status::Active => base,
        Status::Pressed => Style {
            background: Some(iced::Background::Color(lighten_color(
                palette.strong().color,
            ))),
            ..base
        },
        Status::Hovered => Style {
            background: Some(iced::Background::Color(lighten_color(palette.base().color))),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

fn primary_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(&palette.primary);
    states(status, base, &palette.primary)
}

fn secondary_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(&palette.secondary);
    states(status, base, &palette.secondary)
}

fn success_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(&palette.success);
    states(status, base, &palette.success)
}

fn danger_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(&palette.danger);
    states(status, base, &palette.danger)
}

pub fn button<'a, M>(
    content: impl Into<
        Element<
            'a,
            M,
            Theme,
            iced_renderer::fallback::Renderer<iced_wgpu::Renderer, iced_tiny_skia::Renderer>,
        >,
    >,
    variant: ButtonVariant,
) -> iced::widget::Button<'a, M> {
    let style = match variant {
        ButtonVariant::Primary => primary_button,
        ButtonVariant::Secondary => secondary_button,
        ButtonVariant::Success => success_button,
        ButtonVariant::Danger => danger_button,
    };
    iced::widget::button(content).padding(12).style(style)
}
