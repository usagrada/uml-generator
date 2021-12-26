use crate::{
  helper::*,
  make_vec,
  theme::{Theme, ThemeName},
  MakeSvg,
};
use std::collections::{HashMap, HashSet};
use svg::node::element::Group;
use svg::Document;

const RECT_HEIGHT: usize = 20;
const FONT_SIZE: usize = 8;
const PADDING: usize = 3;
const MARGIN: usize = 5;
const X_INDEX: usize = 20;
const Y_INDEX: usize = 20;
const DEFAULT_HEIGHT: usize = 100;
const VERTICAL_HEIGHT: usize = 30;

#[allow(dead_code)]
pub struct Sequence {
  nodes: HashMap<Node, usize>,
  edges: Vec<Edge>,
  max_length: usize,
  markers: HashSet<Markers>,
  theme: Theme,
}

struct Edge(usize, usize, String, Markers);

#[derive(PartialEq, Eq, Hash)]
struct Node {
  name: String,
}

#[allow(dead_code)]
impl Sequence {
  pub fn new(theme: ThemeName) -> Self {
    Sequence {
      nodes: HashMap::new(),
      edges: vec![],
      markers: HashSet::new(),
      max_length: 0,
      theme: Theme::new(theme),
    }
  }

  pub fn add_nodes(&mut self, text: &str) -> &Self {
    self.nodes.insert(
      Node {
        name: text.to_string(),
      },
      self.nodes.len(),
    );
    self.max_length = std::cmp::max(self.max_length, text.len());
    self
  }

  pub fn add_edges(&mut self, edge: (&str, &str, &str, Markers)) -> &Self {
    // TODO
    let (start, end, text, marker) = edge;
    self.markers.insert(marker.clone());
    let source_index = self.nodes.get(&Node { name: start.into() });
    let target_index = self.nodes.get(&Node { name: end.into() });

    match (source_index, target_index) {
      (Some(&s), Some(&t)) => self.edges.push(Edge(s, t, text.to_string(), marker)),
      (_, _) => println!("invalid error"),
    }
    self
  }

  fn make_node(&self) -> Vec<Group> {
    let rect_width = self.rect_width();
    let vertical_height = self.get_vertical_height();
    self
      .nodes
      .iter()
      .enumerate()
      .map(|(index, (node, _))| {
        let (x, y) = self.position(index);
        let text_element1 = {
          let text = make_text(&node.name).position(x + rect_width / 2, y + RECT_HEIGHT / 2);
          let option = make_vec![
            ("fill", self.theme.color.rect.text),
            ("text-anchor", "middle"),
            ("dominant-baseline", "central"),
            ("font-size", FONT_SIZE)
          ];
          make_element(text, &option)
        };
        let text_element2 = text_element1
          .clone()
          .set("y", y + vertical_height + RECT_HEIGHT / 2);

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

  fn make_vertical_line(&self) -> Vec<Group> {
    let height = self.get_vertical_height() - RECT_HEIGHT;
    self
      .nodes
      .iter()
      .enumerate()
      .map(|(index, _)| {
        let (mut x, mut y) = self.position(index);
        x += self.rect_width() / 2;
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

  fn make_horizontal_line(&self) -> Vec<Group> {
    self
      .edges
      .iter()
      .enumerate()
      .map(|(index, value)| {
        let (mut x1, mut y1) = self.position(value.0);
        let (mut x2, mut y2) = self.position(value.1);
        x1 += self.rect_width() >> 1;
        y1 += RECT_HEIGHT / 2 + VERTICAL_HEIGHT * (index + 1);
        x2 += self.rect_width() >> 1;
        y2 += RECT_HEIGHT / 2 + VERTICAL_HEIGHT * (index + 1);
        let path = (x1, y1, x2, y2)
          .make_line()
          .set("stroke", self.theme.color.line.primary)
          .add_marker_end(&value.3);
        let x_mid = (x1 + x2) >> 1;
        let y_mid = (y1 + y2) >> 1; // 実際には y1 == y2;
        let x = x_mid;
        let y = y_mid - FONT_SIZE;
        let text_element = make_text(&value.2)
          .position(x, y)
          .set("text-anchor", "middle")
          .set("fill", self.theme.color.text_primary)
          .set("font-size", FONT_SIZE);
        Group::new().add(path).add(text_element)
      })
      .collect()
  }

  fn rect_width(&self) -> usize {
    FONT_SIZE * self.max_length + PADDING * 2
  }

  // return (x, y)
  fn position(&self, index: usize) -> (usize, usize) {
    (
      X_INDEX + (self.max_length * FONT_SIZE + PADDING * 2 + MARGIN * 2) * index,
      Y_INDEX,
    )
  }
}

impl MakeSvg for Sequence {
  fn make_svg(&self) -> Document {
    let mut sequence_group = Group::new();
    for vline in self.make_vertical_line() {
      sequence_group = sequence_group.add(vline);
    }
    for node in self.make_node() {
      sequence_group = sequence_group.add(node);
    }
    for hline in self.make_horizontal_line() {
      sequence_group = sequence_group.add(hline);
    }
    use svg::node::element::{Definitions, Marker};
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

// <marker id="arrow" ="" 
//         
//         
//       
//     </marker>
