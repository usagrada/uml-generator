use super::Markers;
use svg::node::element::Line;
use svg::Node;

pub trait LineMethods
where
    Self: Sized + Node,
{
    fn add_marker_start(mut self, marker: &Markers) -> Self {
        let marker = marker.get_id();
        if let Some(marker_id) = marker {
            self.assign("marker-start", marker_id);
        }
        self
    }
    fn add_marker_end(mut self, marker: &Markers) -> Self {
        let marker = marker.get_id();
        if let Some(marker_id) = marker {
            self.assign("marker-end", marker_id);
        }
        self
    }
}

impl LineMethods for Line {}
