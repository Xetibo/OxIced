use iced::{
    border::Radius,
    color,
    widget::button::{Status, Style},
    Border, Color, Element, Shadow, Theme, Vector,
};

use super::common::lighten_color;

pub enum ButtonVariant {
    Primary,
    Secondary,
    Success,
    Danger,
    LeftMenuEntry,
    RowEntry,
}

fn styled(background: Color, text: Color, shadow: Color) -> Style {
    Style {
        background: Some(iced::Background::Color(background)),
        text_color: text,
        border: Border {
            color: iced::Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(10),
        },
        shadow: Shadow {
            color: shadow,
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

fn states(status: Status, base: Style, pressed: Color, hovered: Color) -> Style {
    match status {
        Status::Active => base,
        Status::Pressed => Style {
            background: Some(iced::Background::Color(lighten_color(pressed))),
            ..base
        },
        Status::Hovered => Style {
            background: Some(iced::Background::Color(lighten_color(hovered))),
            ..base
        },
        Status::Disabled => disabled(base),
    }
}

pub fn primary_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette().primary;
    let base = styled(palette.base.color, palette.base.text, palette.weak.text);
    states(status, base, palette.strong.color, palette.base.color)
}

pub fn left_menu_entry(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let base = styled(
        palette.background.base.color,
        palette.background.base.text,
        palette.background.weak.color,
    );
    states(
        status,
        base,
        palette.primary.strong.color,
        palette.primary.base.color,
    )
}

// TODO beforepr
pub fn row_entry(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let color = color!(0x181825);
    let base = styled(color, palette.primary.base.text, palette.primary.weak.color);
    states(status, base, color, color)
}

pub fn secondary_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette().secondary;
    let base = styled(palette.base.color, palette.base.text, palette.weak.color);
    states(status, base, palette.strong.color, palette.base.color)
}

pub fn success_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette().success;
    let base = styled(palette.base.color, palette.base.text, palette.weak.color);
    states(status, base, palette.strong.color, palette.base.color)
}

pub fn danger_button(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette().danger;
    let base = styled(palette.base.color, palette.base.text, palette.weak.color);
    states(status, base, palette.strong.color, palette.base.color)
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
        ButtonVariant::LeftMenuEntry => left_menu_entry,
        ButtonVariant::RowEntry => row_entry,
    };
    iced::widget::button(content).padding(12).style(style)
}
