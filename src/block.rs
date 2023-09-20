pub(crate) use builder::BlockBuilder;

use crate::coordinate::rotate;
use crate::coordinate::Position;
use crate::error::ShiftError;

mod builder;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Block {
  x: Position,
  y: Position,
  z: Position,
}

impl Block {
  pub(crate) const fn new(x: Position, y: Position, z: Position) -> Self {
    Self { x, y, z }
  }
}

impl Block {
  pub(crate) fn x(&self) -> Position {
    self.x
  }

  pub(crate) fn y(&self) -> Position {
    self.y
  }

  pub(crate) fn z(&self) -> Position {
    self.z
  }
}

impl Block {
  pub(crate) fn rotate_clockwise_around_x(&mut self) {
    rotate(&mut self.y, &mut self.z);
  }

  pub(crate) fn rotate_clockwise_around_y(&mut self) {
    rotate(&mut self.z, &mut self.x);
  }

  pub(crate) fn rotate_clockwise_around_z(&mut self) {
    rotate(&mut self.x, &mut self.y);
  }
}

impl Block {
  pub(crate) fn shift_x_up(&mut self) -> Result<(), ShiftError> {
    self.x.shift_up()
  }

  pub(crate) fn shift_y_up(&mut self) -> Result<(), ShiftError> {
    self.y.shift_up()
  }

  pub(crate) fn shift_z_up(&mut self) -> Result<(), ShiftError> {
    self.z.shift_up()
  }
}
