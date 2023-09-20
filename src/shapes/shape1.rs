use crate::block::BlockBuilder;
use crate::coordinate::Position::Number1;

use super::Shape;

pub(crate) const fn new_shape1() -> Shape<3> {
  let blocks = [
    BlockBuilder::new().build(),
    BlockBuilder::new().x(Number1).build(),
    BlockBuilder::new().y(Number1).build(),
  ];
  Shape::new(blocks)
}
