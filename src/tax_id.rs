use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "u64", into = "u64")]
pub struct TaxId {
    value: u64,
}

impl TryFrom<u64> for TaxId {
    type Error = String;

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        // todo: find correct constraints and enforce in here
        Ok(TaxId { value })
    }
}

impl Into<u64> for TaxId {
    fn into(self) -> u64 {
        self.value
    }
}

impl TaxId {
    pub fn to_string(&self) -> String {
        return format!("{}", self.value);
    }
}
