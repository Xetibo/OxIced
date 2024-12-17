use iced_core::{border::Radius, Pixels, Theme};
use iced_widget::{
    rule::{FillMode, Style},
    Rule,
};

pub fn rule_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        color: palette.primary.strong.color,
        width: 5,
        radius: Radius::from(2.0),
        fill_mode: FillMode::Percent(90.0),
    }
}

pub fn vertical_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced_widget::vertical_rule(width).style(rule_style)
}

pub fn horizontal_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced_widget::horizontal_rule(width).style(rule_style)
}
