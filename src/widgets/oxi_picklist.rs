use std::borrow::Borrow;

use iced::{
    border::Radius,
    widget::{
        pick_list::{Status, Style},
        PickList,
    },
    Border, Theme,
};

use crate::Message;

pub fn picklist_style(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();
    let mut style = Style {
        background: iced::Background::Color(palette.primary.base.color),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.strong.color,
            width: 3.0,
            radius: Radius::from(10),
        },
        placeholder_color: palette.primary.weak.color,
        handle_color: palette.primary.strong.color,
    };
    match status {
        Status::Active => style,
        Status::Hovered => {
            style.background = iced::Background::Color(palette.primary.strong.color);
            style
        }
        Status::Opened => {
            style.background = iced::Background::Color(palette.primary.weak.color);
            style
        }
    }
}

pub fn pick_list<'a, T, L, V>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> Message + 'a,
) -> PickList<'a, T, L, V, Message>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
{
    iced::widget::pick_list(options, selected, on_selected)
        .padding(10)
        .style(picklist_style)
}
