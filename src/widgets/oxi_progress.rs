use iced_core::{border::Radius, Border, Theme};

use iced_widget::{progress_bar::Style, ProgressBar};

pub fn progress_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: iced_core::Background::Color(palette.background.weak.color),
        bar: iced_core::Background::Color(palette.primary.base.color),
        border: Border {
            color: palette.background.base.text,
            width: 0.0,
            radius: Radius::from(10.0),
        },
    }
}

pub fn progress_bar<'a>(range: std::ops::RangeInclusive<f32>, value: f32) -> ProgressBar<'a> {
    iced_widget::progress_bar(range, value)
        .height(23)
        .style(progress_style)
}
