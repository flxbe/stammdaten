use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "&str", into = "String")]
pub struct PostNumber {
    value: u32,
}

const MIN_POST_NUMBER: u32 = 100_000_000;
const MAX_POST_NUMBER: u32 = 999_999_999;

impl TryFrom<u32> for PostNumber {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        // todo: find correct constraints and enforce in here
        if value < MIN_POST_NUMBER {
            return Err(format!(
                "PostNumber to small: must have exactly 9 digits {}",
                value
            ));
        } else if value > MAX_POST_NUMBER {
            return Err(format!(
                "PostNumber to large: must have exactly 9 digits {}",
                value
            ));
        } else {
            Ok(PostNumber { value })
        }
    }
}

impl TryFrom<&str> for PostNumber {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let clean: String = value.split_whitespace().collect();

        match clean.parse::<u32>() {
            Ok(number) => PostNumber::try_from(number),
            Err(_) => Err(format!("PostNumber invalid: {}", value)),
        }
    }
}

impl Into<u32> for PostNumber {
    fn into(self) -> u32 {
        self.value
    }
}

impl Into<String> for PostNumber {
    fn into(self) -> String {
        self.to_string()
    }
}

impl PostNumber {
    pub fn to_string(&self) -> String {
        let post_number_as_string = format!("{}", self.value);
        return format!(
            "{} {} {}",
            &post_number_as_string[..3],
            &post_number_as_string[3..6],
            &post_number_as_string[6..9]
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn should_correctly_cast_to_string() {
        let post_number = PostNumber::try_from("123 456789").unwrap();

        assert_eq!("123 456 789", post_number.to_string());
    }
}
