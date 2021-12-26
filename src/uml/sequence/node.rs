use crate::helper::*;
use svg::node::element::Text;

#[derive(PartialEq, Eq, Hash)]
pub struct Node {
  pub name: String,
}

impl Node {
  pub fn make_svg(&self) -> Text {
    make_text(&self.name)
  }
}
