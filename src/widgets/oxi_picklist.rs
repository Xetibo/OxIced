use std::borrow::Borrow;

use iced::{
    border::Radius,
    color,
    overlay::menu,
    widget::{self, PickList},
    Border, Theme,
};

use super::common::{darken_color, lighten_color};

pub fn picklist_style(
    theme: &Theme,
    status: widget::pick_list::Status,
) -> widget::pick_list::Style {
    let palette = theme.extended_palette();
    let mut style = widget::pick_list::Style {
        background: iced::Background::Color(palette.background.weak.color),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.strong.color,
            width: 0.0,
            radius: Radius::from(10),
        },
        placeholder_color: darken_color(palette.background.strong.color),
        handle_color: darken_color(palette.background.strong.color),
    };
    match status {
        widget::pick_list::Status::Active => style,
        widget::pick_list::Status::Hovered => {
            style.background =
                iced::Background::Color(lighten_color(palette.background.weak.color));
            style
        }
        widget::pick_list::Status::Opened => {
            // TODO either same as hovered or same as normal
            // style.background =
            //     iced::Background::Color(darken_color(palette.background.strong.color));
            style
        }
    }
}

pub fn menu_style(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();
    menu::Style {
        background: iced::Background::Color(palette.background.weak.color),
        text_color: palette.background.base.text,
        border: Border {
            color: color!(0x89B4FA),
            width: 2.0,
            // TODO this should be dependend on the index
            radius: Radius::from(10),
        },
        selected_text_color: palette.background.base.text,
        selected_background: iced::Background::Color(darken_color(palette.background.strong.color)),
    }
}

pub fn pick_list<'a, T, L, V, M>(
    options: L,
    selected: Option<V>,
    on_selected: impl Fn(T) -> M + 'a,
) -> PickList<'a, T, L, V, M>
where
    T: ToString + PartialEq + Clone + 'a,
    L: Borrow<[T]> + 'a,
    V: Borrow<T> + 'a,
    M: Clone,
{
    iced::widget::pick_list(options, selected, on_selected)
        .padding(10)
        .style(picklist_style)
        .menu_style(menu_style)
}
