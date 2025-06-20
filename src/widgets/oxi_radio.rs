use iced::{
    Theme,
    widget::{
        Radio,
        radio::{Status, Style},
        text::LineHeight,
    },
};

use crate::theme::theme::OXITHEME;

pub fn radio_style(_: &Theme, status: Status) -> Style {
    let palette = OXITHEME;
    let mut style = Style {
        background: iced::Background::Color(palette.secondary_bg),
        text_color: Some(palette.text),
        dot_color: palette.primary,
        border_width: 1.0,
        border_color: palette.text,
    };
    match status {
        Status::Active {
            is_selected: checked,
        } => {
            match checked {
                true => {
                    style.border_color = palette.primary;
                }
                false => (),
            }
            style
        }
        Status::Hovered {
            is_selected: checked,
        } => {
            match checked {
                true => {
                    style.border_color = palette.primary;
                }
                false => (),
            }
            style.background = iced::Background::Color(palette.secondary_bg_hover);
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
        .size(25)
        .spacing(OXITHEME.padding_lg)
        .style(radio_style)
        .text_line_height(LineHeight::Relative(2.0))
}
