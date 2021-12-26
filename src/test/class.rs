use crate::{uml::ClassUML, MakeSvg};

#[test]
fn test_class() {
  // let theme = ThemeName::Default;
  let mut c = ClassUML::new("hello world");

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

  let svg = c.make_svg();

  svg::save("img/class.svg", &svg).unwrap();
}
