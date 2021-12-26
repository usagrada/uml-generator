mod edge;
mod node;
mod sequence;
pub use sequence::Sequence;

const RECT_HEIGHT: usize = 20;
const FONT_SIZE: usize = 8;
const PADDING: usize = 3;
const MARGIN: usize = 5;
const X_INDEX: usize = 20;
const Y_INDEX: usize = 20;
const DEFAULT_HEIGHT: usize = 100;
const VERTICAL_HEIGHT: usize = 30;

#[inline]
fn rect_width(max_length: usize) -> usize {
  FONT_SIZE * max_length + PADDING * 2
}

/// return (x, y)
#[inline]
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
