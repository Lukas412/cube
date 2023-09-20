use crate::{
    cube::CubeCoordinate::{End, Start},
    geometry::Block,
};

use super::Shape;

pub(crate) const fn new_shape3() -> Shape<4> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_x(End),
        Block::center().with_y(Start),
    ];
    Shape::new(blocks)
}
