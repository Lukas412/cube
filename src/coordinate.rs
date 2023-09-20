use crate::error::{OutOfBoundsError, ShiftError};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum Position {
  Number0,
  Number1,
  Number2,
}

impl Position {
  pub(crate) fn invert(&mut self) {
    *self = match self {
      Self::Number0 => Self::Number2,
      Self::Number1 => Self::Number1,
      Self::Number2 => Self::Number0,
    }
  }

  pub(crate) fn increase(&mut self) -> Result<(), OutOfBoundsError> {
    *self = match self {
      Self::Number0 => Self::Number1,
      Self::Number1 => Self::Number2,
      Self::Number2 => return Err(OutOfBoundsError),
    };
    Ok(())
  }

  pub(crate) fn shift_up(&mut self) -> Result<(), ShiftError> {
    self.increase().map_err(Into::into)
  }

  pub(crate) const fn usize(&self) -> usize {
    match self {
      Self::Number0 => 0,
      Self::Number1 => 1,
      Self::Number2 => 2,
    }
  }
}

pub(crate) fn rotate(first: &mut Position, second: &mut Position) {
  first.invert();
  (*first, *second) = (*second, *first);
}
