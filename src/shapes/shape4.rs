use crate::block::BlockBuilder;
use crate::coordinate::Position::{Number1, Number2};

use super::Shape;

pub(crate) const fn new_shape4() -> Shape<4> {
  let blocks = [
    BlockBuilder::new().build(),
    BlockBuilder::new().x(Number1).build(),
    BlockBuilder::new().x(Number2).build(),
    BlockBuilder::new().y(Number1).build(),
  ];
  Shape::new(blocks)
}
