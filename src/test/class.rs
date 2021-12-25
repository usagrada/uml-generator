use crate::{theme::ThemeName, uml::class, MakeSvg};
use svg::Document;

#[test]
fn test_class() {
  // let theme = ThemeName::Default;
  let mut c = class::Class::new();

  c.add_class(
    "class element",
    &[(false, "element"), (true, "element2")],
    &[(true, "method1"), (true, "method2"), (false, "method3")],
  );
  let a = c.make_svg();
  // let bb = s.bounding_box();
  let mut document = Document::new().set("viewBox", (0, 0, 100, 100));
  // for value in a.iter() {
  document = document.add(a);
  // }

  svg::save("img/class.svg", &document).unwrap();
}
