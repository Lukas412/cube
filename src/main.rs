use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Write;

use itertools::Itertools;

use crate::cube::Cube;
use crate::shapes::Shape;
use crate::{
  cube::CubeBuilder,
  shapes::{new_shape1, new_shape2, new_shape3, new_shape4, new_shape5, new_shape6, new_shape7},
};

fn main() -> io::Result<()> {
  let cubes = compute_all_cubes();
  println!("Number of Cubes generated: {}", cubes.len());

  write_cubes_to_file(&cubes)
}

fn compute_all_cubes() -> Vec<Cube> {
  let combinations1: HashSet<_> = new_shape1().shifts().map(Shape::unify).collect();
  let combinations2 = new_shape2().combinations();
  let combinations3 = new_shape3().combinations();
  let combinations4 = new_shape4().combinations();
  let combinations5 = new_shape5().combinations();
  let combinations6 = new_shape6().combinations();
  let combinations7 = new_shape7().combinations();

  let cube_builder = CubeBuilder::new();
  combinations1
    .iter()
    .filter_map(|shape| cube_builder.add_shape_1(shape))
    .cartesian_product(combinations2.iter())
    .filter_map(|(builder, shape)| builder.add_shape_2(shape))
    .cartesian_product(combinations3.iter())
    .filter_map(|(builder, shape)| builder.add_shape_3(shape))
    .cartesian_product(combinations4.iter())
    .filter_map(|(builder, shape)| builder.add_shape_4(shape))
    .cartesian_product(combinations5.iter())
    .filter_map(|(builder, shape)| builder.add_shape_5(shape))
    .cartesian_product(combinations6.iter())
    .filter_map(|(builder, shape)| builder.add_shape_6(shape))
    .cartesian_product(combinations7.iter())
    .filter_map(|(builder, shape)| builder.add_shape_7(shape))
    .filter_map(|builder| builder.build())
    .collect()
}

fn write_cubes_to_file(cubes: &[Cube]) -> io::Result<()> {
  let output: String = cubes
    .iter()
    .enumerate()
    .map(|(index, cube)| format!("# Cube {index}\n{cube}\n"))
    .collect();
  let mut output_file = File::create("cubes.txt")?;
  output_file.write_all(output.as_bytes())?;

  Ok(())
}

mod block;
mod coordinate;
mod cube;
mod error;
mod shapes;
