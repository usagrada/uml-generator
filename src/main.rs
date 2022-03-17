use std::collections::{HashMap, VecDeque};

use uml_generator::{helper::BackgroundColor, theme::ThemeName, uml::ClassUML, MakeSvg};

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    name: String,
    // value: NodeValue,
}

impl Node {
    fn new(id: usize, name: String) -> Self {
        Node {
            id,
            name,
            // value: NodeValue { children: vec![] },
        }
    }
}

fn node(id: usize, name: &str) -> Node {
    Node::new(id, name.to_string())
}

fn main() {
    println!("Hello, world!");
    let nodes = vec![
        node(1, "a"),
        node(2, "b"),
        node(3, "c"),
        node(4, "d"),
        node(5, "e"),
        node(6, "f"),
        node(7, "g"),
    ];
    let edges = vec![(1, 2), (2, 4), (3, 6), (1, 5), (2, 3), (3, 4), (5, 6)];
    // 1 -> 2 -> 3 -> 4
    // 1 -> 5 -> 6
    // 3 -> 6
    // topological_sort(nodes.len(), &mut edges);
    println!("{:?}", edges);
    let ranks = vec![1]; //calc(&nodes, &edges);
    let mut graph = vec![vec![]; nodes.len() + 1];
    for i in 1..ranks.len() {
        graph[ranks[i]].push(nodes[i - 1].clone());
        // println!("{} {}: {}", i, nodes[i - 1].name, ranks[i]);
    }
    for g in graph.iter().enumerate() {
        println!(
            "{}: {}",
            g.0,
            g.1.iter()
                .map(|x| x.name.clone())
                .collect::<Vec<String>>()
                .join(" ")
        );
    }

    let mut s = ClassUML::new("class");
    s.add_class("test1", &[], &[]);
    s.add_class("test2", &[], &[]);
    s.add_class("test3", &[], &[]);
    s.add_class("test4", &[], &[]);
    s.add_class("test5", &[], &[]);
    s.add_class("test6", &[], &[]);
    s.add_class("test7", &[], &[]);
    s.add_edges(edges.as_slice());

    let svg = s.make_svg().change_background_color("#fff".into());

    svg::save("img/main.svg", &svg).unwrap();
}
