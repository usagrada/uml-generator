use std::collections::{HashMap, VecDeque};

use svg::node::element::{Circle, Ellipse, Line, Rectangle, Text};
use svg::node::{Node, Text as TextNode, Value};
use svg::Document;

mod line;
mod marker;
mod theme;
mod transform;
pub use line::*;
pub use marker::*;
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

pub trait SetValue
where
    Self: Sized + Node,
{
    fn set_values(mut self, values: &Vec<(String, Value)>) -> Self {
        for (key, value) in values {
            self.assign(key, value.clone());
        }
        self
    }
}

impl SetValue for Rectangle {}
impl SetValue for Ellipse {}
impl SetValue for Circle {}
impl SetValue for Text {}

pub trait BackgroundColor {
    fn change_background_color(self, color: String) -> Self;
}

impl BackgroundColor for Document {
    fn change_background_color(mut self, color: String) -> Self {
        self.assign("style", format!("background-color:{}", color));
        self
    }
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

pub fn topological_sort_edges(n: usize, edges: &mut Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut cnts = vec![0_usize; n];
    let mut order = vec![];
    for edge in edges.iter() {
        cnts[edge.1 - 1] += 1;
    }

    let mut queue = VecDeque::new();
    for (index, &cnt) in cnts.iter().enumerate() {
        if cnt == 0 {
            queue.push_back(index + 1);
        }
    }

    while !queue.is_empty() {
        // 幅優先探索
        let node = { queue.pop_front().unwrap() };
        order.push(node);
        for edge in edges.iter() {
            if cnts[edge.1 - 1] > 0 {
                cnts[edge.1 - 1] -= 1;
                if cnts[edge.1 - 1] == 0 {
                    queue.push_back(edge.1);
                }
            }
        }
    }
    if order.len() != n {
        panic!("topological sort failed");
    }

    println!("order: {:?}", order);
    let mut hashmap = HashMap::new();
    for (index, o) in order.iter().enumerate() {
        hashmap.insert(index + 1, o);
    }
    edges.sort_by(|from, to| {
        (hashmap.get(&from.0), hashmap.get(&from.1)).cmp(&(hashmap.get(&to.0), hashmap.get(&to.1)))
    });
    edges.to_vec()
}

// T is 1-indexed value
// edge is sorted by topological order
pub fn calc_rank<T>(nodes: &Vec<T>, edges: &Vec<(usize, usize)>) -> Vec<usize> {
    let len = 1 + nodes.len();
    let mut visited = vec![false; len];
    // 仮想ノード0を作成し、そこからエッジがあると考える
    visited[0] = true;
    let mut ranks = vec![1; len];
    for edge in edges {
        let (from, to) = (edge.0, edge.1);
        ranks[to] = ranks[to].max(ranks[from] + 1);
        if visited[to] {
            // check children
            for edge in edges {
                if edge.0 == to {
                    ranks[edge.1] = ranks[edge.1].max(ranks[to] + 1);
                }
            }
        }
        visited[from] = true;
        visited[to] = true;
    }
    ranks
}
