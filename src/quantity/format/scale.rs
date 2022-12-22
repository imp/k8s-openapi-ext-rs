use super::*;

pub use binary::BinaryScale;
pub use decimal::DecimalScale;

mod binary;
mod decimal;

#[derive(Debug)]
pub struct InvalidScale;

impl fmt::Display for InvalidScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "Invalid Scale".fmt(f)
    }
}

impl error::Error for InvalidScale {}
