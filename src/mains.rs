use std::collections::HashMap;

use iced::advanced::subscription;
use iced::keyboard::key::Named;
use iced::widget::container::Style;
use iced::widget::{column, container, rich_text, row, scrollable, span, text, Column, Row};
use iced::{event, futures, window, Alignment, Element, Subscription, Task, Theme};
use theme::get_theme;
use widgets::oxi_button::{button, ButtonVariant};
use widgets::oxi_text_input::text_input;

use iced_layershell::actions::LayershellCustomActions;
use iced_layershell::reexport::{Anchor, Layer};
use iced_layershell::settings::{LayerShellSettings, Settings};
use iced_layershell::Application;
use zbus::{proxy, Connection};

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
    theme: Theme,
    text: String,
    clipboard_content: HashMap<i32, String>,
}

impl Default for Counter {
    fn default() -> Self {
        let data = futures::executor::block_on(get_items());
        let map = if let Ok(map) = data {
            map
        } else {
            HashMap::new()
        };
        Self {
            theme: get_theme(),
            text: "".into(),
            clipboard_content: map,
            // TODO handle err
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
    //Slider(i64),
    Copy(i32),
    Remove(i32),
    ClearClipboard,
    AddTestElement(String),
    Exit,
    //TextChanged(String),
    //Check(),
    //kToggle(bool),
    //Theme(Theme),
    //TextInput(String),
    //Direction(WindowDirection),
    //SizeChange((u32, u32)),
    //IcedEvent(Event),
    Empty(String),
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
        event::listen_with(|event, _status, _id| match event {
            iced::Event::Keyboard(event) => match event {
                iced::keyboard::Event::KeyPressed {
                    modifiers,
                    key: iced::keyboard::key::Key::Named(Named::Escape),
                    modified_key,
                    physical_key,
                    location,
                    text,
                } => Some(Message::Exit),
                _ => None,
            },
            _ => None,
        })
        //Subscription::none()
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            //Message::Slider(val) => {
            //    self.slider_value = val;
            //}
            //Message::Increment(val) => {
            //    self.value += val;
            //}
            //Message::Decrement(val) => {
            //    self.value -= val;
            //}
            //Message::Theme(theme) => self.theme = theme,
            //Message::Check() => {
            //    self.is_checked = !self.is_checked;
            //}
            //Message::Toggle(val) => {
            //    self.is_toggled = val;
            //    self.text += "1";
            //}
            //Message::TextChanged(val) => self.text = val,
            // TODO
            Message::AddTestElement(value) => {
                self.clipboard_content
                    .insert(self.clipboard_content.len() as i32, value);
                Task::none()
            }
            Message::Copy(value) => {
                let _ = futures::executor::block_on(copy_to_clipboard(value as u32));
                // TODO make this work with iced exit?
                std::process::exit(0);
            }
            //let res = self.clipboard.set().text(value);
            //if res.is_err() {
            //    println!("got error lul {:#?}", res.err());
            //}
            Message::Empty(value) => {
                self.text = value;
                Task::none()
            }
            Message::Remove(index) => {
                self.clipboard_content.remove(&index);
                Task::none()
            }
            Message::ClearClipboard => {
                let _ = futures::executor::block_on(delete_all());
                // TODO make this work with iced exit?
                std::process::exit(0);
            }
            Message::Exit => {
                // TODO make this work with iced exit?
                std::process::exit(0);
            }
            _ => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        wrap_in_rounded_box(
            window(self),
            //pick_list(get_all_themes(), Some(&self.theme), Message::Theme).width(Length::Fill),
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

fn clipboard_element<'a>(index: i32, contents: &str) -> Row<'a, Message> {
    row![
        button(
            rich_text!(span(contents.to_owned())),
            ButtonVariant::Primary
        )
        .padding([20, 5])
        .width(iced::Length::Fill)
        .on_press(Message::Copy(index)),
        button("X", ButtonVariant::Primary)
            .on_press(Message::Remove(index))
            .padding([20, 5]),
    ]
    .padding(20)
    .spacing(20)
}

fn window<'a>(state: &Counter) -> Column<'a, Message> {
    let elements: Vec<Row<'_, Message>> = state
        .clipboard_content
        .iter()
        .map(|(key, value)| clipboard_element(*key, value))
        .collect();

    let mut elements_col = column![];
    for element in elements {
        elements_col = elements_col.push_maybe(Some(element));
    }
    let elements_scrollable = scrollable(elements_col);

    column![
        text_input("something", state.text.as_str(), Message::Empty),
        row![
            button("AddTestElement", ButtonVariant::Primary)
                .on_press(Message::AddTestElement("henlo".into())),
            button("Clear all", ButtonVariant::Primary).on_press(Message::ClearClipboard)
        ],
        elements_scrollable
    ]
    .padding(10)
    .align_x(Alignment::Center)
}

#[proxy(
    interface = "org.Xetibo.OxiPasteDaemon",
    default_service = "org.Xetibo.OxiPasteDaemon",
    default_path = "/org/Xetibo/OxiPasteDaemon"
)]
trait OxiPasteDbus {
    async fn GetAll(&self) -> zbus::Result<Vec<(Vec<u8>, String)>>;
    async fn Paste(&self, index: u32) -> zbus::Result<()>;
    async fn DeleteAll(&self) -> zbus::Result<()>;
}

async fn get_items() -> zbus::Result<HashMap<i32, String>> {
    let connection = Connection::session().await?;
    let proxy = OxiPasteDbusProxy::new(&connection).await?;
    let reply = proxy.GetAll().await?;

    let mut map = HashMap::new();
    for item in reply {
        if item.1.contains("text") {
            map.insert(map.len() as i32, String::from_utf8(item.0).unwrap());
        }
    }
    Ok(map)
}

async fn copy_to_clipboard(index: u32) -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = OxiPasteDbusProxy::new(&connection).await?;
    proxy.Paste(index).await?;
    Ok(())
}

async fn delete_all() -> zbus::Result<()> {
    let connection = Connection::session().await?;
    let proxy = OxiPasteDbusProxy::new(&connection).await?;
    proxy.DeleteAll().await?;
    Ok(())
}

//fn testing_box<'a>(state: &Counter) -> Column<'a, Message> {
//    column![
//        button("Increment", ButtonVariant::Primary).on_press(Message::Increment(10)),
//        text(state.value).size(50),
//        button("Decrement", ButtonVariant::Secondary).on_press(Message::Decrement(20)),
//        button("success", ButtonVariant::Success).on_press(Message::Increment(10)),
//        button("danger", ButtonVariant::Danger).on_press(Message::Increment(10)),
//        checkbox("what", state.is_checked, |_| { Message::Check() }),
//        radio("first", 10, None, Message::Increment),
//        radio("second", 20, None, Message::Increment),
//        slider(0..=100, state.slider_value as i32, |val| Message::Slider(
//            val as i64
//        )),
//        text_input("something", state.text.as_str(), Message::TextChanged),
//        toggler(Some(state.text.clone()), state.is_toggled, Message::Toggle),
//        progress_bar(0.0..=100.0, state.slider_value as f32),
//        horizontal_rule(100),
//        vertical_rule(100),
//    ]
//    .padding(20)
//    .max_width(500)
//    .align_x(Alignment::Center)
//}
