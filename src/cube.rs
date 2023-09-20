use crate::shapes::ShapeType;
use std::fmt::{Display, Formatter};

pub(crate) use builder::CubeBuilder;

mod builder;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Cube {
  structure: [[[ShapeType; 3]; 3]; 3],
}

impl Cube {
  pub(crate) fn new(structure: [[[ShapeType; 3]; 3]; 3]) -> Self {
    Self { structure }
  }
}

impl Display for Cube {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    let plane0 = self.structure[0];
    let plane1 = self.structure[1];
    let plane2 = self.structure[2];
    writeln!(
      f,
      "1. {} {} {}  2. {} {} {}  3. {} {} {} ",
      plane0[0][0],
      plane0[0][1],
      plane0[0][2],
      plane1[0][0],
      plane1[0][1],
      plane1[0][2],
      plane2[0][0],
      plane2[0][1],
      plane2[0][2]
    )?;
    writeln!(
      f,
      "   {} {} {}     {} {} {}     {} {} {} ",
      plane0[1][0],
      plane0[1][1],
      plane0[1][2],
      plane1[1][0],
      plane1[1][1],
      plane1[1][2],
      plane2[1][0],
      plane2[1][1],
      plane2[1][2]
    )?;
    writeln!(
      f,
      "   {} {} {}     {} {} {}     {} {} {} ",
      plane0[2][0],
      plane0[2][1],
      plane0[2][2],
      plane1[2][0],
      plane1[2][1],
      plane1[2][2],
      plane2[2][0],
      plane2[2][1],
      plane2[2][2]
    )?;
    writeln!(f)
  }
}
