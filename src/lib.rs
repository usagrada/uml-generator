use svg::node::element::Group;
use svg::Document;
pub mod helper;
#[cfg(test)]
mod test;
pub mod theme;
pub mod uml;

#[macro_export]
macro_rules! make_vec {
  ( $( $x:expr ),* ) => ( vec![ $( crate::helper::make_pair($x) ),* ] );
}

pub trait MakeSvg {
  fn bounding_box(&self) -> (usize, usize, usize, usize);
  fn make_svg(&self) -> Group;
  fn make_img<T: Into<String>>(&self, name: T) {
    let svg = self.make_svg();
    let document = Document::new().set("viewBox", self.bounding_box()).add(svg);
    svg::save(name.into(), &document).unwrap();
  }
}
