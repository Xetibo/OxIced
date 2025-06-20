use iced::{
    Theme,
    widget::{
        Toggler,
        text::LineHeight,
        toggler::{Status, Style},
    },
};

use crate::{
    theme::theme::OXITHEME,
    utils::color::{darken_color, disable_color},
};

pub fn toggler_style(_: &Theme, status: Status) -> Style {
    let palette = OXITHEME;
    let mut style = Style {
        background: palette.secondary_bg,
        background_border_width: 2.0,
        background_border_color: palette.secondary_bg,
        foreground: palette.primary,
        foreground_border_width: 2.0,
        foreground_border_color: palette.primary,
    };
    let toggled_style = Style {
        background: palette.primary,
        background_border_color: palette.primary,
        foreground: palette.primary_contrast,
        foreground_border_color: palette.primary_contrast,
        ..style
    };
    match status {
        Status::Active {
            is_toggled: toggled,
        } => {
            let mut new_style = match toggled {
                true => toggled_style,
                false => style,
            };
            new_style.foreground = darken_color(&palette.primary_contrast, palette.tint_amount);
            new_style.foreground_border_color =
                darken_color(&palette.primary_contrast, palette.tint_amount);
            new_style
        }
        Status::Hovered {
            is_toggled: toggled,
        } => {
            let mut new_style = match toggled {
                true => toggled_style,
                false => style,
            };
            new_style.foreground = darken_color(&palette.primary_contrast, palette.shade_amount);
            new_style.foreground_border_color =
                darken_color(&palette.primary_contrast, palette.shade_amount);
            new_style
        }
        // TODO
        Status::Disabled => {
            style.background = disable_color(&style.background);
            style.background_border_color = disable_color(&style.background_border_color);
            style.foreground = disable_color(&style.foreground);
            style.foreground_border_color = disable_color(&style.foreground_border_color);
            style
        }
    }
}

pub fn toggler<'a, M>(is_checked: bool) -> Toggler<'a, M> {
    // TODO label and on toggle
    iced::widget::toggler(is_checked)
        .text_line_height(LineHeight::Relative(4.0))
        .size(30)
        .style(toggler_style)
}
