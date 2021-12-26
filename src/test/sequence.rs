use crate::helper::Markers;
use crate::{theme::ThemeName, uml::Sequence, MakeSvg};

#[test]
fn test_sequence() {
  let theme = ThemeName::Default;
  let mut s = Sequence::new(theme);
  s.add_nodes("test1");
  s.add_nodes("test2");
  s.add_nodes("test3");
  s.add_nodes("test4");
  s.add_edges(("test1", "test3", "result", Markers::Array));
  s.add_edges(("test3", "test2", "result", Markers::Array));
  s.add_edges(("test4", "test3", "result", Markers::Array));
  s.add_edges(("test2", "test3", "result", Markers::Array));

  let svg = s.make_svg();

  svg::save("img/sequence.svg", &svg).unwrap();
}
