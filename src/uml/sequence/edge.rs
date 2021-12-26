use super::*;
use crate::helper::*;
use svg::node::element::Text;

pub struct Edge(usize, usize, String, Markers);

impl Edge {
  pub fn new(x: usize, y: usize, s: String, m: Markers) -> Self {
    Edge(x, y, s, m)
  }

  #[inline]
  pub fn node1(&self) -> usize {
    self.0
  }

  #[inline]
  pub fn node2(&self) -> usize {
    self.1
  }

  #[inline]
  pub fn position_node1(&self, max_length: usize) -> (usize, usize) {
    position(self.0, max_length)
  }

  #[inline]
  pub fn position_node2(&self, max_length: usize) -> (usize, usize) {
    position(self.1, max_length)
  }

  #[inline]
  pub fn marker_type(&self) -> Markers {
    self.3.clone()
  }

  pub fn make_text(&self) -> Text {
    make_text(&self.2)
  }
}

#[test]
fn make_edge() {
  let edge = Edge::new(0, 1, "".into(), Markers::None);
  assert_eq!(edge.node1(), 0);
  assert_eq!(edge.node2(), 1);
  assert_eq!(edge.marker_type(), Markers::None);
}
