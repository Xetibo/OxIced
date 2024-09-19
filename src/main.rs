use iced::theme::palette;
use iced::widget::container::Style;
use iced::widget::{column, container, text, Column};
use iced::{event, Alignment, Element, Event, Length, Subscription, Task, Theme};
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
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;

mod theme;
mod widgets;

//pub fn main() -> iced::Result {
pub fn main() -> Result<(), iced_layershell::Error> {
    let settings = Settings {
        layer_settings: LayerShellSettings {
            size: Some((600, 600)),
            exclusive_zone: 0,
            anchor: Anchor::Left | Anchor::Right,
            binded_output_name: Some("pingpang".into()),
            layer: Layer::Overlay,
            margin: (100, 100, 100, 100),
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
    //TextInput(String),
    //Direction(WindowDirection),
    //SizeChange((u32, u32)),
    //IcedEvent(Event),
}
impl TryInto<LayershellCustomActions> for Message {
    type Error = Self;
    fn try_into(self) -> Result<LayershellCustomActions, Self::Error> {
        Err(self)
    }
}
//match self {
//    Self::Direction(direction) => Ok(match direction {
//        WindowDirection::Left => LayershellCustomActions::AnchorChange(
//            Anchor::Left | Anchor::Top | Anchor::Bottom,
//        ),
//        WindowDirection::Top => LayershellCustomActions::AnchorChange(
//            Anchor::Top | Anchor::Left | Anchor::Right,
//        ),
//        WindowDirection::Right => LayershellCustomActions::AnchorChange(
//            Anchor::Top | Anchor::Bottom | Anchor::Right,
//        ),
//        WindowDirection::Bottom => LayershellCustomActions::AnchorChange(
//            Anchor::Bottom | Anchor::Left | Anchor::Right,
//        ),
//    }),
//    Self::SizeChange((x, y)) => Ok(LayershellCustomActions::SizeChange((x, y))),
//    _ => Err(self),
//}
//        Err(self)
//    }
//}
fn box_style(theme: &Theme) -> Style {
    let palette = theme.extended_palette();
    Style {
        background: Some(iced::Background::Color(palette.background.base.color)),
        border: iced::border::rounded(10),
        ..container::rounded_box(theme)
    }
}

fn wrap_in_rounded_box<'a>(
    content: impl Into<Element<'a, Message, Theme>>,
) -> Element<'a, Message> {
    container(content)
        .style(box_style)
        .align_x(Alignment::Center)
        .padding(50)
        .max_width(550)
        .into()
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
        String::from("Oxiced")
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        //event::listen().map(Message::IcedEvent)
        Subscription::none()
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
            //_ => (),
        };
        Task::none()
    }

    fn view(&self) -> Element<Message> {
        wrap_in_rounded_box(
            column![
                testing_box_2(self),
                pick_list(get_all_themes(), Some(&self.theme), Message::Theme).width(Length::Fill),
            ]
            .padding(20)
            .max_width(530)
            .align_x(Alignment::Center),
        )
    }

    fn theme(&self) -> Theme {
        self.theme.clone()
    }

    // remove the annoying background color
    fn style(&self, theme: &Self::Theme) -> iced_layershell::Appearance {
        let palette = theme.extended_palette();
        iced_layershell::Appearance {
            background_color: iced::Color::TRANSPARENT,
            text_color: palette.background.base.text,
        }
    }

    fn scale_factor(&self) -> f64 {
        1.0
    }
}

fn testing_box_2<'a>(state: &Counter) -> Column<'a, Message> {
    column![
        button("Increment", ButtonVariant::Primary).on_press(Message::Increment(10)),
        text(state.value).size(50),
        button("Decrement", ButtonVariant::Secondary).on_press(Message::Decrement(20)),
    ]
    .padding(20)
    .max_width(500)
    .align_x(Alignment::Center)
}

fn testing_box<'a>(state: &Counter) -> Column<'a, Message> {
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
    .max_width(500)
    .align_x(Alignment::Center)
}
