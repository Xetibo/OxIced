use iced::{Color, Theme, theme::Palette};
use once_cell::sync::Lazy;
use optional_struct::{Applicable, optional_struct};
use serde::Deserialize;

use crate::utils::{
    color::{mk_color, mk_dark_color, mk_light_color},
    file::get_theme_toml,
};

pub const OXITHEME: Lazy<ComputedOxiTheme> = Lazy::new(|| {
    let default_theme = default_theme();
    let theme_str_opt = get_theme_toml();
    if let Err(_) = theme_str_opt {
        return ComputedOxiTheme::from(default_theme);
    };
    let theme_opt = toml::from_str::<OptionalOxiTheme>(&theme_str_opt.unwrap());
    let theme = if let Ok(theme) = theme_opt {
        theme.build(default_theme)
    } else {
        default_theme
    };
    ComputedOxiTheme::from(theme)
});

pub fn get_derived_iced_theme() -> Theme {
    let theme = OXITHEME;
    let palette = Palette {
        background: theme.base,
        text: theme.text,
        primary: theme.primary,
        success: theme.good,
        danger: theme.bad,
    };
    Theme::custom(String::from("OxiTheme"), palette)
}

fn default_theme() -> OxiTheme {
    OxiTheme {
        base: String::from("1e1e2e"),
        mantle: String::from("181825"),
        primary_bg: String::from("313244"),
        secondary_bg: String::from("45475a"),
        tertiary_bg: String::from("585b70"),
        text: String::from("cdd6f4"),
        text_muted: String::from("585b70"),
        primary: String::from("89b4fa"),
        secondary: String::from("b4befe"),
        primary_contrast: String::from("ffffff"),
        secondary_contrast: String::from("ffffff"),
        good: String::from("a6e3a1"),
        good_contrast: String::from("000000"),
        bad: String::from("f38ba8"),
        bad_contrast: String::from("ffffff"),
        info: String::from("94e2d5"),
        info_contrast: String::from("ffffff"),
        warning: String::from("f9e2af"),
        warning_contrast: String::from("ffffff"),
        rose: String::from("f5e0dc"),
        lavender: String::from("b4befe"),
        blue: String::from("89b4fa"),
        mauve: String::from("cba6f7"),
        flamingo: String::from("f2cdcd"),
        tint_amount: 0.04,
        shade_amount: 0.08,
        border_radius: 10,
        border_color_weak: String::from("cdd6f4"),
        border_color_strong: String::from("89b4fa"),
        padding_xs: 4.0,
        padding_sm: 8.0,
        padding_md: 12.0,
        padding_lg: 16.0,
        padding_xl: 24.0,
        padding_xxl: 32.0,
        font_sm: 10.0,
        font_md: 14.0,
        font_lg: 18.0,
        font_xl: 24.0,
        font_xxl: 32.0,
    }
}

// TODO shadow color
#[optional_struct]
#[derive(Deserialize)]
#[allow(dead_code)]
pub struct OxiTheme {
    /// TODO beforepr
    /// Base16: base00 -> base
    pub base: String,
    /// Background of an application
    /// Base16: base01 -> mantle
    pub mantle: String,
    /// Background for sidebars and cards
    /// Base16: base02 -> surface0
    pub primary_bg: String,
    /// Background for selectors/pickers
    /// Base16: base02 -> surface1
    pub secondary_bg: String,
    /// Background for card in cards
    /// Base16: base03 -> surface2
    pub tertiary_bg: String,
    /// General text
    /// Base16: base04 -> text
    pub text: String,
    /// Muted text
    /// Base16: base03 -> surface2
    pub text_muted: String,

    // Base 16 ?? where do we take primary??
    /// Primary theme color
    /// Buttons etc
    pub primary: String,
    /// Secondary theme color
    /// Buttons etc
    pub secondary: String,
    /// Contrast text to primary
    pub primary_contrast: String,
    /// Contrast text to secondary
    pub secondary_contrast: String,

    /// Success/green
    /// Base16: base0B -> green
    pub good: String,
    pub good_contrast: String,
    /// Error/red
    /// Base16: base08 -> red
    pub bad: String,
    pub bad_contrast: String,
    /// Information/teal
    /// Base16: base0C -> teal
    pub info: String,
    pub info_contrast: String,
    /// Warning/yellow
    /// Base16: base0A -> yellow
    pub warning: String,
    pub warning_contrast: String,

    pub rose: String,
    pub lavender: String,
    pub blue: String,
    pub mauve: String,
    pub flamingo: String,

    pub shade_amount: f32,
    pub tint_amount: f32,

    pub border_radius: u16,
    /// Borders for Selectors and similar
    pub border_color_weak: String,
    /// Borders for hard cuts like sidebars
    pub border_color_strong: String,

    pub padding_xs: f32,
    pub padding_sm: f32,
    pub padding_md: f32,
    pub padding_lg: f32,
    pub padding_xl: f32,
    pub padding_xxl: f32,

    pub font_sm: f32,
    pub font_md: f32,
    pub font_lg: f32,
    pub font_xl: f32,
    pub font_xxl: f32,
}

impl From<OxiTheme> for ComputedOxiTheme {
    fn from(value: OxiTheme) -> Self {
        Self {
            base: mk_color(&value.base),
            mantle: mk_color(&value.mantle),
            mantle_hover: mk_light_color(&value.mantle, value.tint_amount),
            mantle_active: mk_light_color(&value.mantle, value.shade_amount),
            primary_bg: mk_color(&value.primary_bg),
            primary_bg_hover: mk_light_color(&value.primary_bg, value.tint_amount),
            primary_bg_active: mk_light_color(&value.primary_bg, value.shade_amount),
            secondary_bg: mk_color(&value.secondary_bg),
            secondary_bg_hover: mk_light_color(&value.secondary_bg, value.tint_amount),
            secondary_bg_active: mk_light_color(&value.secondary_bg, value.shade_amount),
            tertiary_bg: mk_color(&value.tertiary_bg),
            tertiary_bg_hover: mk_light_color(&value.tertiary_bg, value.tint_amount),
            tertiary_bg_active: mk_light_color(&value.tertiary_bg, value.shade_amount),
            text: mk_color(&value.text),
            text_muted: mk_color(&value.text_muted),
            primary: mk_color(&value.primary),
            primary_hover: mk_dark_color(&value.primary, value.tint_amount),
            primary_active: mk_dark_color(&value.primary, value.shade_amount),
            secondary: mk_color(&value.secondary),
            secondary_hover: mk_dark_color(&value.secondary, value.tint_amount),
            secondary_active: mk_dark_color(&value.secondary, value.shade_amount),
            primary_contrast: mk_color(&value.primary_contrast),
            secondary_contrast: mk_color(&value.secondary_contrast),
            good: mk_color(&value.good),
            good_hover: mk_dark_color(&value.good, value.tint_amount),
            good_active: mk_dark_color(&value.good, value.tint_amount),
            good_contrast: mk_color(&value.good_contrast),
            bad: mk_color(&value.bad),
            bad_hover: mk_dark_color(&value.bad, value.tint_amount),
            bad_active: mk_dark_color(&value.bad, value.tint_amount),
            bad_contrast: mk_color(&value.bad_contrast),
            info: mk_color(&value.info),
            info_hover: mk_dark_color(&value.info, value.tint_amount),
            info_active: mk_dark_color(&value.info, value.tint_amount),
            info_contrast: mk_color(&value.info_contrast),
            warning: mk_color(&value.warning),
            warning_hover: mk_dark_color(&value.warning, value.tint_amount),
            warning_active: mk_dark_color(&value.warning, value.tint_amount),
            warning_contrast: mk_color(&value.warning_contrast),
            rose: mk_color(&value.rose),
            lavender: mk_color(&value.lavender),
            blue: mk_color(&value.blue),
            mauve: mk_color(&value.mauve),
            flamingo: mk_color(&value.flamingo),
            shade_amount: value.shade_amount,
            tint_amount: value.tint_amount,
            border_radius: value.border_radius,
            border_color_weak: mk_color(&value.border_color_weak),
            border_color_strong: mk_color(&value.border_color_strong),
            padding_xs: value.padding_xs,
            padding_sm: value.padding_sm,
            padding_md: value.padding_md,
            padding_lg: value.padding_lg,
            padding_xl: value.padding_xl,
            padding_xxl: value.padding_xxl,
            font_sm: value.font_sm,
            font_md: value.font_md,
            font_lg: value.font_lg,
            font_xl: value.font_xl,
            font_xxl: value.font_xxl,
        }
    }
}

#[allow(dead_code)]
pub struct ComputedOxiTheme {
    pub base: Color,

    pub mantle: Color,
    pub mantle_hover: Color,
    pub mantle_active: Color,

    pub primary_bg: Color,
    pub primary_bg_hover: Color,
    pub primary_bg_active: Color,

    pub secondary_bg: Color,
    pub secondary_bg_hover: Color,
    pub secondary_bg_active: Color,

    pub tertiary_bg: Color,
    pub tertiary_bg_hover: Color,
    pub tertiary_bg_active: Color,

    pub text: Color,
    pub text_muted: Color,

    pub primary: Color,
    pub primary_hover: Color,
    pub primary_active: Color,
    pub secondary: Color,
    pub secondary_hover: Color,
    pub secondary_active: Color,

    pub primary_contrast: Color,
    pub secondary_contrast: Color,

    pub good: Color,
    pub good_hover: Color,
    pub good_active: Color,
    pub good_contrast: Color,

    pub bad: Color,
    pub bad_hover: Color,
    pub bad_active: Color,
    pub bad_contrast: Color,

    pub info: Color,
    pub info_hover: Color,
    pub info_active: Color,
    pub info_contrast: Color,

    pub warning: Color,
    pub warning_hover: Color,
    pub warning_active: Color,
    pub warning_contrast: Color,

    pub rose: Color,
    pub lavender: Color,
    pub blue: Color,
    pub mauve: Color,
    pub flamingo: Color,

    pub shade_amount: f32,
    pub tint_amount: f32,

    pub border_radius: u16,
    pub border_color_weak: Color,
    pub border_color_strong: Color,

    pub padding_xs: f32,
    pub padding_sm: f32,
    pub padding_md: f32,
    pub padding_lg: f32,
    pub padding_xl: f32,
    pub padding_xxl: f32,

    pub font_sm: f32,
    pub font_md: f32,
    pub font_lg: f32,
    pub font_xl: f32,
    pub font_xxl: f32,
}
