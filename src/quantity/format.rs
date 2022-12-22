use super::*;

pub use scale::BinaryScale;
pub use scale::DecimalScale;
pub use scale::InvalidScale;

mod scale;

#[derive(Clone, Copy, Debug)]
pub enum Format {
    DecimalExponent,         // e.g., 12e6
    BinarySI(BinaryScale),   // e.g., 12Mi (12 * 2^20)
    DecimalSI(DecimalScale), // e.g., 12M  (12 * 10^6)
}

impl Format {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DecimalExponent => "DecimalExponent",
            Self::BinarySI(_) => "BinarySI",
            Self::DecimalSI(_) => "DecimalSI",
        }
    }

    pub fn integer_multiplier(&self) -> Option<i64> {
        let multiplier = match self {
            Self::DecimalExponent => None,
            Self::BinarySI(b) => b.integer_multiplier(),
            Self::DecimalSI(d) => d.integer_multiplier(),
        };
        dbg!(multiplier).and_then(|m| i64::try_from(m).ok())
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
