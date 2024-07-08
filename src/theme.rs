use std::{fs::File, io::Read};

use iced::{
    color,
    theme::{
        palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success},
        Palette,
    },
    Theme,
};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub fn get_all_themes() -> Vec<Theme> {
    let current = get_theme().clone();
    let all = [
        Theme::ALL,
        &[Theme::custom_with_fn(
            TOKYO_NIGHT_DARK_NAME.into(),
            TOKYO_NIGHT_DARK,
            |_: Palette| tokyo_generate(TOKYO_NIGHT_DARK),
        )],
    ]
    .concat();
    if all.contains(&current) {
        all
    } else {
        [
            Theme::ALL,
            &[current],
            &[Theme::custom_with_fn(
                TOKYO_NIGHT_DARK_NAME.into(),
                TOKYO_NIGHT_DARK,
                |_: Palette| tokyo_generate(TOKYO_NIGHT_DARK),
            )],
        ]
        .concat()
    }
}

const TOKYO_NIGHT_DARK_NAME: &str = "Tokyo Night Dark";

pub const TOKYO_NIGHT_DARK: Palette = Palette {
    background: color!(0x1a1b26), // Background (Night)
    text: color!(0xC0CAF5),       // Text
    primary: color!(0x1a1b26),    // Background (Night)
    success: color!(0x9ece6a),    // Green
    danger: color!(0xf7768e),     // Red
};

#[derive(Deserialize)]
pub struct TomlSimple {
    name: String,
    background: u32,
    text: u32,
    primary: u32,
    success: u32,
    danger: u32,
}
impl From<TomlSimple> for Palette {
    fn from(val: TomlSimple) -> Self {
        Palette {
            background: color!(val.background),
            text: color!(val.text),
            primary: color!(val.primary),
            success: color!(val.success),
            danger: color!(val.danger),
        }
    }
}

#[derive(Deserialize)]
pub struct TomlExtended {
    name: String,
    is_dark: bool,
    background: TomlPalette,
    primary: TomlPalette,
    secondary: TomlPalette,
    success: TomlPalette,
    danger: TomlPalette,
}

impl From<TomlExtended> for Extended {
    fn from(val: TomlExtended) -> Self {
        Extended {
            background: Background {
                base: Pair {
                    color: color!(val.background.base.color),
                    text: color!(val.background.base.text),
                },
                weak: Pair {
                    color: color!(val.background.weak.color),
                    text: color!(val.background.weak.text),
                },
                strong: Pair {
                    color: color!(val.background.strong.color),
                    text: color!(val.background.strong.text),
                },
            },
            primary: Primary {
                base: Pair {
                    color: color!(val.primary.base.color),
                    text: color!(val.primary.base.text),
                },
                weak: Pair {
                    color: color!(val.primary.weak.color),
                    text: color!(val.primary.weak.text),
                },
                strong: Pair {
                    color: color!(val.primary.strong.color),
                    text: color!(val.primary.strong.text),
                },
            },
            secondary: Secondary {
                base: Pair {
                    color: color!(val.secondary.base.color),
                    text: color!(val.secondary.base.text),
                },
                weak: Pair {
                    color: color!(val.secondary.weak.color),
                    text: color!(val.secondary.weak.text),
                },
                strong: Pair {
                    color: color!(val.secondary.strong.color),
                    text: color!(val.secondary.strong.text),
                },
            },
            success: Success {
                base: Pair {
                    color: color!(val.success.base.color),
                    text: color!(val.success.base.text),
                },
                weak: Pair {
                    color: color!(val.success.weak.color),
                    text: color!(val.success.weak.text),
                },
                strong: Pair {
                    color: color!(val.success.strong.color),
                    text: color!(val.success.strong.text),
                },
            },
            danger: Danger {
                base: Pair {
                    color: color!(val.danger.base.color),
                    text: color!(val.danger.base.text),
                },
                weak: Pair {
                    color: color!(val.danger.weak.color),
                    text: color!(val.danger.weak.text),
                },
                strong: Pair {
                    color: color!(val.danger.strong.color),
                    text: color!(val.danger.strong.text),
                },
            },
            is_dark: val.is_dark,
        }
    }
}

#[derive(Deserialize)]
pub struct TomlPalette {
    base: TomlPair,
    weak: TomlPair,
    strong: TomlPair,
}

#[derive(Deserialize)]
pub struct TomlPair {
    color: u32,
    text: u32,
}

pub fn get_theme() -> Theme {
    THEME.clone()
}

static THEME: Lazy<Theme> = Lazy::new(|| {
    let theme_string = get_theme_toml();
    if theme_string.is_err() {
        return Theme::custom_with_fn(
            TOKYO_NIGHT_DARK_NAME.into(),
            TOKYO_NIGHT_DARK,
            tokyo_generate,
        );
    }
    let theme_string = theme_string.unwrap();
    let theme = parse_simple_palette(&theme_string);
    if let Some(theme) = theme {
        return theme;
    }
    parse_extended_palette(&theme_string)
});

fn get_theme_toml() -> std::io::Result<String> {
    let config = xdg::BaseDirectories::with_prefix("oxiced")?;
    let theme_path = config.find_config_file("theme.toml");
    if theme_path.is_none() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "file not found",
        ));
    }
    let theme_path = theme_path.unwrap();
    let mut theme_file = File::open(theme_path)?;
    let mut theme_string = String::from("");
    theme_file.read_to_string(&mut theme_string)?;
    Ok(theme_string)
}

fn parse_simple_palette(theme_string: &str) -> Option<Theme> {
    let parsed_theme: Result<TomlSimple, _> = toml::from_str(theme_string);
    if parsed_theme.is_err() {
        return None;
    }
    let parsed_theme = parsed_theme.unwrap();
    Some(Theme::custom(
        parsed_theme.name.clone(),
        parsed_theme.into(),
    ))
}

fn parse_extended_palette(theme_string: &str) -> Theme {
    let parsed_theme: Result<TomlExtended, _> = toml::from_str(theme_string);
    if let Err(error) = parsed_theme {
        println!("Could not parse theme file: {}", error);
        return Theme::custom_with_fn(
            TOKYO_NIGHT_DARK_NAME.into(),
            TOKYO_NIGHT_DARK,
            tokyo_generate,
        );
    }
    let parsed_theme = parsed_theme.unwrap();
    Theme::custom_with_fn(parsed_theme.name.clone(), TOKYO_NIGHT_DARK, |_: Palette| {
        parsed_theme.into()
    })
}

fn tokyo_generate(palette: Palette) -> Extended {
    Extended {
        background: Background {
            base: Pair {
                color: palette.background, // base background
                text: palette.text,
            },
            weak: Pair {
                color: color!(0x222430), // used for dropdowns etc
                text: palette.text,
            },
            strong: Pair {
                color: palette.background, // used for not hovered borders
                text: palette.text,
            },
        },
        primary: Primary {
            base: Pair {
                color: color!(0x282A38), // used for buttons default and hovered dropdowns
                text: palette.text,
            },
            weak: Pair {
                color: palette.background, // no idea
                text: palette.text,
            },
            strong: Pair {
                // ???????????????????????????????/
                color: color!(0x3E4052), // used for buttons when hovered
                text: palette.text,
            },
        },
        secondary: Secondary {
            base: Pair {
                color: palette.background, // no idea
                text: palette.text,
            },
            weak: Pair {
                color: palette.background, // no idea
                text: palette.text,
            },
            strong: Pair {
                color: palette.background, // no idea
                text: palette.text,
            },
        },
        success: Success {
            base: Pair {
                color: palette.background,
                text: palette.text,
            },
            weak: Pair {
                color: palette.background,
                text: palette.text,
            },
            strong: Pair {
                color: palette.background,
                text: palette.text,
            },
        },
        danger: Danger {
            base: Pair {
                color: palette.background,
                text: palette.text,
            },
            weak: Pair {
                color: palette.background,
                text: palette.text,
            },
            strong: Pair {
                color: palette.background,
                text: palette.text,
            },
        },
        is_dark: true,
    }
}
