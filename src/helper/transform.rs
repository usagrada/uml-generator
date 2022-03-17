use svg::node::element::{Circle, Ellipse, Group, Rectangle, Text};
use svg::Node;

pub trait Transform {
    fn transform(self, x: usize, y: usize) -> Self;
}

impl Transform for Group {
    fn transform(self, x: usize, y: usize) -> Self {
        self.set("transform", format!("translate({}, {})", x, y))
    }
}

pub trait Position
where
    Self: Sized + Node,
{
    fn position(mut self, x: usize, y: usize) -> Self {
        self.assign("x", x);
        self.assign("y", y);
        self
    }
}

impl Position for Rectangle {}

impl Position for Circle {
    fn position(self, x: usize, y: usize) -> Self {
        self.set("cx", x).set("cy", y)
    }
}

impl Position for Ellipse {
    fn position(self, x: usize, y: usize) -> Self {
        self.set("cx", x).set("cy", y)
    }
}

impl Position for Text {}
