use iced::{
    border::Radius,
    widget::{progress_bar::Style, ProgressBar},
    Border, Theme,
};

use super::common::lighten_color;

pub fn progress_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: iced::Background::Color(lighten_color(palette.background.weak.color)),
        bar: iced::Background::Color(palette.primary.base.color),
        border: Border {
            color: palette.background.base.text,
            width: 0.0,
            radius: Radius::from(10.0),
        },
    }
}

pub fn progress_bar<'a>(range: std::ops::RangeInclusive<f32>, value: f32) -> ProgressBar<'a> {
    iced::widget::progress_bar(range, value)
        .height(23)
        .style(progress_style)
}
