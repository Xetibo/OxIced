use iced::theme::palette::{Background, Danger, Pair, Primary, Secondary, Success};

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
