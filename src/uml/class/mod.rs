mod node;

use crate::helper::*;
use crate::theme::{Theme, ThemeName};
use crate::MakeSvg;
use node::*;
use svg::node::element::{Group, Line};
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

    pub fn get_ranks(&mut self) -> Vec<usize> {
        let edges = topological_sort_edges(self.nodes.len(), &mut self.edges);
        let rank = calc_rank(&self.nodes, &edges);
        println!("rank: {:?}", &rank);
        rank
    }

    pub fn make_edge(&self, ranks: &Vec<usize>, rranks: &Vec<usize>, from: usize, to: usize) -> Line {
        let from_rank = ranks[from] - 1;
        let to_rank = ranks[to] - 1;

        let from_rrank = rranks[from];
        let to_rrank = rranks[to];

        let from_x = from_rrank * 120 + 50;
        let from_y = from_rank * 120 + 100;
        let to_x = to_rrank * 120 + 50;
        let to_y = to_rank * 120;
        Line::new()
            .set("x1", from_x)
            .set("y1", from_y)
            .set("x2", to_x)
            .set("y2", to_y)
            .set("stroke-width", 2)
            .set("stroke", "#000000")
    }
}

// struct Line

impl MakeSvg for ClassUML {
    fn make_svg(&mut self) -> Document {
        let mut group = Group::new();
        // 0-n
        let mut ranks = self.get_ranks();
        
        let mut cnt = vec![0; ranks.len()];
        let mut row_ranks = vec![0usize; ranks.len()];
        let mut node_vec = vec![];
        for (index, node) in self.nodes.iter().enumerate() {
            let rank = ranks[index + 1] - 1;
            let node_svg = node
                .make_svg(&self.theme)
                .transform(120 * cnt[rank], 120 * rank);
            node_vec.push(node_svg);
            row_ranks[index + 1] = cnt[rank];
            cnt[rank] += 1;
        }

        for (index, _) in self.nodes.iter().enumerate() {
            for edge in self.edges.iter() {
                if edge.0 == index + 1 {
                    println!("{:?} {}", edge, index);
                    let edge_svg = self.make_edge(&ranks, &row_ranks, edge.0, edge.1);
                    group = group.add(edge_svg)
                }
            }
        }
        for node_svg in node_vec {
            group = group.add(node_svg);
        }
        group = group.transform(10, 10);
        ranks.reverse();
        let w = cnt.iter().fold(0, |acc, &c| if c > 0 { c } else { acc });
        let h = *ranks.iter().max().unwrap();
        self.bbox_cnt = (w, h);
        Document::new()
            .add(group)
            .set("viewBox", self.bounding_box())
    }

    fn bounding_box(&self) -> (usize, usize, usize, usize) {
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
