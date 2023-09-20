use crate::geometry::Block;

mod shape1;
mod shape2;
mod shape3;
mod shape4;
mod shape5;
mod shape6;
mod shape7;

pub(crate) struct Shape<const N: usize> {
    blocks: [Block; N],
}

impl<const N: usize> Shape<N> {
    pub(crate) const fn new(blocks: [Block; N]) -> Self {
        Self { blocks }
    }

    pub(crate) fn combinations(&self) -> Vec<Shape<N>> {
        self.rotations().iter().flat_map(Self::moves).collect()
    }

    fn rotations(&self) -> Vec<Shape<N>> {
        let rotations = Vec::new();
        rotations
    }

    fn moves(&self) -> Vec<Shape<N>> {}

    fn move_x_up(&mut self) {
        self.blocks.iter_mut()
    }

    fn move_y_up(&mut self) {
        self.blocks.iter_mut()
    }

    fn move_z_up(&mut self) {
        self.blocks.iter_mut()
    }
}
