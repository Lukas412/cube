use crate::{
    cube::CubeCoordinate::{Center, End, Start},
    geometry::Block,
};

use super::Shape;

pub(crate) const fn new_shape6() -> Shape<4> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_z(End),
        Block::new(Start, Start, Center),
    ];
    Shape::new(blocks)
}
