use iced::{
    Border, Color, Element, Length, Shadow, Theme, Vector,
    border::Radius,
    widget::button::{Status, Style},
};

use crate::theme::theme_impl::OXITHEME;

pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    PrimaryBg,
    SecondaryBg,
}

fn styled(background: Color, text: Color, shadow: Color) -> Style {
    Style {
        background: Some(iced::Background::Color(background)),
        text_color: text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(OXITHEME.border_radius as u16),
        },
        shadow: Shadow {
            color: shadow,
            offset: Vector { x: 0.2, y: 0.2 },
            blur_radius: 2.0,
        },
        snap: false,
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

fn states(status: Status, base: Style, pressed: Color, hovered: Color) -> Style {
    match status {
        Status::Active => base,
        Status::Pressed => Style {
            background: Some(iced::Background::Color(pressed)),
            ..base
        },
        Status::Hovered => Style {
            background: Some(iced::Background::Color(hovered)),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn primary_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let base = styled(palette.primary, palette.primary_contrast, palette.primary);
    states(status, base, palette.primary_active, palette.primary_hover)
}

pub fn primary_bg_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let color = palette.primary_bg;
    let base = styled(color, palette.text, color);
    states(
        status,
        base,
        palette.primary_bg_active,
        palette.primary_bg_hover,
    )
}

pub fn secondary_bg_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let color = palette.secondary_bg;
    let base = styled(color, palette.text, color);
    states(
        status,
        base,
        palette.secondary_bg_active,
        palette.secondary_bg_hover,
    )
}

pub fn secondary_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let base = styled(
        palette.secondary,
        palette.secondary_contrast,
        palette.secondary,
    );
    states(
        status,
        base,
        palette.secondary_active,
        palette.secondary_hover,
    )
}

pub fn success_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let base = styled(palette.good, palette.good_contrast, palette.good);
    states(status, base, palette.good_active, palette.good_hover)
}

pub fn danger_button(_: &Theme, status: Status) -> Style {
    let palette = &OXITHEME;
    let base = styled(palette.bad, palette.bad_contrast, palette.bad);
    states(status, base, palette.bad_active, palette.bad_hover)
}

pub fn button<'a, M>(
    content: impl Into<Element<'a, M>>,
    variant: ButtonVariant,
) -> iced::widget::Button<'a, M> {
    let style = match variant {
        ButtonVariant::Primary => primary_button,
        ButtonVariant::Secondary => secondary_button,
        ButtonVariant::Success => success_button,
        ButtonVariant::Danger => danger_button,
        ButtonVariant::PrimaryBg => primary_bg_button,
        ButtonVariant::SecondaryBg => secondary_bg_button,
    };
    iced::widget::button(content)
        .padding(OXITHEME.padding_md)
        .style(style)
}

pub fn row_button<'a, M>(
    content: impl Into<Element<'a, M>>,
    variant: ButtonVariant,
) -> iced::widget::Button<'a, M> {
    button(content, variant).width(Length::Fill)
}
