use std::cell::Cell;

use crate::widgets::oxi_svg::{SvgStyleVariant, svg_from_path};

pub const ICONPATH: Cell<&'static str> = Cell::new("./assets/{}.svg");

fn path<I: ToString>(icon: I) -> String {
    ICONPATH.get().replace("{}", &icon.to_string())
}

pub fn icon_widget<'a, I: ToString>(icon: I) -> iced::widget::Svg<'a> {
    svg_from_path(SvgStyleVariant::Primary, path(icon))
}

pub fn icon_widget_from_plain_path<'a>(plain_path: impl Into<String>) -> iced::widget::Svg<'a> {
    svg_from_path(SvgStyleVariant::Primary, plain_path.into())
}
