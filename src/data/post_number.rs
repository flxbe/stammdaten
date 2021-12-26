use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "u32", into = "u32")]
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

impl Into<u32> for PostNumber {
    fn into(self) -> u32 {
        self.value
    }
}

impl PostNumber {
    pub fn to_string(&self) -> String {
        let post_number_as_string = format!("{}", self.value);
        return format!("{} {} {}", &post_number_as_string[..3], 
                                   &post_number_as_string[3..6], 
                                   &post_number_as_string[6..9]);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn should_correctly_cast_to_string() {
        let post_number = PostNumber{ value: 123456789};

        assert_eq!("123 456 789", post_number.to_string());
    }
}


