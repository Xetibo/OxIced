use iced::{
    theme::palette::{Background, Danger, Pair, Primary, Secondary, Success},
    Color,
};

pub trait StylingCategory {
    fn base(&self) -> Pair;
    fn weak(&self) -> Pair;
    fn strong(&self) -> Pair;
}

impl StylingCategory for Background {
    fn base(&self) -> Pair {
        self.base
    }

    fn weak(&self) -> Pair {
        self.weak
    }

    fn strong(&self) -> Pair {
        self.strong
    }
}

impl StylingCategory for Primary {
    fn base(&self) -> Pair {
        self.base
    }

    fn weak(&self) -> Pair {
        self.weak
    }

    fn strong(&self) -> Pair {
        self.strong
    }
}

impl StylingCategory for Secondary {
    fn base(&self) -> Pair {
        self.base
    }

    fn weak(&self) -> Pair {
        self.weak
    }

    fn strong(&self) -> Pair {
        self.strong
    }
}

impl StylingCategory for Success {
    fn base(&self) -> Pair {
        self.base
    }

    fn weak(&self) -> Pair {
        self.weak
    }

    fn strong(&self) -> Pair {
        self.strong
    }
}

impl StylingCategory for Danger {
    fn base(&self) -> Pair {
        self.base
    }

    fn weak(&self) -> Pair {
        self.weak
    }

    fn strong(&self) -> Pair {
        self.strong
    }
}

pub fn lighten_color(color: Color) -> Color {
    Color {
        r: f32::clamp(color.r + 0.05, 0.0, 1.0),
        g: f32::clamp(color.g + 0.05, 0.0, 1.0),
        b: f32::clamp(color.b + 0.05, 0.0, 1.0),
        ..color
    }
}

pub fn darken_color(color: Color) -> Color {
    Color {
        r: f32::clamp(color.r - 0.05, 0.0, 1.0),
        g: f32::clamp(color.g - 0.05, 0.0, 1.0),
        b: f32::clamp(color.b - 0.05, 0.0, 1.0),
        ..color
    }
}
