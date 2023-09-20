use crate::{cube::CubeCoordinate::Start, geometry::Block};

use super::Shape;

pub(crate) const fn new_shape1() -> Shape<3> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_y(Start),
    ];
    Shape::new(blocks)
}
