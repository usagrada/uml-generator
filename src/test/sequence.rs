use crate::{uml::sequence, MakeSvg};
use svg::Document;

#[test]
fn test_sequence() {
    let mut s = sequence::Sequence::new();
    s.add_nodes("test1");
    s.add_nodes("test2");
    s.add_nodes("test3");
    s.add_nodes("test4");
    s.add_edges(("test1", "test3", "result"));
    s.add_edges(("test3", "test2", "result"));
    s.add_edges(("test4", "test3", "result"));
    s.add_edges(("test2", "test3", "result"));

    let a = s.make_svg();
    let bb = s.bounding_box();
    let mut document = Document::new().set("viewBox", bb);
    for value in a.iter() {
        document = document.add(value.clone());
    }

    svg::save("img/sequence.svg", &document).unwrap();
}
