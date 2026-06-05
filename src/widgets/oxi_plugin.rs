use iced::{
    Background, Border, Color, Element, Length, Shadow, Theme,
    border::Radius,
    widget::{Button, Container, button, container, text},
};

use crate::theme::theme_impl::OXITHEME;

pub const BAR_CONTROL_HEIGHT: f32 = 22.5;

pub fn text_primary(_: &Theme) -> text::Style {
    text::Style {
        color: Some(OXITHEME.text),
    }
}

pub fn text_muted(_: &Theme) -> text::Style {
    text::Style {
        color: Some(OXITHEME.text_muted),
    }
}

pub fn text_accent(_: &Theme) -> text::Style {
    text::Style {
        color: Some(OXITHEME.primary),
    }
}

pub fn bar_button_style(_: &Theme, status: button::Status) -> button::Style {
    let base = button::Style {
        background: None,
        text_color: OXITHEME.primary,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: Shadow::default(),
        snap: false,
    };
    match status {
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(OXITHEME.primary_bg_hover)),
            ..base
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(OXITHEME.primary_bg_active)),
            ..base
        },
        button::Status::Active | button::Status::Disabled => base,
    }
}

pub fn flat_primary_bg_button_style(_: &Theme, status: button::Status) -> button::Style {
    flat_bg_button_style(status, OXITHEME.primary_bg, OXITHEME.primary)
}

pub fn flat_secondary_bg_button_style(_: &Theme, status: button::Status) -> button::Style {
    flat_bg_button_style(status, OXITHEME.secondary_bg, OXITHEME.text)
}

fn flat_bg_button_style(status: button::Status, bg: Color, text: Color) -> button::Style {
    let background = match status {
        button::Status::Hovered => OXITHEME.primary_bg_hover,
        button::Status::Pressed => OXITHEME.primary_bg_active,
        button::Status::Active | button::Status::Disabled => bg,
    };
    button::Style {
        background: Some(Background::Color(background)),
        text_color: text,
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius::from(8.0),
        },
        shadow: Shadow::default(),
        snap: false,
    }
}

pub fn bar_button<'a, M>(content: impl Into<Element<'a, M>>) -> Button<'a, M> {
    button(content)
        .style(bar_button_style)
        .padding([0, 8])
        .height(BAR_CONTROL_HEIGHT)
        .width(Length::Shrink)
}

pub fn compact_card_style(bg: Color) -> impl Fn(&Theme) -> container::Style {
    move |_| container::Style {
        background: Some(Background::Color(bg)),
        border: Border {
            radius: Radius::from(10.0),
            color: Color::TRANSPARENT,
            width: 0.0,
        },
        shadow: Shadow::default(),
        ..Default::default()
    }
}

pub fn compact_card<'a, M>(content: impl Into<Element<'a, M>>, bg: Color) -> Container<'a, M> {
    Container::new(content)
        .style(compact_card_style(bg))
        .padding(10)
        .width(Length::Fill)
}

#[cfg(test)]
mod tests {
    use iced::widget::button;

    use super::*;

    #[test]
    fn bar_button_style_is_flat_until_hovered() {
        let theme = Theme::Dark;
        let active = bar_button_style(&theme, button::Status::Active);
        let hovered = bar_button_style(&theme, button::Status::Hovered);

        assert!(active.background.is_none());
        assert!(hovered.background.is_some());
        assert_eq!(active.shadow, Shadow::default());
    }

    #[test]
    fn text_helpers_use_theme_roles() {
        let theme = Theme::Dark;

        assert_eq!(text_primary(&theme).color, Some(OXITHEME.text));
        assert_eq!(text_muted(&theme).color, Some(OXITHEME.text_muted));
        assert_eq!(text_accent(&theme).color, Some(OXITHEME.primary));
    }
}
