use iced::{
    border::Radius,
    widget::{
        rule::{FillMode, Style},
        Rule,
    },
    Pixels, Theme,
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
    iced::widget::vertical_rule(width).style(rule_style)
}

pub fn horizontal_rule<'a>(width: impl Into<Pixels>) -> Rule<'a> {
    iced::widget::horizontal_rule(width).style(rule_style)
}
