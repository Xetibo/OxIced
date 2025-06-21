use crate::theme::legacy_theme::get_all_themes;
use crate::theme::theme::get_derived_iced_theme;
use crate::widgets::oxi_button::{ButtonVariant, button};
use crate::widgets::oxi_card::Card;
use crate::widgets::oxi_checkbox::checkbox;
use crate::widgets::oxi_picklist::pick_list;
use crate::widgets::oxi_progress::progress_bar;
use crate::widgets::oxi_radio::radio;
use crate::widgets::oxi_rule::{horizontal_rule, vertical_rule};
use crate::widgets::oxi_slider::slider;
use crate::widgets::oxi_text_input::text_input;
use crate::widgets::oxi_toggler::toggler;
use iced::widget::{Column, column, text};
use iced::{Alignment, Length, Theme};

pub fn test_app() -> iced::Result {
    iced::application("pingpang", Counter::update, Counter::view)
        .theme(Counter::theme)
        .run()
}

struct Counter {
    value: i64,
    theme: Theme,
    is_checked: bool,
    is_toggled: bool,
    text: String,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: 0,
            theme: get_derived_iced_theme(),
            is_checked: false,
            is_toggled: false,
            text: "".into(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Increment(i64),
    Decrement(i64),
    Set(i64),
    TextChanged(String),
    Check(),
    Toggle(bool),
    Theme(Theme),
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Increment(val) => {
                self.value += val;
            }
            Message::Decrement(val) => {
                self.value -= val;
            }
            Message::Theme(theme) => self.theme = theme,
            Message::Check() => {
                self.is_checked = !self.is_checked;
            }
            Message::Toggle(val) => {
                self.is_toggled = val;
                self.text += "1";
            }
            Message::TextChanged(val) => self.text = val,
            Message::Set(val) => {
                self.value = val;
            }
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            counter_box(self),
            pick_list(get_all_themes(), Some(&self.theme), Message::Theme).width(Length::Fill),
        ]
        .padding(20)
        .align_x(Alignment::Center)
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }
}

fn counter_box<'a>(state: &Counter) -> Column<'a, Message> {
    column![
        button("Increment", ButtonVariant::Primary).on_press(Message::Increment(10)),
        text(state.value).size(50),
        button("Decrement", ButtonVariant::Secondary).on_press(Message::Decrement(20)),
        button("success", ButtonVariant::Success).on_press(Message::Increment(10)),
        button("danger", ButtonVariant::Danger).on_press(Message::Increment(10)),
        button("row", ButtonVariant::Neutral).on_press(Message::Increment(10)),
        checkbox("what", state.is_checked, |_| { Message::Check() }),
        radio("first", 10, Some(state.value), Message::Set),
        radio("second", 20, Some(state.value), Message::Set),
        slider(0.0..=100.0, state.value as f64, |val| Message::Set(
            val as i64
        )),
        // TODO broken
        toggler(state.is_toggled).on_toggle(Message::Toggle),
        progress_bar(0.0..=100.0, state.value as f32),
        Card::mk_title_card(
            String::from("test"),
            text_input("something", state.text.as_str(), Message::TextChanged).into(),
        ),
        horizontal_rule(10),
        vertical_rule(10),
    ]
    .padding(20)
    .align_x(Alignment::Center)
}
