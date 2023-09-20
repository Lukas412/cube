use std::marker::PhantomData;

use duplicate::duplicate_item;

use crate::block::Block;
use crate::cube::Cube;
use crate::shapes::{AssociateShapeType, Shape, ShapeType};

#[derive(Clone, Debug, Default)]
pub(crate) struct CubeBuilder<Step> {
  structure: [[[Option<ShapeType>; 3]; 3]; 3],
  marker: PhantomData<Step>,
}

impl CubeBuilder<Step0> {
  pub(crate) fn new() -> Self {
    Self {
      structure: Default::default(),
      marker: Default::default(),
    }
  }

  pub(crate) fn add_shape_1(&self, shape: &Shape<3>) -> Option<CubeBuilder<Step1>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step1> {
  pub(crate) fn add_shape_2(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step2>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step2> {
  pub(crate) fn add_shape_3(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step3>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step3> {
  pub(crate) fn add_shape_4(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step4>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step4> {
  pub(crate) fn add_shape_5(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step5>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step5> {
  pub(crate) fn add_shape_6(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step6>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step6> {
  pub(crate) fn add_shape_7(&self, shape: &Shape<4>) -> Option<CubeBuilder<Step7>> {
    self.add_shape(shape)
  }
}

impl CubeBuilder<Step7> {
  pub(crate) fn build(&self) -> Option<Cube> {
    let mut structure = [[[ShapeType::default(); 3]; 3]; 3];
    for x in 0..=2 {
      for y in 0..=2 {
        for z in 0..=2 {
          let read = self.structure[x][y][z]?;
          structure[x][y][z] = read;
        }
      }
    }
    Some(Cube::new(structure))
  }
}

impl<Step> CubeBuilder<Step>
where
  Step: Clone,
{
  fn add_shape<const N: usize, NextStep>(&self, shape: &Shape<N>) -> Option<CubeBuilder<NextStep>>
  where
    Step: AssociateShapeType,
    NextStep: Clone,
  {
    if !self.can_fit_shape(shape) {
      return None;
    }

    let mut builder = CubeBuilder {
      structure: self.structure,
      marker: Default::default(),
    };
    for block in shape.blocks() {
      builder.set(block, Step::shape_type());
    }
    Some(builder)
  }

  fn can_fit_shape<const N: usize>(&self, shape: &Shape<N>) -> bool {
    shape.blocks().iter().all(|block| self.get(block).is_none())
  }

  fn set(&mut self, block: &Block, shape_type: ShapeType) {
    self.structure[block.x().usize()][block.y().usize()][block.z().usize()] = Some(shape_type);
  }

  fn get(&self, block: &Block) -> Option<ShapeType> {
    self.structure[block.x().usize()][block.y().usize()][block.z().usize()]
  }
}

#[derive(Clone)]
pub(crate) struct Step0;
#[derive(Clone)]
pub(crate) struct Step1;
#[derive(Clone)]
pub(crate) struct Step2;
#[derive(Clone)]
pub(crate) struct Step3;
#[derive(Clone)]
pub(crate) struct Step4;
#[derive(Clone)]
pub(crate) struct Step5;
#[derive(Clone)]
pub(crate) struct Step6;
#[derive(Clone)]
pub(crate) struct Step7;

#[duplicate_item(
    Step Shape;
    [Step0] [Shape1];
    [Step1] [Shape2];
    [Step2] [Shape3];
    [Step3] [Shape4];
    [Step4] [Shape5];
    [Step5] [Shape6];
    [Step6] [Shape7];
)]
impl AssociateShapeType for Step {
  fn shape_type() -> ShapeType {
    ShapeType::Shape
  }
}
