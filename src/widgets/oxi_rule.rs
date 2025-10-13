use iced::{
    Pixels, Theme,
    border::Radius,
    widget::{
        Rule,
        rule::{FillMode, Style},
    },
};

use crate::theme::theme_impl::OXITHEME;

pub fn rule_style(_: &Theme) -> Style {
    let palette = &OXITHEME;
    Style {
        color: palette.primary,
        radius: Radius::from(2.0),
        fill_mode: FillMode::Percent(90.0),
        snap: false,
    }
}

pub fn vertical_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced::widget::rule::vertical(width).style(rule_style)
}

pub fn horizontal_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced::widget::rule::horizontal(width).style(rule_style)
}
