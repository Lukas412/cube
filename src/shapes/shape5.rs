use crate::{
    cube::CubeCoordinate::{Center, End, Start},
    geometry::Block,
};

use super::Shape;

pub(crate) const fn new_shape5() -> Shape<4> {
    let blocks = [
        Block::center(),
        Block::center().with_x(Start),
        Block::center().with_y(End),
        Block::new(Start, End, Center),
    ];
    Shape::new(blocks)
}
