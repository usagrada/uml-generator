use crate::{uml::Class, MakeSvg};
use svg::Document;

#[test]
fn test_class() {
  // let theme = ThemeName::Default;
  let mut c = Class::new("hello world");

  c.add_class(
    "class element",
    &[(false, "element"), (true, "element2")],
    &[(true, "method1"), (true, "method2"), (false, "method3")],
  );

  c.add_class(
    "class element2",
    &[(false, "element"), (true, "element2")],
    &[(true, "method1"), (true, "method2"), (false, "method3")],
  );

  let document = Document::new()
    .set("viewBox", c.bounding_box())
    .add(c.make_svg());

  svg::save("img/class.svg", &document).unwrap();
}
