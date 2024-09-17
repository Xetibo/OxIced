use iced::widget::{column, text, Column};
use iced::{Alignment, Length, Theme};
use theme::{get_all_themes, get_theme};
use widgets::oxi_button::{button, ButtonVariant};
use widgets::oxi_checkbox::checkbox;
use widgets::oxi_picklist::pick_list;
use widgets::oxi_progress::progress_bar;
use widgets::oxi_radio::radio;
use widgets::oxi_rule::{horizontal_rule, vertical_rule};
use widgets::oxi_slider::slider;
use widgets::oxi_text_input::text_input;
use widgets::oxi_toggler::toggler;

mod theme;
mod widgets;

pub fn main() -> iced::Result {
    iced::application("pingpang", Counter::update, Counter::view)
        .theme(Counter::theme)
        .run()
}

struct Counter {
    value: i64,
    slider_value: i64,
    theme: Theme,
    is_checked: bool,
    is_toggled: bool,
    text: String,
}

impl Default for Counter {
    fn default() -> Self {
        Self {
            value: 0,
            slider_value: 0,
            theme: get_theme(),
            is_checked: false,
            is_toggled: false,
            text: "".into(),
        }
    }
}

#[derive(Debug, Clone)]
enum Message {
    Slider(i64),
    Increment(i64),
    Decrement(i64),
    TextChanged(String),
    Check(),
    Toggle(bool),
    Theme(Theme),
}

impl Counter {
    fn update(&mut self, message: Message) {
        match message {
            Message::Slider(val) => {
                self.slider_value = val;
            }
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
        }
    }

    fn view(&self) -> Column<Message> {
        column![
            counter_box(self),
            pick_list(get_all_themes(), Some(&self.theme), Message::Theme).width(Length::Fill),
        ]
        .padding(20)
        .align_items(Alignment::Center)
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
        checkbox("what", state.is_checked, |_| { Message::Check() }),
        radio("first", 10, None, Message::Increment),
        radio("second", 20, None, Message::Increment),
        slider(0..=100, state.slider_value as i32, |val| Message::Slider(
            val as i64
        )),
        text_input("something", state.text.as_str(), Message::TextChanged),
        toggler(Some(state.text.clone()), state.is_toggled, Message::Toggle),
        progress_bar(0.0..=100.0, state.slider_value as f32),
        horizontal_rule(100),
        vertical_rule(100),
    ]
    .padding(20)
    .align_items(Alignment::Center)
}
