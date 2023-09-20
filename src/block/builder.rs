use crate::block::Block;
use crate::coordinate::Position;
use crate::coordinate::Position::Number0;

pub(crate) struct BlockBuilder {
  x: Position,
  y: Position,
  z: Position,
}

impl BlockBuilder {
  pub(crate) const fn new() -> Self {
    Self {
      x: Number0,
      y: Number0,
      z: Number0,
    }
  }

  pub(crate) const fn x(mut self, x: Position) -> Self {
    self.x = x;
    self
  }

  pub(crate) const fn y(mut self, y: Position) -> Self {
    self.y = y;
    self
  }

  pub(crate) const fn z(mut self, z: Position) -> Self {
    self.z = z;
    self
  }

  pub(crate) const fn build(self) -> Block {
    Block::new(self.x, self.y, self.z)
  }
}
