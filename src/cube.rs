use crate::{
    geometry::{Block, Orientation3D},
    shapes::Shape,
};

pub(crate) struct Cube {
    shape1: CubePart<Shape<3>>,
    shape2: CubePart<Shape<4>>,
    shape3: CubePart<Shape<4>>,
    shape4: CubePart<Shape<4>>,
    shape5: CubePart<Shape<4>>,
    shape6: CubePart<Shape<4>>,
    shape7: CubePart<Shape<4>>,
}

pub(crate) struct CubePart<Shape> {
    origin: Block,
    orientation: Orientation3D,
    shape: Shape,
}

pub(crate) enum CubeCoordinate {
    Start,
    Center,
    End,
}
