use crate::{
    cube::CubeCoordinate::{Center, End, Start},
    geometry::Block,
};

use super::Shape;

pub(crate) const fn new_shape4() -> Shape<4> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_x(End),
        Block::new(End, Start, Center),
    ];
    Shape::new(blocks)
}
