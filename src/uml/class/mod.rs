mod node;
use std::borrow::BorrowMut;

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
    name: String,
    nodes: Vec<ClassNode>,
    edges: Vec<(usize, usize)>,
    theme: Theme,
    bbox_cnt: (usize, usize),
}

impl ClassUML {
    pub fn new<T: Into<String>>(name: T) -> Self {
        Self {
            name: name.into(),
            nodes: vec![],
            edges: vec![],
            theme: Theme::new(ThemeName::Default),
            bbox_cnt: (0, 0),
        }
    }

    pub fn add_class(&mut self, name: &str, elements: &[(bool, &str)], methods: &[(bool, &str)]) {
        self.nodes.push(ClassNode::new(
            name,
            elements.iter().map(|&e| ClassElement::new(e)).collect(),
            methods.iter().map(|&m| ClassMethod::new(m)).collect(),
        ));
    }

    pub fn add_edges(&mut self, edges: &[(usize, usize)]) {
        self.edges.extend_from_slice(edges);
    }

    pub fn get_ranks(&self) -> Vec<usize> {
        let edges = topological_sort_edges(self.nodes.len(), &mut self.edges.clone());
        let rank = calc_rank(&self.nodes, &edges);
        println!("rank: {:?}", &rank);
        rank
    }
}

impl MakeSvg for ClassUML {
    fn make_svg(&mut self) -> Document {
        let mut group = Group::new();
        let mut ranks = self.get_ranks();

        let mut cnt = vec![0; ranks.len()];
        for (index, node) in self.nodes.iter().enumerate() {
            let rank = ranks[index] - 1;
            let node_svg = node
                .make_svg(&self.theme)
                .transform(120 * cnt[ranks[index]], 120 * rank);
            cnt[ranks[index]] += 1;
            group = group.add(node_svg)
        }
        group = group.transform(10, 10);
        ranks.reverse();
        let w = cnt.iter().fold(0, |acc, &c| if c > 0 { c } else { acc });
        let h = *cnt.iter().max().unwrap();
        self.bbox_cnt = (w, h);
        Document::new()
            .add(group)
            .set("viewBox", self.bounding_box())
    }

    fn bounding_box(&self) -> (usize, usize, usize, usize) {
        let mut ranks = self.get_ranks();
        ranks.reverse();
        let x = self.bbox_cnt.0 * 120;
        let y = self.bbox_cnt.1 * 120;
        (0, 0, x, y)
    }
}

#[test]
fn class_test() {
    let mut class = ClassUML::new("class component");
    class.add_class("hello world", &[], &[]);
    assert!(class.nodes == vec![ClassNode::new("hello world", vec![], vec![])]);
}
