use crate::helper::*;
use crate::theme::{Theme, ThemeName};
use crate::MakeSvg;
use svg::node::element::{Group, Line, Rectangle, Text};
use svg::node::Text as TextNode;

const FONT_SIZE: usize = 8;
const PADDING: usize = 3;
const MARGIN: usize = 5;

#[allow(dead_code)]
pub struct ClassUML {
  pub name: String,
  nodes: Vec<ClassNode>,
  theme: Theme,
}

impl ClassUML {
  pub fn new<T: Into<String>>(name: T) -> Self {
    Self {
      name: name.into(),
      nodes: vec![],
      theme: Theme::new(ThemeName::Default),
    }
  }

  pub fn add_class(&mut self, name: &str, elements: &[(bool, &str)], methods: &[(bool, &str)]) {
    self.nodes.push(ClassNode {
      name: name.to_string(),
      elements: elements.iter().map(|&e| Element::new(e)).collect(),
      methods: methods.iter().map(|&m| Method::new(m)).collect(),
    });
  }
}

impl MakeSvg for ClassUML {
  fn make_svg(&self) -> Group {
    let mut group = Group::new();
    for (index, node) in self.nodes.iter().enumerate() {
      group = group.add(node.make_svg(&self.theme).transform(0, 120 * index))
    }
    group.transform(10, 10)
  }

  fn bounding_box(&self) -> (usize, usize, usize, usize) {
    let x = 300;
    let y = 300;
    (0, 0, x, y)
  }
}

#[test]
fn class_test() {
  let mut class = ClassUML::new("class component");
  class.add_class("hello world", &[], &[]);
  assert!(
    class.nodes
      == vec![ClassNode {
        name: String::from("hello world"),
        elements: vec![],
        methods: vec![],
      }]
  );
}

#[allow(dead_code)]
#[derive(PartialEq)]
struct ClassNode {
  name: String,
  elements: Vec<Element>,
  methods: Vec<Method>,
}

impl ClassNode {
  fn make_svg(&self, theme: &Theme) -> Group {
    let node_rect = make_rect(100, 100, theme);
    let node_text = self.make_text();
    let elements_svg = self.make_elements_svg();
    let methods_svg = self.make_methods_svg();

    let height_title = FONT_SIZE * 2;
    let line1 = Line::new()
      .set("x1", 0)
      .set("y1", height_title)
      .set("x2", 100)
      .set("y2", height_title)
      .set("stroke", "#000");
    let elements_height = (FONT_SIZE + PADDING) * self.elements.len();

    let line2 = Line::new()
      .set("x1", 0)
      .set("y1", height_title + elements_height + MARGIN * 2)
      .set("x2", 100)
      .set("y2", height_title + elements_height + MARGIN * 2)
      .set("stroke", "#000");

    Group::new()
      .add(node_rect)
      .add(line1)
      .add(line2)
      .add(node_text)
      .add(elements_svg)
      .add(methods_svg)
  }

  fn make_text(&self) -> Text {
    let text_node = TextNode::new(self.name.clone());
    Text::new()
      .add(text_node)
      .set("x", 50) // width / 2
      .set("y", 3 * FONT_SIZE / 2)
      .set("font-size", FONT_SIZE * 3 / 2)
      .set("text-anchor", "middle")
  }

  fn make_elements_svg(&self) -> Group {
    let mut group = Group::new().set("transform", format!("translate(0, {})", 2 * FONT_SIZE));
    let elements_svg: Vec<Group> = self
      .elements
      .iter()
      .enumerate()
      .map(|(index, el)| {
        el.make_svg().set(
          "transform",
          format!(
            "translate(10, {})",
            MARGIN + FONT_SIZE + index * (FONT_SIZE + PADDING)
          ),
        )
      })
      .collect();
    for element in elements_svg {
      group = group.add(element);
    }
    group
  }

  /// make group of methods
  fn make_methods_svg(&self) -> Group {
    let mut group = Group::new().set(
      "transform",
      format!(
        "translate(0,{})",
        2 * FONT_SIZE + (FONT_SIZE + PADDING) * self.elements.len() + MARGIN * 2
      ),
    );
    let methods_svg = self.methods.iter().enumerate().map(|(index, method)| {
      method.make_svg().set(
        "transform",
        format!(
          "translate(10, {})",
          MARGIN + FONT_SIZE + index * (FONT_SIZE + PADDING)
        ),
      )
    });
    for method in methods_svg {
      group = group.add(method);
    }
    group
  }
}

fn make_rect(width: usize, height: usize, theme: &Theme) -> Rectangle {
  (width, height).make_rect().set_theme(theme)
}

#[derive(PartialEq)]
struct Element {
  public: bool,
  name: String,
}

impl Element {
  fn new<T: Into<String>>(e: (bool, T)) -> Self {
    Self {
      public: e.0,
      name: e.1.into(),
    }
  }

  fn make_svg(&self) -> Group {
    Group::new().add(self.make_mark()).add(self.make_text())
  }

  fn make_text(&self) -> Text {
    let text_node = TextNode::new(self.name.clone());
    Text::new()
      .add(text_node)
      .set("x", FONT_SIZE)
      .set("font-size", FONT_SIZE)
  }
  fn make_mark(&self) -> Text {
    let mark = if self.public { "+" } else { "-" };
    let mark_node = TextNode::new(mark);
    Text::new()
      .add(mark_node)
      .set("text-anchor", "middle")
      .set("font-size", FONT_SIZE)
  }
}

impl Method {
  fn new<T: Into<String>>(m: (bool, T)) -> Self {
    Self {
      public: m.0,
      name: m.1.into(),
    }
  }

  fn make_svg(&self) -> Group {
    Group::new().add(self.make_mark()).add(self.make_text())
  }

  fn make_text(&self) -> Text {
    let text_node = TextNode::new(self.name.clone());
    Text::new()
      .add(text_node)
      .set("x", FONT_SIZE)
      .set("font-size", FONT_SIZE)
  }
  fn make_mark(&self) -> Text {
    let mark = if self.public { "+" } else { "-" };
    let mark_node = TextNode::new(mark);
    Text::new()
      .add(mark_node)
      .set("text-anchor", "middle")
      .set("font-size", FONT_SIZE)
  }
}

#[derive(PartialEq)]
struct Method {
  public: bool,
  name: String,
}
