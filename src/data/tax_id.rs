use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "u64", into = "u64")]
pub struct TaxId {
    value: u64,
}

const MIN_TAX_ID: u64 = 10_000_000_000;
const MAX_TAX_ID: u64 = 99_999_999_999;

impl TryFrom<u64> for TaxId {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // todo: find correct constraints and enforce in here
        if value < MIN_TAX_ID {
            return Err(format!(
                "Tax-ID to small: must have exactly 11 digits {}",
                value
            ));
        } else if value > MAX_TAX_ID {
            return Err(format!(
                "Tax-ID to large: must have exactly 11 digits {}",
                value
            ));
        } else {
            Ok(TaxId { value })
        }
    }
}

impl TryFrom<&str> for TaxId {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let clean: String = value.split_whitespace().collect();

        match clean.parse::<u64>() {
            Ok(number) => TaxId::try_from(number),
            Err(_) => Err(format!("Tax-ID invalid: {}", value)),
        }
    }
}

impl Into<u64> for TaxId {
    fn into(self) -> u64 {
        self.value
    }
}

impl Into<String> for TaxId {
    fn into(self) -> String {
        self.to_string()
    }
}

impl TaxId {
    pub fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}
