use derive_more::{Display, Error, From};

#[derive(Debug, Display, Error, From)]
pub(crate) enum ShiftError {
    OutOfBounds(#[error(source)] OutOfBoundsError),
}

#[derive(Debug, Display, Error)]
pub(crate) struct OutOfBoundsError;
