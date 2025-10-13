use std::{env, path::PathBuf};

use iced::{
    Color, Element,
    Length::Fill,
    Point, Rectangle, Renderer, Size, Task, mouse,
    theme::palette::Extended,
    widget::canvas::{self, Frame, Geometry},
};
use oxiced::{
    theme::legacy_theme::{ExtendedHex, TomlBase16, parse_extended_palette},
    utils::file::open_file,
};

pub fn palette() -> Result<(), iced::Error> {
    iced::application(ThemeDisplay::new, ThemeDisplay::update, ThemeDisplay::view).run()
}

struct ThemeDisplay {
    theme: Extended,
    hex: ExtendedHex<'static>,
    canvas_cache: canvas::Cache,
}

#[derive(Debug)]
enum Message {}

fn colorname(name: &'static str, hex: &String, color: Color) -> (String, Color) {
    (format!("{name}: #{hex}"), color)
}

impl ThemeDisplay {
    fn new() -> (Self, Task<Message>) {
        let args: Vec<String> = env::args().collect();
        if args.len() < 3 {
            panic!("Enter a path to a toml");
        }
        let theme_path = args[2].clone();
        let file_res = open_file(PathBuf::from(theme_path));
        if let Err(error) = file_res {
            panic!("{}", error);
        }
        let file = file_res.unwrap();
        let theme_opt = parse_extended_palette::<TomlBase16>(&file);
        if let Err(error) = theme_opt {
            panic!("{}", error);
        }
        let theme = theme_opt.unwrap();
        println!("Parsed the following theme: {}", &theme);
        let palette = *theme.extended_palette();

        let raw_theme_opt: Result<TomlBase16, _> = toml::from_str(&file);
        if let Err(error) = raw_theme_opt {
            panic!("{}", error);
        }
        let static_theme: &'static TomlBase16 = Box::leak(Box::new(raw_theme_opt.unwrap()));
        let hex = ExtendedHex::<'static>::from(static_theme);
        (
            Self {
                theme: palette,
                hex,
                canvas_cache: canvas::Cache::default(),
            },
            Task::none(),
        )
    }

    fn update(&mut self, _: Message) -> Task<Message> {
        Task::none()
    }

    fn draw(&self, frame: &mut Frame) {
        // 3 + 3 + 3 + 3

        let colors = vec![
            colorname(
                "Background weak",
                self.hex.background_weak_color,
                self.theme.background.weak.color,
            ),
            colorname(
                "Background weak text",
                self.hex.background_weak_text,
                self.theme.background.weak.text,
            ),
            colorname(
                "Background base",
                self.hex.background_base_color,
                self.theme.background.base.color,
            ),
            colorname(
                "Background base text",
                self.hex.background_base_text,
                self.theme.background.base.text,
            ),
            colorname(
                "Background strong",
                self.hex.background_strong_color,
                self.theme.background.strong.color,
            ),
            colorname(
                "Background strong text",
                self.hex.background_strong_text,
                self.theme.background.strong.text,
            ),
            colorname(
                "Primary weak",
                self.hex.primary_weak_color,
                self.theme.primary.weak.color,
            ),
            colorname(
                "primary weak text",
                self.hex.primary_weak_text,
                self.theme.primary.weak.text,
            ),
            colorname(
                "Primary base",
                self.hex.primary_base_color,
                self.theme.primary.base.color,
            ),
            colorname(
                "primary base text",
                self.hex.primary_base_text,
                self.theme.primary.base.text,
            ),
            colorname(
                "Primary strong",
                self.hex.primary_strong_color,
                self.theme.primary.strong.color,
            ),
            colorname(
                "primary strong text",
                self.hex.primary_strong_text,
                self.theme.primary.strong.text,
            ),
            colorname(
                "Secondary weak",
                self.hex.secondary_weak_color,
                self.theme.secondary.weak.color,
            ),
            colorname(
                "secondary weak text",
                self.hex.secondary_weak_text,
                self.theme.secondary.weak.text,
            ),
            colorname(
                "Secondary base",
                self.hex.secondary_base_color,
                self.theme.secondary.base.color,
            ),
            colorname(
                "secondary base text",
                self.hex.secondary_base_text,
                self.theme.secondary.base.text,
            ),
            colorname(
                "Secondary strong",
                self.hex.secondary_strong_color,
                self.theme.secondary.strong.color,
            ),
            colorname(
                "secondary strong text",
                self.hex.secondary_strong_text,
                self.theme.secondary.strong.text,
            ),
            colorname(
                "Success weak",
                self.hex.success_weak_color,
                self.theme.success.weak.color,
            ),
            colorname(
                "success weak text",
                self.hex.success_weak_text,
                self.theme.success.weak.text,
            ),
            colorname(
                "Success base",
                self.hex.success_base_color,
                self.theme.success.base.color,
            ),
            colorname(
                "success base text",
                self.hex.success_base_text,
                self.theme.success.base.text,
            ),
            colorname(
                "Success strong",
                self.hex.success_strong_color,
                self.theme.success.strong.color,
            ),
            colorname(
                "success strong text",
                self.hex.success_strong_text,
                self.theme.success.strong.text,
            ),
            colorname(
                "Danger weak",
                self.hex.danger_weak_color,
                self.theme.danger.weak.color,
            ),
            colorname(
                "Danger weak text",
                self.hex.danger_weak_text,
                self.theme.danger.weak.text,
            ),
            colorname(
                "Danger base",
                self.hex.danger_base_color,
                self.theme.danger.base.color,
            ),
            colorname(
                "Danger base text",
                self.hex.danger_base_text,
                self.theme.danger.base.text,
            ),
            colorname(
                "Danger strong",
                self.hex.danger_strong_color,
                self.theme.danger.strong.color,
            ),
            colorname(
                "Danger strong text",
                self.hex.danger_strong_text,
                self.theme.danger.strong.text,
            ),
        ];

        let mut col: i32 = 0;
        let count = 15;
        for (i, (name, color)) in colors.iter().enumerate() {
            if i as i32 / ((col + 1) * count) >= 1 {
                col += 1;
            }
            frame.fill_text(canvas::Text {
                content: name.clone(),
                position: Point {
                    x: 50.0 + col as f32 * 600.0,
                    y: ((i as i32 + 1 - (col * count)) as f32 * 50.0) + 25.0,
                },
                // vertical_alignment: iced::alignment::Vertical::Center,
                ..canvas::Text::default()
            });
            frame.fill_rectangle(
                Point {
                    x: 350.0 + col as f32 * 600.0,
                    y: (i as i32 + 1 - (col * count)) as f32 * 50.0,
                },
                Size {
                    width: 50.0,
                    height: 50.0,
                },
                *color,
            );
        }
    }

    fn view(&self) -> Element<Message> {
        iced::widget::Canvas::new(self)
            .height(Fill)
            .width(Fill)
            .into()
    }
}

impl<Message> canvas::Program<Message> for ThemeDisplay {
    type State = ();

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _theme: &iced::Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry> {
        let theme = self.canvas_cache.draw(renderer, bounds.size(), |frame| {
            self.draw(frame);
        });

        vec![theme]
    }
}
