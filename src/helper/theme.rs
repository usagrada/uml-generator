use crate::theme::Theme;
use svg::node::element::{Circle, Ellipse, Rectangle};

pub trait SetTheme {
    fn set_theme(self, theme: &Theme) -> Self;
}

impl SetTheme for Rectangle {
    fn set_theme(self, theme: &Theme) -> Self {
        self.set("fill", theme.color.rect.fill)
            .set("stroke", theme.color.rect.frame)
            .set("stroke-width", 1)
    }
}

impl SetTheme for Circle {
    fn set_theme(self, theme: &Theme) -> Self {
        self.set("fill", theme.color.rect.fill)
            .set("stroke", theme.color.rect.frame)
            .set("stroke-width", 1)
    }
}

impl SetTheme for Ellipse {
    fn set_theme(self, theme: &Theme) -> Self {
        self.set("fill", theme.color.rect.fill)
            .set("stroke", theme.color.rect.frame)
            .set("stroke-width", 1)
    }
}
