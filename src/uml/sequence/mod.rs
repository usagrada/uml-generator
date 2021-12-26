mod edge;
mod node;
mod values;
use values::*;
mod sequence;
pub use sequence::Sequence;

fn rect_width(max_length: usize) -> usize {
  FONT_SIZE * max_length + PADDING * 2
}

// return (x, y)
fn position(index: usize, max_length: usize) -> (usize, usize) {
  (position_x(index, max_length), position_y(index, max_length))
}

#[inline]
fn position_x(index: usize, max_length: usize) -> usize {
  X_INDEX + (max_length * FONT_SIZE + PADDING * 2 + MARGIN * 2) * index
}

#[inline]
fn position_y(_index: usize, _max_length: usize) -> usize {
  Y_INDEX
}
