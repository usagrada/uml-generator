use crate::theme::Theme;
use svg::node::element::{Circle, Ellipse, Rectangle};
use svg::node::{Node, Value};

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

pub fn make_rect(width: usize, height: usize, _theme: &Theme) -> Rectangle {
  Rectangle::new().set("width", width).set("height", height)
}

pub trait MakeRect {
  /// 長方形を作るための関数
  fn make_rect(self) -> Rectangle;
}

impl MakeRect for usize {
  /// 正方形を作るための関数
  fn make_rect(self) -> Rectangle {
    Rectangle::new().set("width", self).set("height", self)
  }
}

impl MakeRect for (usize, usize) {
  /// 長方形を作るための関数
  fn make_rect(self) -> Rectangle {
    Rectangle::new().set("width", self.0).set("height", self.1)
  }
}

#[test]
fn make_rect_from_usize() {
  (0, 0).make_rect();
  println!("{}", 2.make_rect());
}

pub trait MakeCircle {
  /// 円を作るための関数
  fn make_circle(self) -> Circle;
}

impl MakeCircle for usize {
  fn make_circle(self) -> Circle {
    Circle::new()
      .set("cx", 0usize)
      .set("cy", 0usize)
      .set("r", self)
  }
}

impl MakeCircle for (usize, usize, usize) {
  fn make_circle(self) -> Circle {
    Circle::new()
      .set("cx", self.0)
      .set("cy", self.1)
      .set("r", self.2)
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

pub trait MakeEllipse {
  /// 楕円を作るための関数
  fn make_ellipse(self) -> Ellipse;
}

impl MakeEllipse for (usize, usize) {
  fn make_ellipse(self) -> Ellipse {
    Ellipse::new()
      .set("cx", 0)
      .set("cy", 0)
      .set("rx", self.0)
      .set("ry", self.1)
  }
}

impl MakeEllipse for (usize, usize, usize, usize) {
  fn make_ellipse(self) -> Ellipse {
    Ellipse::new()
      .set("cx", self.0)
      .set("cy", self.1)
      .set("rx", self.2)
      .set("ry", self.3)
  }
}
