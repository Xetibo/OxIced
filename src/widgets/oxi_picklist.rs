use std::borrow::Borrow;

use iced_core::{border::Radius, Border, Theme};

use iced_widget::{
    overlay::menu,
    {self, PickList},
};

use super::common::darken_color;

pub fn picklist_style(
    theme: &Theme,
    status: iced_widget::pick_list::Status,
) -> iced_widget::pick_list::Style {
    let palette = theme.extended_palette();
    let mut style = iced_widget::pick_list::Style {
        background: iced_core::Background::Color(palette.background.weak.color),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.weak.color,
            width: 0.0,
            radius: Radius::from(10),
        },
        placeholder_color: darken_color(palette.background.strong.color),
        handle_color: darken_color(palette.background.strong.color),
    };
    match status {
        iced_widget::pick_list::Status::Active => style,
        iced_widget::pick_list::Status::Hovered => {
            style.background =
                iced_core::Background::Color(darken_color(palette.background.strong.color));
            style
        }
        // TODO
        iced_widget::pick_list::Status::Opened { is_hovered } => {
            style.background =
                iced_core::Background::Color(darken_color(palette.background.strong.color));
            style
        }
    }
}

pub fn menu_style(theme: &Theme) -> menu::Style {
    let palette = theme.extended_palette();
    menu::Style {
        background: iced_core::Background::Color(palette.background.weak.color),
        text_color: palette.background.base.text,
        border: Border {
            color: palette.background.strong.color,
            width: 0.0,
            radius: Radius::from(10),
        },
        selected_text_color: palette.background.base.text,
        selected_background: iced_core::Background::Color(darken_color(
            palette.background.strong.color,
        )),
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
    iced_widget::pick_list(options, selected, on_selected)
        .padding(10)
        .style(picklist_style)
        .menu_style(menu_style)
}
