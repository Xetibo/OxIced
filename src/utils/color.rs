use iced::{Color, color};

pub fn mk_color(color: &str) -> Color {
    color!(u32::from_str_radix(color, 16).unwrap_or(0) as f32)
}

pub fn mk_dark_color(color: &str, amount: f32) -> Color {
    darken_color(&mk_color(color), amount)
}

pub fn mk_light_color(color: &str, amount: f32) -> Color {
    lighten_color(&mk_color(color), amount)
}

pub fn lighten_color(color: &Color, amount: f32) -> Color {
    Color {
        r: f32::clamp(color.r + amount, 0.0, 1.0),
        g: f32::clamp(color.g + amount, 0.0, 1.0),
        b: f32::clamp(color.b + amount, 0.0, 1.0),
        ..*color
    }
}

pub fn darken_color(color: &Color, amount: f32) -> Color {
    Color {
        r: f32::clamp(color.r - amount, 0.0, 1.0),
        g: f32::clamp(color.g - amount, 0.0, 1.0),
        b: f32::clamp(color.b - amount, 0.0, 1.0),
        ..*color
    }
}
