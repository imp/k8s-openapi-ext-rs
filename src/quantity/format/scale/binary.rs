use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum BinaryScale {
    None = 0,
    Kibi = 10,
    Mebi = 20,
    Gibi = 30,
    Tebi = 40,
    Pebi = 50,
    Exbi = 60,
}

impl BinaryScale {
    pub fn integer_multiplier(&self) -> Option<u128> {
        let exp = *self as u32;
        2_u128.checked_pow(exp)
    }

    fn as_str(&self) -> &str {
        match self {
            Self::None => "",
            Self::Kibi => "Ki",
            Self::Mebi => "Mi",
            Self::Gibi => "Gi",
            Self::Tebi => "Ti",
            Self::Pebi => "Pi",
            Self::Exbi => "Ei",
        }
    }

    fn as_str_alternative(&self) -> &str {
        match self {
            Self::None => "",
            Self::Kibi => "Ki (2^10)",
            Self::Mebi => "Mi (2^20)",
            Self::Gibi => "Gi (2^30)",
            Self::Tebi => "Ti (2^40)",
            Self::Pebi => "Pi (2^50)",
            Self::Exbi => "Ei (2^60)",
        }
    }
}

impl fmt::Display for BinaryScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.as_str_alternative().fmt(f)
        } else {
            self.as_str().fmt(f)
        }
    }
}

impl str::FromStr for BinaryScale {
    type Err = InvalidScale;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Ki" | "KiB" => Ok(Self::Kibi),
            "Mi" | "MiB" => Ok(Self::Mebi),
            "Gi" | "GiB" => Ok(Self::Gibi),
            "Ti" | "TiB" => Ok(Self::Tebi),
            "Pi" | "PiB" => Ok(Self::Pebi),
            "Ei" | "EiB" => Ok(Self::Exbi),
            _ => Err(InvalidScale),
        }
    }
}
