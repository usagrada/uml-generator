use crate::helper::Markers;
use crate::{helper::*, theme::ThemeName, uml::Sequence, MakeSvg};

#[test]
fn test_sequence() {
    let theme = ThemeName::Default;
    let mut s = Sequence::new(theme);
    s.add_node("test1");
    s.add_node("test2");
    s.add_node("test3");
    s.add_node("test4");
    s.add_edge(("test1", "test3", "result", Markers::Array));
    s.add_edge(("test3", "test2", "result", Markers::Array));
    s.add_edge(("test4", "test3", "result", Markers::Array));
    s.add_edge(("test2", "test3", "result", Markers::Array));

    let svg = s.make_svg().change_background_color("#fff".into());

    svg::save("img/sequence.svg", &svg).unwrap();
}
