use iced::{
    widget::{
        radio::{Status, Style},
        text::LineHeight,
        Radio,
    },
    Theme,
};

pub fn radio_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced::Background::Color(palette.primary.base.color),
        text_color: Some(palette.background.base.text),
        dot_color: palette.background.base.text,
        border_width: 1.0,
        border_color: palette.primary.strong.color,
    };
    match status {
        Status::Active { is_selected: _ } => style,
        Status::Hovered { is_selected: _ } => {
            style.background = iced::Background::Color(palette.primary.strong.color);
            style
        }
    }
}

pub fn radio<'a, V, M>(
    label: impl Into<String>,
    value: V,
    selected: Option<V>,
    on_click: impl FnOnce(V) -> M + 'a,
) -> Radio<'a, M>
where
    V: Copy + Eq,
    M: Clone,
{
    iced::widget::radio(label, value, selected, on_click)
        .size(20)
        .spacing(10)
        .style(radio_style)
        .text_line_height(LineHeight::Relative(2.0))
}
