use iced::{
    Border, Theme,
    border::Radius,
    widget::{ProgressBar, progress_bar::Style},
};

use crate::theme::theme::OXITHEME;

pub fn progress_style(_: &Theme) -> Style {
    let palette = OXITHEME;
    Style {
        background: iced::Background::Color(palette.secondary_bg),
        bar: iced::Background::Color(palette.primary),
        border: Border {
            color: palette.border_color_weak,
            width: 0.0,
            radius: Radius::from(palette.border_radius),
        },
    }
}

pub fn progress_bar<'a>(range: std::ops::RangeInclusive<f32>, value: f32) -> ProgressBar<'a> {
    iced::widget::progress_bar(range, value)
        .height(23)
        .style(progress_style)
}
