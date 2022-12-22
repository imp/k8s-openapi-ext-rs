use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecimalScale {
    Nano = -9,
    Micro = -6,
    Milli = -3,
    None = 0,
    Kilo = 3,
    Mega = 6,
    Giga = 9,
    Tera = 12,
    Peta = 15,
    Exa = 18,
}

impl DecimalScale {
    pub fn integer_multiplier(&self) -> Option<u128> {
        match self {
            Self::None => Some(1),
            Self::Kilo => Some(1_000),
            Self::Mega => Some(1_000_000),
            Self::Giga => Some(1_000_000_000),
            Self::Tera => Some(1_000_000_000_000),
            Self::Peta => Some(1_000_000_000_000_000),
            Self::Exa => Some(1_000_000_000_000_000_000),
            _ => None,
        }
    }

    fn as_str(&self) -> &str {
        match self {
            Self::Nano => "n",
            Self::Micro => "µ",
            Self::Milli => "m",
            Self::None => "",
            Self::Kilo => "k",
            Self::Mega => "M",
            Self::Giga => "G",
            Self::Tera => "T",
            Self::Peta => "P",
            Self::Exa => "E",
        }
    }

    fn as_str_alternative(&self) -> &str {
        match self {
            Self::Nano => "n (10^-9)",
            Self::Micro => "µ (10^-6)",
            Self::Milli => "m (10^-3)",
            Self::None => "",
            Self::Kilo => "k (10^3)",
            Self::Mega => "M (10^6)",
            Self::Giga => "G (10^9)",
            Self::Tera => "T (10^12)",
            Self::Peta => "P (10^15)",
            Self::Exa => "E (10^18)",
        }
    }
}

impl fmt::Display for DecimalScale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            self.as_str_alternative().fmt(f)
        } else {
            self.as_str().fmt(f)
        }
    }
}

impl str::FromStr for DecimalScale {
    type Err = InvalidScale;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n" => Ok(Self::Nano),
            "u" | "µ" => Ok(Self::Micro),
            "m" => Ok(Self::Milli),
            "k" => Ok(Self::Kilo),
            "M" => Ok(Self::Mega),
            "G" => Ok(Self::Giga),
            "T" => Ok(Self::Tera),
            "P" => Ok(Self::Peta),
            "E" => Ok(Self::Exa),
            _ => Err(InvalidScale),
        }
    }
}
