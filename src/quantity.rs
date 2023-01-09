use std::error;
use std::fmt;
use std::num;
use std::str;

pub use format::BinaryScale;
pub use format::DecimalScale;
pub use format::Format;
pub use format::InvalidScale;

mod format;
mod impls;

#[derive(Clone, Debug)]
pub struct Quantity {
    int: u128,
    format: Format,
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidQuantity(num::ParseIntError);

impl From<num::ParseIntError> for InvalidQuantity {
    fn from(value: num::ParseIntError) -> Self {
        Self(value)
    }
}

impl fmt::Display for InvalidQuantity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl error::Error for InvalidQuantity {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        Some(&self.0)
    }
}

#[cfg(test)]
mod tests;
