use std::borrow::Borrow;

use iced::{
    Border, Theme,
    border::Radius,
    overlay::menu,
    widget::{self, PickList},
};

use crate::theme::theme_impl::OXITHEME;

pub fn picklist_style(_: &Theme, status: widget::pick_list::Status) -> widget::pick_list::Style {
    let palette = &OXITHEME;
    let mut style = widget::pick_list::Style {
        background: iced::Background::Color(palette.primary_bg),
        text_color: palette.text,
        border: Border {
            color: palette.primary_bg,
            width: 1.0,
            radius: Radius::from(palette.border_radius as u16),
        },
        placeholder_color: palette.text,
        handle_color: palette.text,
    };
    match status {
        widget::pick_list::Status::Active => style,
        widget::pick_list::Status::Hovered => {
            style.background = iced::Background::Color(palette.primary_bg_hover);
            style
        }
        widget::pick_list::Status::Opened { is_hovered: _ } => {
            style.border.color = palette.primary;
            style
        }
    }
}

pub fn menu_style(_: &Theme) -> menu::Style {
    let palette = &OXITHEME;
    menu::Style {
        background: iced::Background::Color(palette.mantle),
        text_color: palette.text,
        border: Border {
            color: palette.primary,
            width: 2.0,
            radius: Radius::from(OXITHEME.border_radius as u16),
        },
        selected_text_color: palette.text,
        selected_background: iced::Background::Color(palette.primary_bg_hover),
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
        .padding(OXITHEME.padding_lg)
        .style(picklist_style)
        .menu_style(menu_style)
}
