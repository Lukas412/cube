use derive_more::Display;
use std::collections::HashSet;
use std::iter;

pub(crate) use self::{
  shape1::new_shape1, shape2::new_shape2, shape3::new_shape3, shape4::new_shape4,
  shape5::new_shape5, shape6::new_shape6, shape7::new_shape7,
};
use crate::block::Block;
use crate::error::ShiftError;

mod shape1;
mod shape2;
mod shape3;
mod shape4;
mod shape5;
mod shape6;
mod shape7;

pub(crate) trait AssociateShapeType {
  fn shape_type() -> ShapeType;
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash, Display)]
pub(crate) enum ShapeType {
  #[default]
  #[display(fmt = "1")]
  Shape1,
  #[display(fmt = "2")]
  Shape2,
  #[display(fmt = "3")]
  Shape3,
  #[display(fmt = "4")]
  Shape4,
  #[display(fmt = "5")]
  Shape5,
  #[display(fmt = "6")]
  Shape6,
  #[display(fmt = "7")]
  Shape7,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Shape<const N: usize> {
  blocks: [Block; N],
}

impl<const N: usize> Shape<N> {
  const fn new(blocks: [Block; N]) -> Self {
    Self { blocks }
  }

  pub(crate) fn combinations(&self) -> HashSet<Self> {
    iter::once(self.clone())
      .flat_map(Self::shifts)
      .flat_map(Self::rotations)
      .map(Self::unify)
      .collect()
  }

  pub(crate) fn unify(mut self) -> Self {
    self.blocks.sort();
    self
  }

  pub(crate) fn blocks(&self) -> &[Block; N] {
    &self.blocks
  }
}

impl<const N: usize> Shape<N> {
  pub(crate) fn rotations(self) -> impl Iterator<Item = Self> {
    self
      .x_axis_rotations()
      .flat_map(Self::y_axis_rotations)
      .flat_map(Self::z_axis_rotations)
  }

  fn x_axis_rotations(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self), |rotation| {
      Some(rotation.clone().rotate_clockwise_around_x())
    })
    .take(4)
  }

  fn y_axis_rotations(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self), |rotation| {
      Some(rotation.clone().rotate_clockwise_around_y())
    })
    .take(4)
  }

  fn z_axis_rotations(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self.rotate_clockwise_around_z()), |rotation| {
      Some(
        rotation
          .clone()
          .rotate_clockwise_around_z()
          .rotate_clockwise_around_z(),
      )
    })
    .take(2)
  }

  fn rotate_clockwise_around_x(mut self) -> Self {
    for block in self.blocks.iter_mut() {
      block.rotate_clockwise_around_x();
    }
    self
  }

  fn rotate_clockwise_around_y(mut self) -> Self {
    for block in self.blocks.iter_mut() {
      block.rotate_clockwise_around_y();
    }
    self
  }

  fn rotate_clockwise_around_z(mut self) -> Self {
    for block in self.blocks.iter_mut() {
      block.rotate_clockwise_around_z();
    }
    self
  }
}

impl<const N: usize> Shape<N> {
  pub(crate) fn shifts(self) -> impl Iterator<Item = Self> {
    self
      .x_shifts()
      .flat_map(Self::y_shifts)
      .flat_map(Self::z_shifts)
  }

  fn x_shifts(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self), |shift| shift.clone().shift_x_up().ok())
  }

  fn shift_x_up(mut self) -> Result<Self, ShiftError> {
    for block in self.blocks.iter_mut() {
      block.shift_x_up()?;
    }
    Ok(self)
  }

  fn y_shifts(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self), |shift| shift.clone().shift_y_up().ok())
  }

  fn shift_y_up(mut self) -> Result<Self, ShiftError> {
    for block in self.blocks.iter_mut() {
      block.shift_y_up()?;
    }
    Ok(self)
  }

  fn z_shifts(self) -> impl Iterator<Item = Self> {
    iter::successors(Some(self), |shift| shift.clone().shift_z_up().ok())
  }

  fn shift_z_up(mut self) -> Result<Self, ShiftError> {
    for block in self.blocks.iter_mut() {
      block.shift_z_up()?;
    }
    Ok(self)
  }
}
