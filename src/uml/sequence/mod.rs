mod edge;
mod node;
use edge::Edge;
use node::Node;

const RECT_HEIGHT: usize = 20;
const FONT_SIZE: usize = 8;
const PADDING: usize = 3;
const MARGIN: usize = 5;
const X_INDEX: usize = 20;
const Y_INDEX: usize = 20;
const DEFAULT_HEIGHT: usize = 100;
const VERTICAL_HEIGHT: usize = 30;

#[inline]
fn rect_width(max_length: usize) -> usize {
  FONT_SIZE * max_length + PADDING * 2
}

/// return (x, y)
#[inline]
fn position(index: usize, max_length: usize) -> (usize, usize) {
  (position_x(index, max_length), position_y(index, max_length))
}

#[inline]
fn position_x(index: usize, max_length: usize) -> usize {
  X_INDEX + (max_length * FONT_SIZE + PADDING * 2 + MARGIN * 2) * index
}

#[inline]
fn position_y(_index: usize, _max_length: usize) -> usize {
  Y_INDEX
}

use crate::{
  helper::*,
  make_vec,
  theme::{Theme, ThemeName},
  MakeSvg,
};
use std::collections::{HashMap, HashSet};
use svg::node::element::Group;
use svg::Document;

pub struct Sequence {
  nodes: HashMap<Node, usize>,
  edges: Vec<Edge>,
  max_length: usize,
  markers: HashSet<Markers>,
  theme: Theme,
}

impl Sequence {
  pub fn new(theme: ThemeName) -> Self {
    Sequence {
      nodes: HashMap::new(),
      edges: Vec::new(),
      markers: HashSet::new(),
      max_length: 0,
      theme: Theme::new(theme),
    }
  }

  pub fn add_node(&mut self, text: &str) -> &Self {
    self.nodes.insert(
      Node {
        name: text.to_string(),
      },
      self.nodes.len(),
    );
    if self.max_length < text.len() {
      self.max_length = text.len();
    }
    self
  }
  pub fn add_nodes(&mut self, texts: Vec<&str>) -> &Self {
    for text in texts {
      self.add_node(text);
    }
    self
  }

  pub fn add_edge(&mut self, edge: (&str, &str, &str, Markers)) -> &Self {
    let (start, end, text, marker) = edge;
    self.markers.insert(marker.clone());
    let source_index = self.nodes.get(&Node { name: start.into() });
    let target_index = self.nodes.get(&Node { name: end.into() });

    match (source_index, target_index) {
      (Some(&s), Some(&t)) => self.edges.push(Edge::new(s, t, text.to_string(), marker)),
      (_, _) => println!("invalid error"),
    }
    self
  }

  pub fn add_edges(&mut self, edges: Vec<(&str, &str, &str, Markers)>) -> &Self {
    for edge in edges {
      self.add_edge(edge);
    }
    self
  }

  fn make_nodes(&self) -> Vec<Group> {
    let rect_width = rect_width(self.max_length);
    let vertical_height = self.get_vertical_height();
    self
      .nodes
      .iter()
      .map(|(node, &index)| {
        let (x, y) = position(index, self.max_length);
        let option = make_vec![
          ("fill", self.theme.color.rect.text),
          ("text-anchor", "middle"),
          ("dominant-baseline", "central"),
          ("font-size", FONT_SIZE)
        ];
        let text_element1 = node
          .make_svg()
          .position(x + rect_width / 2, y + RECT_HEIGHT / 2)
          .set_values(&option);

        let text_element2 = node
          .make_svg()
          .position(x + rect_width / 2, y + vertical_height + RECT_HEIGHT / 2)
          .set_values(&option);

        let rect_element1 = (rect_width, RECT_HEIGHT)
          .make_rect()
          .position(x, y)
          .set("rx", 2usize)
          .set("ry", 2usize)
          .set("fill", self.theme.color.rect.fill)
          .set("stroke", self.theme.color.rect.frame)
          .set("stroke-width", 1usize);
        let rect_element2 = rect_element1.clone().set("y", y + vertical_height);

        Group::new()
          .add(rect_element1)
          .add(text_element1)
          .add(rect_element2)
          .add(text_element2)
      })
      .collect()
  }

  fn get_vertical_height(&self) -> usize {
    std::cmp::max(DEFAULT_HEIGHT, (self.edges.len() + 1) * VERTICAL_HEIGHT)
  }

  // 縦線を引く
  fn make_vertical_lines(&self) -> Vec<Group> {
    let height = self.get_vertical_height() - RECT_HEIGHT;
    self
      .nodes
      .iter()
      .enumerate()
      .map(|(index, _)| {
        let (mut x, mut y) = position(index, self.max_length);
        x += rect_width(self.max_length) / 2;
        y += RECT_HEIGHT;
        let path = (x, y, x, y + height)
          .make_line()
          // .set("stroke-dasharray", "4")
          .set("stroke", self.theme.color.line.second);
        println!("{}", self.theme.color.line.second);
        Group::new().add(path)
      })
      .collect()
  }

  // 横線を引く
  fn make_horizontal_lines(&self) -> Vec<Group> {
    self
      .edges
      .iter()
      .enumerate()
      .map(|(index, value)| {
        let mut x1 = value.position_node1(self.max_length).0;
        let mut x2 = value.position_node2(self.max_length).0;
        let (_, y) = position(index, self.max_length);
        let y_path = y + RECT_HEIGHT / 2 + VERTICAL_HEIGHT * (index + 1);
        x1 += rect_width(self.max_length) >> 1;
        x2 += rect_width(self.max_length) >> 1;
        let path = (x1, y_path, x2, y_path)
          .make_line()
          .set("stroke", self.theme.color.line.primary)
          .add_marker_end(&value.marker_type());
        let x_mid = (x1 + x2) >> 1;
        let x = x_mid;
        let y = y_path - FONT_SIZE;
        let text_element = value
          .make_text()
          .position(x, y)
          .set("text-anchor", "middle")
          .set("fill", self.theme.color.text_primary)
          .set("font-size", FONT_SIZE);
        Group::new().add(path).add(text_element)
      })
      .collect()
  }
}

impl MakeSvg for Sequence {
  fn make_svg(&self) -> Document {
    let mut sequence_group = Group::new();
    for vline in self.make_vertical_lines() {
      sequence_group = sequence_group.add(vline);
    }
    for node in self.make_nodes() {
      sequence_group = sequence_group.add(node);
    }
    for hline in self.make_horizontal_lines() {
      sequence_group = sequence_group.add(hline);
    }
    use svg::node::element::Definitions;
    let mut defs = Definitions::new();
    for markers in self.markers.iter() {
      let marker_svg = markers.make_svg();
      defs = defs.add(marker_svg);
    }

    Document::new()
      .set("viewBox", self.bounding_box())
      .add(defs)
      .add(sequence_group)
  }

  fn bounding_box(&self) -> (usize, usize, usize, usize) {
    let x =
      2 * X_INDEX + (self.max_length * FONT_SIZE + PADDING * 2 + MARGIN * 2) * self.nodes.len();
    let y = 2 * Y_INDEX + RECT_HEIGHT * 2 + self.get_vertical_height();
    (0, 0, x, y)
  }
}
