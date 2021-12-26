use svg::node::element::{Circle, Ellipse, Line, Rectangle, Text};
use svg::node::{Node, Text as TextNode, Value};

mod theme;
mod transform;
pub use theme::*;
pub use transform::*;

pub fn make_element<T>(mut elemenet: T, values: &[(String, Value)]) -> T
where
  T: Node,
{
  for (key, value) in values.iter() {
    elemenet.assign(key, value.to_owned());
  }
  elemenet
}

pub fn make_pair<K, V>(prop: (K, V)) -> (String, Value)
where
  K: Into<String>,
  V: Into<Value>,
{
  (prop.0.into(), prop.1.into())
}

pub fn make_rect(width: usize, height: usize) -> Rectangle {
  Rectangle::new().set("width", width).set("height", height)
}

pub trait MakeRect {
  /// 長方形を作るための関数
  fn make_rect(self) -> Rectangle;
}

impl MakeRect for usize {
  /// 正方形を作るための関数
  fn make_rect(self) -> Rectangle {
    make_rect(self, self)
  }
}

impl MakeRect for (usize, usize) {
  /// 長方形を作るための関数
  fn make_rect(self) -> Rectangle {
    make_rect(self.0, self.1)
  }
}

#[test]
fn make_rect_from_usize() {
  (0, 0).make_rect();
  println!("{}", 2.make_rect());
}

/// 円を作るための関数
pub fn make_circle(radius: usize) -> Circle {
  Circle::new().set("r", radius)
}

pub trait MakeCircle {
  /// 円を作るための関数
  fn make_circle(self) -> Circle;
}

impl MakeCircle for usize {
  fn make_circle(self) -> Circle {
    make_circle(self)
  }
}

impl MakeCircle for (usize, usize, usize) {
  fn make_circle(self) -> Circle {
    make_circle(self.2).position(self.0, self.1)
  }
}

#[test]
fn make_circle_from_usize3() {
  use MakeCircle;
  let a = (1, 2, 3);
  println!("{}", a.make_circle());
  println!("{:?}", a);
  3.make_circle();
}

/// makeEllipse
pub fn make_ellipse(rx: usize, ry: usize) -> Ellipse {
  Ellipse::new().set("rx", rx).set("ry", ry)
}

pub trait MakeEllipse {
  /// 楕円を作るための関数
  fn make_ellipse(self) -> Ellipse;
}

impl MakeEllipse for (usize, usize) {
  fn make_ellipse(self) -> Ellipse {
    make_ellipse(self.0, self.1)
  }
}

impl MakeEllipse for (usize, usize, usize, usize) {
  fn make_ellipse(self) -> Ellipse {
    make_ellipse(self.2, self.3).position(self.0, self.1)
  }
}
/// makeLine
pub fn make_line(x1: usize, y1: usize, x2: usize, y2: usize) -> Line {
  Line::new()
    .set("x1", x1)
    .set("y1", y1)
    .set("x2", x2)
    .set("y2", y2)
}

pub trait MakeLine {
  fn make_line(self) -> Line;
}

impl MakeLine for (usize, usize) {
  fn make_line(self) -> Line {
    make_line(0, 0, self.0, self.1)
  }
}

impl MakeLine for (usize, usize, usize, usize) {
  fn make_line(self) -> Line {
    make_line(self.0, self.1, self.2, self.3)
  }
}

/// makeText
pub fn make_text<T: Into<String>>(text: T) -> Text {
  let text_node = TextNode::new(text.into());
  Text::new().add(text_node)
}

pub trait MakeText {
  fn make_text(self) -> Text;
}

impl MakeText for String {
  fn make_text(self) -> Text {
    make_text(self)
  }
}
