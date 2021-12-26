mod node;
use crate::helper::*;
use crate::theme::{Theme, ThemeName};
use crate::MakeSvg;
use node::*;
use svg::node::element::Group;
use svg::Document;
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
    self.nodes.push(ClassNode::new(
      name,
      elements.iter().map(|&e| ClassElement::new(e)).collect(),
      methods.iter().map(|&m| ClassMethod::new(m)).collect(),
    ));
  }
}

impl MakeSvg for ClassUML {
  fn make_svg(&self) -> Document {
    let mut group = Group::new();
    for (index, node) in self.nodes.iter().enumerate() {
      group = group.add(node.make_svg(&self.theme).transform(0, 120 * index))
    }
    group = group.transform(10, 10);
    Document::new()
      .add(group)
      .set("viewBox", self.bounding_box())
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
  assert!(class.nodes == vec![ClassNode::new("hello world", vec![], vec![])]);
}
