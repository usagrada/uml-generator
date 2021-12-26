use crate::{
  helper::make_element,
  make_vec,
  theme::{Theme, ThemeName},
  MakeSvg,
};
use svg::node::element::{Group, Line, Rectangle, Text};
use svg::node::Text as TextNode;

const RECT_HEIGHT: usize = 20;
const FONT_SIZE: usize = 8;
const PADDING: usize = 3;
const MARGIN: usize = 5;
const X_INDEX: usize = 20;
const Y_INDEX: usize = 20;
const DEFAULT_HEIGHT: usize = 100;
const VERTICAL_HEIGHT: usize = 30;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Sequence {
  nodes: Vec<Node>,
  edges: Vec<Edge>,
  max_length: usize,
  theme: Theme,
}
#[derive(Debug)]
struct Edge(usize, usize, String);

#[derive(Debug)]
struct Node {
  name: String,
}

#[allow(dead_code)]
impl Sequence {
  pub fn new(theme: ThemeName) -> Self {
    Sequence {
      nodes: vec![],
      edges: vec![],
      max_length: 0,
      theme: Theme::new(theme),
    }
  }

  pub fn add_nodes(&mut self, text: &str) -> &Self {
    self.nodes.push(Node {
      name: text.to_string(),
    });
    self.max_length = std::cmp::max(self.max_length, text.len());
    self
  }

  pub fn add_edges(&mut self, edge: (&str, &str, &str)) -> &Self {
    let (start, end, text) = edge;
    let source_index = self.nodes.iter().position(|x| x.name == start);
    let target_index = self.nodes.iter().position(|x| x.name == end);

    match (source_index, target_index) {
      (Some(s), Some(t)) => self.edges.push(Edge(s, t, text.to_string())),
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
      .map(|(index, obj)| {
        let (x, y) = self.position(index);
        let text_node = TextNode::new(&obj.name);
        let option = make_vec![
          ("x", x + rect_width / 2),
          ("y", y + RECT_HEIGHT / 2),
          ("fill", self.theme.color.rect.text),
          ("text-anchor", "middle"),
          ("dominant-baseline", "central"),
          ("font-size", FONT_SIZE)
        ];
        let text_element1 = make_element(Text::new(), &option).add(text_node);
        let text_element2 = text_element1
          .clone()
          .set("y", y + vertical_height + RECT_HEIGHT / 2);

        let rect_element1 = Rectangle::new()
          .set("x", x)
          .set("y", y)
          .set("rx", 2)
          .set("ry", 2)
          .set("width", rect_width)
          .set("height", RECT_HEIGHT)
          .set("fill", self.theme.color.rect.fill)
          .set("stroke", self.theme.color.rect.frame)
          .set("stroke-width", 1);
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
    let height = self.get_vertical_height();
    self
      .nodes
      .iter()
      .enumerate()
      .map(|(index, _)| {
        let (mut x, mut y) = self.position(index);
        x += self.rect_width() / 2;
        y += RECT_HEIGHT / 2;
        let path = Line::new()
          .set("x1", x)
          .set("y1", y)
          .set("x2", x)
          .set("y2", y + height)
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
        x1 += self.rect_width() / 2;
        y1 += RECT_HEIGHT / 2 + VERTICAL_HEIGHT * (index + 1);
        x2 += self.rect_width() / 2;
        y2 += RECT_HEIGHT / 2 + VERTICAL_HEIGHT * (index + 1);
        let path = Line::new()
          .set("x1", x1)
          .set("y1", y1)
          .set("x2", x2)
          .set("y2", y2)
          .set("stroke", self.theme.color.line.primary)
          .set("marker-end", "url(#m)");
        let x_mid = (x1 + x2) / 2;
        let y_mid = (y1 + y2) / 2; // 実際には y1 == y2;
        let x = x_mid;
        let y = y_mid - FONT_SIZE;
        let text_node = TextNode::new(&value.2);
        let text_element = Text::new()
          .add(text_node)
          .set("text-anchor", "middle")
          .set("fill", self.theme.color.text_primary)
          .set("font-size", FONT_SIZE)
          .set("x", x)
          .set("y", y);
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
  fn make_svg(&self) -> Group {
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

    sequence_group
  }

  fn bounding_box(&self) -> (usize, usize, usize, usize) {
    let x =
      2 * X_INDEX + (self.max_length * FONT_SIZE + PADDING * 2 + MARGIN * 2) * self.nodes.len();
    let y = 2 * Y_INDEX + RECT_HEIGHT * 2 + self.get_vertical_height();
    (0, 0, x, y)
  }
}
