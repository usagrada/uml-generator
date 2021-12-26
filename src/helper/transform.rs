use svg::node::element::Group;
pub trait Transform {
  fn transform(self, x: usize, y: usize) -> Self;
}

impl Transform for Group {
  fn transform(self, x: usize, y: usize) -> Self {
    self.set("transform", format!("translate({}, {})", x, y))
  }
}
