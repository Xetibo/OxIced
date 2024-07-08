use iced::{
    border::Radius,
    widget::{progress_bar::Style, ProgressBar},
    Border, Theme,
};

pub fn progress_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: iced::Background::Color(palette.background.weak.color),
        bar: iced::Background::Color(palette.primary.strong.color),
        border: Border {
            color: palette.background.base.text,
            width: 1.0,
            radius: Radius::from(10.0),
        },
    }
}

pub fn progress_bar<'a>(range: std::ops::RangeInclusive<f32>, value: f32) -> ProgressBar<'a> {
    iced::widget::progress_bar(range, value).height(20).style(progress_style)
}
