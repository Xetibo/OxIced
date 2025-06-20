use iced::{
    Pixels, Theme,
    border::Radius,
    widget::{
        Rule,
        rule::{FillMode, Style},
    },
};

use crate::theme::theme::OXITHEME;

pub fn rule_style(_: &Theme) -> Style {
    let palette = OXITHEME;
    Style {
        color: palette.primary,
        width: 5,
        radius: Radius::from(2.0),
        fill_mode: FillMode::Percent(90.0),
    }
}

pub fn vertical_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced::widget::vertical_rule(width).style(rule_style)
}

pub fn horizontal_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced::widget::horizontal_rule(width).style(rule_style)
}
