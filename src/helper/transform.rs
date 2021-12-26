use svg::node::element::{Circle, Ellipse, Group, Rectangle, Text};
pub trait Transform {
  fn transform(self, x: usize, y: usize) -> Self;
}

impl Transform for Group {
  fn transform(self, x: usize, y: usize) -> Self {
    self.set("transform", format!("translate({}, {})", x, y))
  }
}

pub trait Position {
  fn position(self, x: usize, y: usize) -> Self;
}

impl Position for Rectangle {
  fn position(self, x: usize, y: usize) -> Self {
    self.set("x", x).set("y", y)
  }
}

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

impl Position for Text {
  fn position(self, x: usize, y: usize) -> Self {
    self.set("x", x).set("y", y)
  }
}
