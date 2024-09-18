use iced::widget::{column, text, Column};
use iced::{event, Alignment, Element, Event, Length, Task, Theme};
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

use iced_layershell::actions::LayershellCustomActions;
use iced_layershell::reexport::Anchor;
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;

mod theme;
mod widgets;

//pub fn main() -> iced::Result {
pub fn main() -> Result<(), iced_layershell::Error> {
    let settings = Settings {
        layer_settings: LayerShellSettings {
            size: Some((0, 400)),
            exclusive_zone: 400,
            anchor: Anchor::Bottom | Anchor::Left | Anchor::Right,
            binded_output_name: Some("pingpang".into()),
            ..Default::default()
        },
        ..Default::default()
    };
    Counter::run(settings)
    //iced::application("pingpang", Counter::update, Counter::view)
    //    .theme(Counter::theme)
    //    .settings(settings)
    //    .run()
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

#[derive(Debug, Clone, Copy)]
enum WindowDirection {
    Top,
    Left,
    Right,
    Bottom,
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

    TextInput(String),
    Direction(WindowDirection),
    SizeChange((u32, u32)),
    IcedEvent(Event),
}
impl TryInto<LayershellCustomActions> for Message {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActions, Self::Error> {
        match self {
            Self::Direction(direction) => Ok(match direction {
                WindowDirection::Left => LayershellCustomActions::AnchorChange(
                    Anchor::Left | Anchor::Top | Anchor::Bottom,
                ),
                WindowDirection::Top => LayershellCustomActions::AnchorChange(
                    Anchor::Top | Anchor::Left | Anchor::Right,
                ),
                WindowDirection::Right => LayershellCustomActions::AnchorChange(
                    Anchor::Top | Anchor::Bottom | Anchor::Right,
                ),
                WindowDirection::Bottom => LayershellCustomActions::AnchorChange(
                    Anchor::Bottom | Anchor::Left | Anchor::Right,
                ),
            }),
            Self::SizeChange((x, y)) => Ok(LayershellCustomActions::SizeChange((x, y))),
            _ => Err(self),
        }
    }
}

impl Application for Counter {
    type Message = Message;
    type Flags = ();
    type Theme = Theme;
    type Executor = iced::executor::Default;

    fn new(_flags: ()) -> (Self, Task<Message>) {
        (
            Self {
                ..Default::default()
            },
            Task::none(),
        )
    }

    fn namespace(&self) -> String {
        String::from("Counter - Iced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        event::listen().map(Message::IcedEvent)
    }
    fn update(&mut self, message: Message) -> Task<Message> {
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
            _ => (),
        };
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        column![
            counter_box(self),
            pick_list(get_all_themes(), Some(&self.theme), Message::Theme).width(Length::Fill),
        ]
        .padding(20)
        .align_x(Alignment::Center)
        .into()
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
    .align_x(Alignment::Center)
}
