use chrono::prelude::*;
use chrono::Duration;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
#[serde(try_from = "&str", into = "String")]
pub struct IdCardNumber {
    value: String,
}

impl TryFrom<&str> for IdCardNumber {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        // See: https://www.bmi.bund.de/SharedDocs/downloads/DE/veroeffentlichungen/themen/moderne-verwaltung/ausweise/personalausweis-seriennummer.html
        static VALID_ID_NUMBER_CHARS: [char; 26] = [
            'C', 'F', 'G', 'H', 'J', 'K', 'L', 'M', 'N', 'P', 'R', 'T', 'V', 'W', 'X', 'Y', 'Z',
            '1', '2', '3', '4', '5', '6', '7', '8', '9',
        ];

        if !value.chars().all(|c| VALID_ID_NUMBER_CHARS.contains(&c)) {
            Err(format!("Invalid IdCardNumber: {}", value))
        } else {
            Ok(IdCardNumber {
                value: String::from(value),
            })
        }
    }
}

impl Into<String> for IdCardNumber {
    fn into(self) -> String {
        self.value
    }
}

impl IdCardNumber {
    pub fn to_string(&self) -> String {
        self.value.clone()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct IdCard {
    pub card_number: IdCardNumber,
    pub expires_after: DateTime<Utc>,
}

impl IdCard {
    pub fn time_until_expiration(&self) -> Duration {
        self.expires_after - Utc::now()
    }

    pub fn has_expired(&self) -> bool {
        self.time_until_expiration() <= Duration::zero()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_only_accept_valid_id_card_numbers() {
        IdCardNumber::try_from("9321CFG").unwrap();
    }

    #[test]
    fn should_reject_invalid_id_card_numbers() {
        let value = IdCardNumber::try_from("0O");

        assert_eq!(value.is_err(), true);
    }

    #[test]
    fn should_correctly_detect_expired_id_cards() {
        let card = create_id_card(Utc::now() - Duration::minutes(1));

        assert_eq!(card.has_expired(), true);
    }

    #[test]
    fn should_correctly_detect_unexpired_id_cards() {
        let card = create_id_card(Utc::now() + Duration::minutes(1));

        assert_eq!(card.has_expired(), false);
    }

    fn create_id_card(expires_after: DateTime<Utc>) -> IdCard {
        return IdCard {
            card_number: IdCardNumber::try_from("123").unwrap(),
            expires_after,
        };
    }
}
