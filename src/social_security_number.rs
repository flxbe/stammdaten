use serde::{Deserialize, Serialize};
use std::char;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
enum RegionCode {
    BundBraunschweigHannover = 50,
}

impl TryFrom<&str> for RegionCode {
    type Error = String;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "50" => Ok(Self::BundBraunschweigHannover),
            _ => Err(format!("Invalid Region Code: {}", v)),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(try_from = "&str", into = "String")]
pub struct SocialSecurityNumber {
    region_code: RegionCode,
    day_of_birth: u8,
    month_of_birth: u8,
    year_of_birth: u8,
    first_letter_of_birth_name: char,
    serial_number: u8,
    check_digit: u8,
}

impl TryFrom<&str> for SocialSecurityNumber {
    type Error = String;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        let clean: String = v.split_whitespace().collect();
        if clean.len() != 12 {
            return Err(format!("Social security number has wrong length: {}", v));
        }

        let region_code = RegionCode::try_from(&clean[0..2])?;

        let day_of_birth = clean[2..4]
            .parse::<u8>()
            .map_err(|_e| format!("Invalid day of birth: {}", v))?;

        let month_of_birth = clean[4..6]
            .parse::<u8>()
            .map_err(|_e| format!("Invalid month of birth: {}", v))?;

        let year_of_birth = clean[6..8]
            .parse::<u8>()
            .map_err(|_e| format!("Invalid year of birth: {}", v))?;

        // todo: check letter is between A..Z
        let first_letter_of_birth_name = clean.chars().nth(8).unwrap();

        let serial_number = clean[9..11]
            .parse::<u8>()
            .map_err(|_e| format!("Invalid serial number: {}", v))?;

        let check_digit = clean[11..12]
            .parse::<u8>()
            .map_err(|_e| format!("Invalid check digit: {}", v))?;

        // todo: validate check digit
        // https://de.wikipedia.org/wiki/Versicherungsnummer#Berechnung_der_Pr%C3%BCfziffer

        Ok(SocialSecurityNumber {
            region_code,
            day_of_birth,
            month_of_birth,
            year_of_birth,
            first_letter_of_birth_name,
            serial_number,
            check_digit,
        })
    }
}

impl SocialSecurityNumber {
    pub fn to_string(&self) -> String {
        return format!(
            "{:02} {:02}{:02}{:02} {}{:02}{}",
            self.region_code as u8,
            self.day_of_birth,
            self.month_of_birth,
            self.year_of_birth,
            self.first_letter_of_birth_name,
            self.serial_number,
            self.check_digit
        );
    }
}

impl Into<String> for SocialSecurityNumber {
    fn into(self) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_parse_valid_social_security_numbers() {
        SocialSecurityNumber::try_from("50 01 0101 N012").unwrap();
        SocialSecurityNumber::try_from("50010101N012").unwrap();
        SocialSecurityNumber::try_from("50  010101N012").unwrap();
    }

    #[test]
    fn should_correctly_print_the_social_security_number() {
        let value = SocialSecurityNumber::try_from("50 010 101 N012").unwrap();

        assert_eq!(value.to_string(), "50 010101 N012")
    }
}
