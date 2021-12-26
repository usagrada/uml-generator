use super::*;
use crate::helper::*;
use svg::node::element::Text;

pub(crate) struct Edge(usize, usize, String, Markers);

impl Edge {
  pub fn new(x: usize, y: usize, s: String, m: Markers) -> Self {
    Edge(x, y, s, m)
  }

  pub fn node1(&self) -> usize {
    self.0
  }
  pub fn node2(&self) -> usize {
    self.1
  }

  pub fn position_node1(&self, max_length: usize) -> (usize, usize) {
    position(self.0, max_length)
  }
  
  pub fn position_node2(&self, max_length: usize) -> (usize, usize) {
    position(self.1, max_length)
  }

  pub fn marker_type(&self) -> &Markers {
    &self.3
  }

  pub fn make_text(&self) -> Text {
    make_text(&self.2)
  }
}
