use super::*;

impl Quantity {
    pub fn as_int64(&self) -> Option<i64> {
        let int = i64::try_from(self.int).ok()?;
        let multiplier = self.format.integer_multiplier()?;
        int.checked_mul(multiplier)
    }
}

impl str::FromStr for Quantity {
    type Err = InvalidQuantity;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let length = text.len();

        // First check two-letter suffixes
        if length > 2 {
            let (number, suffix) = text.split_at(length - 2);
            if let Ok(scale) = suffix.parse() {
                let format = Format::BinarySI(scale);
                if let Ok(int) = number.parse() {
                    return Ok(Self { int, format });
                }
            }
        }

        // Then check single letter suffixes
        if length > 1 {
            let (number, suffix) = text.split_at(length - 1);
            if let Ok(scale) = suffix.parse() {
                let format = Format::DecimalSI(scale);
                if let Ok(int) = number.parse() {
                    return Ok(Self { int, format });
                }
            }
        }

        // TODO Add check for exponential notation 'e'|'E'

        // And finally try without suffix
        let int = text.parse()?;
        let format = Format::DecimalSI(DecimalScale::None);
        Ok(Self { int, format })
    }
}

impl From<Quantity> for Option<i64> {
    fn from(value: Quantity) -> Self {
        value.into()
    }
}

impl From<Quantity> for Option<u64> {
    fn from(value: Quantity) -> Self {
        value.as_int64()?.try_into().ok()
    }
}
