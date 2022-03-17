use super::*;
use crate::helper::*;
use crate::theme::Theme;

use svg::node::element::{Group, Line, Text};
use svg::node::Text as TextNode;

#[allow(dead_code)]
#[derive(PartialEq)]
pub struct ClassNode {
    name: String,
    elements: Vec<ClassElement>,
    methods: Vec<ClassMethod>,
}

#[derive(PartialEq)]
pub struct ClassElement {
    public: bool,
    name: String,
}

#[derive(PartialEq)]
pub struct ClassMethod {
    public: bool,
    name: String,
}

impl ClassNode {
    pub fn new<T: Into<String>>(
        name: T,
        elements: Vec<ClassElement>,
        methods: Vec<ClassMethod>,
    ) -> Self {
        ClassNode {
            name: name.into(),
            elements,
            methods,
        }
    }
    pub fn make_svg(&self, theme: &Theme) -> Group {
        let node_rect = make_rect(100, 100).set_theme(theme);
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

    pub fn make_text(&self) -> Text {
        let text_node = TextNode::new(self.name.clone());
        Text::new()
            .add(text_node)
            .set("x", 50) // width / 2
            .set("y", 3 * FONT_SIZE / 2)
            .set("font-size", FONT_SIZE * 3 / 2)
            .set("text-anchor", "middle")
    }

    pub fn make_elements_svg(&self) -> Group {
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
    pub fn make_methods_svg(&self) -> Group {
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

impl ClassElement {
    pub fn new<T: Into<String>>(e: (bool, T)) -> Self {
        Self {
            public: e.0,
            name: e.1.into(),
        }
    }

    pub fn make_svg(&self) -> Group {
        Group::new().add(self.make_mark()).add(self.make_text())
    }

    pub fn make_text(&self) -> Text {
        let text_node = TextNode::new(self.name.clone());
        Text::new()
            .add(text_node)
            .set("x", FONT_SIZE)
            .set("font-size", FONT_SIZE)
    }
    pub fn make_mark(&self) -> Text {
        let mark = if self.public { "+" } else { "-" };
        let mark_node = TextNode::new(mark);
        Text::new()
            .add(mark_node)
            .set("text-anchor", "middle")
            .set("font-size", FONT_SIZE)
    }
}

impl ClassMethod {
    pub fn new<T: Into<String>>(m: (bool, T)) -> Self {
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
