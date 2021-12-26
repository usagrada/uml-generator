use svg::node::element::{Marker, Path};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Markers {
  None,
  Array,
}

impl Markers {
  pub fn get_id(&self) -> Option<String> {
    match self {
      Markers::None => None,
      _ => Some(format!("url(#{})", self.get_as_id())),
    }
  }

  fn get_as_id(&self) -> String {
    format!("marker-{}", self.clone() as usize)
  }

  pub fn make_svg(&self) -> Marker {
    let mut marker = Marker::new().set("id", self.get_as_id());
    match self {
      Markers::Array => {
        marker = marker
          .set("viewBox", (0, 0, 10, 10))
          .set("markerWidth", 5)
          .set("markerHeight", 5)
          .set("orient", "auto-start-reverse")
          .set("refX", 10)
          .set("refY", "5");
        marker = marker.add(Path::new().set("d", "M 0 0 L 10 5 L 0 10 z"));
      }
      _ => {}
    }
    marker
  }
}

trait MarkerMethod {
  fn add_marker(self, maker: Markers) -> Option<Marker>;
}

impl MarkerMethod for Marker {
  fn add_marker(self, marker: Markers) -> Option<Marker> {
    match marker {
      Markers::Array => Some(make_marker()),
      _ => None,
    }
  }
}

fn make_marker() -> Marker {
  Marker::new()
}

trait MarkerTrait {
  fn make_mark(&self) -> &Self {
    // self.add()
    self
  }
}

impl MarkerTrait for Marker {}
