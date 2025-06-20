use iced::{
    Theme, color,
    theme::{
        Palette,
        palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success},
    },
};
use once_cell::sync::Lazy;
use serde::{Deserialize, de::DeserializeOwned};

use crate::{
    theme::theme::OXITHEME,
    utils::{
        color::{darken_color, lighten_color},
        file::get_theme_toml,
    },
};

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

pub trait TomlTheme {
    fn name(&self) -> String;
}

//pub struct Base16 {
//    name: String,
//    base00: Color, //base
//    base01: Color, //mantle
//    base02: Color, //surface0
//    base03: Color, //surface1
//    base04: Color, //surface2
//    base05: Color, //text
//    base06: Color, //rosewater
//    base07: Color, //lavender
//    base08: Color, //red
//    base09: Color, //peach
//    base0a: Color, //yellow
//    base0b: Color, //green
//    base0c: Color, //teal
//    base0d: Color, //blue
//    base0e: Color, //mauve
//    base0f: Color, //flamingo
//}

#[derive(Deserialize)]
#[allow(dead_code)]
pub struct TomlBase16 {
    name: String,
    base00: String, // Default Background
    base01: String, // Lighter Background (Used for status bars, line number and folding marks)
    base02: String, // Selection Background
    base03: String, // Comments, Invisibles, Line Highlighting
    base04: String, // Dark Foreground (Used for status bars)
    base05: String, // Default Foreground, Caret, Delimiters, Operators
    base06: String, // Light Foreground (Not often used)
    base07: String, // Light Background (Not often used)
    base08: String, // Variables, XML Tags, Markup Link Text, Markup Lists, Diff Deleted
    base09: String, // Integers, Boolean, Constants, XML Attributes, Markup Link Url
    base0a: String, // Classes, Markup Bold, Search Text Background
    base0b: String, // Strings, Inherited Class, Markup Code, Diff Inserted
    base0c: String, // Support, Regular Expressions, Escape Characters, Markup Quotes
    base0d: String, // Functions, Methods, Attribute IDs, Headings
    base0e: String, // Keywords, Storage, Selector, Markup Italic, Diff Changed
    base0f: String, // Deprecated, Opening/Closing Embedded Language Tags, e.g. <?php ?>
}

pub struct ExtendedHex<'a> {
    pub background_weak_color: &'a String,
    pub background_base_color: &'a String,
    pub background_strong_color: &'a String,
    pub background_weak_text: &'a String,
    pub background_base_text: &'a String,
    pub background_strong_text: &'a String,

    pub primary_weak_color: &'a String,
    pub primary_base_color: &'a String,
    pub primary_strong_color: &'a String,
    pub primary_weak_text: &'a String,
    pub primary_base_text: &'a String,
    pub primary_strong_text: &'a String,

    pub secondary_weak_color: &'a String,
    pub secondary_base_color: &'a String,
    pub secondary_strong_color: &'a String,
    pub secondary_weak_text: &'a String,
    pub secondary_base_text: &'a String,
    pub secondary_strong_text: &'a String,

    pub success_weak_color: &'a String,
    pub success_base_color: &'a String,
    pub success_strong_color: &'a String,
    pub success_weak_text: &'a String,
    pub success_base_text: &'a String,
    pub success_strong_text: &'a String,

    pub danger_weak_color: &'a String,
    pub danger_base_color: &'a String,
    pub danger_strong_color: &'a String,
    pub danger_weak_text: &'a String,
    pub danger_base_text: &'a String,
    pub danger_strong_text: &'a String,
}

impl<'a> From<&'a TomlBase16> for ExtendedHex<'a> {
    fn from(value: &'a TomlBase16) -> Self {
        Self {
            background_weak_color: &value.base00,
            background_base_color: &value.base01,
            background_strong_color: &value.base01,

            background_weak_text: &value.base05,
            background_base_text: &value.base05,
            background_strong_text: &value.base05,

            primary_weak_color: &value.base02,
            primary_base_color: &value.base03,
            primary_strong_color: &value.base04,

            primary_weak_text: &value.base05,
            primary_base_text: &value.base05,
            primary_strong_text: &value.base05,

            secondary_weak_color: &value.base02,
            secondary_base_color: &value.base03,
            secondary_strong_color: &value.base04,

            secondary_weak_text: &value.base05,
            secondary_base_text: &value.base05,
            secondary_strong_text: &value.base05,

            success_weak_color: &value.base06,
            success_base_color: &value.base07,
            success_strong_color: &value.base08,

            success_weak_text: &value.base09,
            success_base_text: &value.base0a,
            success_strong_text: &value.base0b,

            danger_weak_color: &value.base0c,
            danger_base_color: &value.base0d,
            danger_strong_color: &value.base0e,

            danger_weak_text: &value.base0f,
            // not used currently
            danger_base_text: &value.base0f,
            danger_strong_text: &value.base0f,
        }
    }
}

impl From<TomlBase16> for Extended {
    fn from(val: TomlBase16) -> Self {
        let hex = ExtendedHex::from(&val);
        Extended {
            background: Background {
                base: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.background_base_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.background_base_text, 16).unwrap_or(0) as f32
                    ),
                },
                weak: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.background_weak_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.background_weak_text, 16).unwrap_or(0) as f32
                    ),
                },
                strong: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.background_strong_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.background_strong_text, 16).unwrap_or(0) as f32
                    ),
                },
            },
            primary: Primary {
                base: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.primary_base_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.primary_base_text, 16).unwrap_or(0) as f32
                    ),
                },
                weak: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.primary_weak_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.primary_weak_text, 16).unwrap_or(0) as f32
                    ),
                },
                strong: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.primary_strong_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.primary_strong_text, 16).unwrap_or(0) as f32
                    ),
                },
            },
            secondary: Secondary {
                base: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.secondary_base_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.secondary_base_text, 16).unwrap_or(0) as f32
                    ),
                },
                weak: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.secondary_weak_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.secondary_weak_text, 16).unwrap_or(0) as f32
                    ),
                },
                strong: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.secondary_strong_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.secondary_strong_text, 16).unwrap_or(0) as f32
                    ),
                },
            },
            success: Success {
                base: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.success_base_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.success_base_text, 16).unwrap_or(0) as f32
                    ),
                },
                weak: Pair {
                    color: darken_color(
                        &color!(
                            u32::from_str_radix(&hex.success_weak_color, 16).unwrap_or(0) as f32
                        ),
                        OXITHEME.tint_amount,
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.success_weak_text, 16).unwrap_or(0) as f32
                    ),
                },
                strong: Pair {
                    color: lighten_color(
                        &color!(
                            u32::from_str_radix(&hex.success_strong_color, 16).unwrap_or(0) as f32
                        ),
                        OXITHEME.tint_amount,
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.success_strong_text, 16).unwrap_or(0) as f32
                    ),
                },
            },
            danger: Danger {
                base: Pair {
                    color: color!(
                        u32::from_str_radix(&hex.danger_base_color, 16).unwrap_or(0) as f32
                    ),
                    text: color!(u32::from_str_radix(&hex.danger_base_text, 16).unwrap_or(0) as f32),
                },
                weak: Pair {
                    color: darken_color(
                        &color!(u32::from_str_radix(&hex.danger_weak_color, 16).unwrap_or(0) as f32),
                        OXITHEME.tint_amount,
                    ),
                    text: color!(u32::from_str_radix(&hex.danger_weak_text, 16).unwrap_or(0) as f32),
                },
                strong: Pair {
                    color: lighten_color(
                        &color!(
                            u32::from_str_radix(&hex.danger_strong_color, 16).unwrap_or(0) as f32
                        ),
                        OXITHEME.tint_amount,
                    ),
                    text: color!(
                        u32::from_str_radix(&hex.danger_strong_text, 16).unwrap_or(0) as f32
                    ),
                },
            },
            is_dark: true,
        }
    }
}

impl TomlTheme for TomlBase16 {
    fn name(&self) -> String {
        self.name.clone()
    }
}

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

impl TomlTheme for TomlExtended {
    fn name(&self) -> String {
        self.name.clone()
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
    let base16_theme = parse_extended_palette::<TomlBase16>(&theme_string);
    if let Ok(theme) = base16_theme {
        return theme;
    }
    let theme = parse_simple_palette(&theme_string);
    if let Ok(theme) = theme {
        return theme;
    }
    let extended_theme = parse_extended_palette::<TomlExtended>(&theme_string);
    if let Ok(theme) = extended_theme {
        return theme;
    }
    Theme::custom_with_fn(
        TOKYO_NIGHT_DARK_NAME.into(),
        TOKYO_NIGHT_DARK,
        tokyo_generate,
    )
});

fn parse_simple_palette(theme_string: &str) -> Result<Theme, toml::de::Error> {
    let parsed_theme: TomlSimple = toml::from_str(theme_string)?;
    Ok(Theme::custom(
        parsed_theme.name.clone(),
        parsed_theme.into(),
    ))
}

pub fn parse_extended_palette<T: DeserializeOwned + TomlTheme + Into<Extended>>(
    theme_string: &str,
) -> Result<Theme, toml::de::Error> {
    let parsed_theme: T = toml::from_str(theme_string)?;
    Ok(Theme::custom_with_fn(
        parsed_theme.name(),
        TOKYO_NIGHT_DARK,
        |_: Palette| parsed_theme.into(),
    ))
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
