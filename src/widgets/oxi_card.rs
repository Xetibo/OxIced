use iced::{
    Alignment, Element, Length, Theme,
    alignment::{Horizontal, Vertical},
    border,
    widget::{Row, column, container::Style, row, text},
};

use crate::{
    theme::theme::OXITHEME,
    widgets::{
        oxi_button::{self, ButtonVariant},
        oxi_icon::icon_widget,
    },
};

pub enum CardHeader<'a, T: Clone, I: ToString> {
    Title(String),
    TitleWithIcon {
        title: String,
        icon: I,
    },
    TitleWithElement {
        title: String,
        element: Element<'a, T>,
    },
    Custom(Element<'a, T>),
}

impl<'a, T: Clone + 'a, I: ToString> CardHeader<'a, T, I> {
    fn mk_title(
        title: String,
        icon_opt: Option<I>,
        element: Option<Element<'a, T>>,
    ) -> Element<'a, T> {
        let palette = OXITHEME;
        let mut elems: Vec<Element<'a, T>> = vec![
            text(title).size(palette.font_xl).into(),
            column!(element.unwrap_or(row!().into()))
                .width(Length::Fill)
                .align_x(Alignment::End)
                .into(),
        ];
        if let Some(icon) = icon_opt {
            elems.insert(0, icon_widget::<I>(icon).into());
        }
        Row::from_vec(elems).padding(palette.padding_lg).into()
    }

    fn mk_header(self) -> Element<'a, T> {
        match self {
            CardHeader::Title(title) => Self::mk_title(title, None, None),
            CardHeader::TitleWithIcon { title, icon } => Self::mk_title(title, Some(icon), None),
            CardHeader::TitleWithElement { title, element } => {
                Self::mk_title(title, None, Some(element))
            }
            CardHeader::Custom(element) => element,
        }
    }
}

pub struct Card<'a, T: Clone + 'a, I: ToString> {
    header: Option<CardHeader<'a, T, I>>,
    body: Element<'a, T>,
    on_click: Option<fn() -> T>,
}

impl<'a, T: Clone + 'a> Card<'a, T, String> {
    pub fn mk_title_card(title: String, body: Element<'a, T>) -> Element<'a, T> {
        Self {
            header: Some(CardHeader::Title(title)),
            body,
            on_click: None,
        }
        .view()
    }
}

impl<'a, T: Clone + 'a, I: ToString + 'a> Card<'a, T, I> {
    fn style(_: &Theme) -> Style {
        let palette = OXITHEME;

        Style {
            background: Some(palette.mantle.into()),
            border: border::rounded(palette.border_radius),
            ..Style::default()
        }
    }

    fn clickable_card(
        element: impl Into<Element<'a, T>>,
        on_click: fn() -> T,
    ) -> impl Into<Element<'a, T>> {
        oxi_button::button(element, ButtonVariant::Neutral).on_press_with(on_click)
    }

    fn view(self) -> Element<'a, T> {
        let card = iced::widget::container(
            column!(
                self.header
                    .map(|value| value.mk_header())
                    .unwrap_or(row!().into()),
                self.body,
            )
            .spacing(OXITHEME.padding_md)
            .width(Length::Fill)
            .height(Length::Fill),
        )
        .padding(OXITHEME.padding_lg)
        .style(Self::style)
        .align_x(Horizontal::Center)
        .align_y(Vertical::Center);
        if let Some(click) = self.on_click {
            Self::clickable_card(card, click).into()
        } else {
            card.into()
        }
    }
}
