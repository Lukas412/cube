use crate::{cube::CubeCoordinate::Start, geometry::Block};

use super::Shape;

pub(crate) const fn new_shape2() -> Shape<4> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_y(Start),
        Block::center().with_z(Start),
    ];
    Shape::new(blocks)
}
