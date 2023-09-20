use crate::block::BlockBuilder;
use crate::coordinate::Position::{Number1, Number2};

use super::Shape;

pub(crate) const fn new_shape5() -> Shape<4> {
  let blocks = [
    BlockBuilder::new().build(),
    BlockBuilder::new().x(Number1).build(),
    BlockBuilder::new().x(Number1).y(Number1).build(),
    BlockBuilder::new().x(Number2).build(),
  ];
  Shape::new(blocks)
}
