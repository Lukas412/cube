use crate::block::BlockBuilder;
use crate::coordinate::Position::Number1;

use super::Shape;

pub(crate) const fn new_shape2() -> Shape<4> {
  let blocks = [
    BlockBuilder::new().build(),
    BlockBuilder::new().x(Number1).build(),
    BlockBuilder::new().y(Number1).build(),
    BlockBuilder::new().z(Number1).build(),
  ];
  Shape::new(blocks)
}
