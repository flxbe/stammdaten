use serde::{Deserialize, Serialize};
use std::char;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Copy, Clone, PartialEq, Eq, Debug)]
enum RegionCode {
    MecklenburgVorpommern = 2,
    Thueringen = 3,
    Brandenburg = 4,
    SachsenAnhalt = 8,
    Sachsen = 9,
    Hannover = 10,
    Westfalen = 11,
    Hessen = 12,
    Rheinprovinz = 13,
    Oberbayern = 14,
    NiederbayernOberpfalz = 15,
    RheinlandPfalz = 16,
    Saarland = 17,
    OberMittelFranken = 18,
    Hamburg = 19,
    Unterfranken = 20,
    Schwaben = 21,
    Wuerttemberg = 23,
    Baden = 24,
    Berlin = 25,
    SchleswigHolstein = 26,
    OldenburgBremen = 28,
    Braunschweig = 29,
    KnappschaftBahnSeeBahn = 38,
    KnappschaftBahnSeeSeefahrt = 39,
    ZentraleAnlagestelleAltersvermoegen = 40,
    BundMecklenburgVorpommern = 42,
    BundThueringen = 43,
    BundBrandenburg = 44,
    BundSachsenAnhalt = 48,
    BundSachsen = 49,
    BundHannover = 50,
    BundWestfalen = 51,
    BundHessen = 52,
    BundRheinprovinz = 53,
    BundOberbayern = 54,
    BundNiederbayernOberpfalz = 55,
    BundRheinlandPfalz = 56,
    BundSaarland = 57,
    BundOberMittelFranken = 58,
    BundHamburg = 59,
    BundUnterfranken = 60,
    BundSchwaben = 61,
    BundWuerttemberg = 63,
    BundBaden = 64,
    BundBerlin = 65,
    BundSchleswigHolstein = 66,
    BundOldenburgBremen = 68,
    BundBraunschweig = 69,
    BundKanppschaftBahnSeeBahn = 78,
    BundKnappschaftBahnSeeSeefahrt = 79,
    KnappschaftBahnSeeBBHNWSH = 80,
    KnappschaftBahnSeeHR = 81,
    KnappschaftBahnSeeBWBRPS = 82,
    KnappschaftBahnSeeBMVSAST = 89,
}

impl TryFrom<&str> for RegionCode {
    type Error = String;

    fn try_from(v: &str) -> Result<Self, Self::Error> {
        match v {
            "02" => Ok(Self::MecklenburgVorpommern),
            "03" => Ok(Self::Thueringen),
            "04" => Ok(Self::Brandenburg),
            "08" => Ok(Self::SachsenAnhalt),
            "09" => Ok(Self::Sachsen),
            "10" => Ok(Self::Hannover),
            "11" => Ok(Self::Westfalen),
            "12" => Ok(Self::Hessen),
            "13" => Ok(Self::Rheinprovinz),
            "14" => Ok(Self::Oberbayern),
            "15" => Ok(Self::NiederbayernOberpfalz),
            "16" => Ok(Self::RheinlandPfalz),
            "17" => Ok(Self::Saarland),
            "18" => Ok(Self::OberMittelFranken),
            "19" => Ok(Self::Hamburg),
            "20" => Ok(Self::Unterfranken),
            "21" => Ok(Self::Schwaben),
            "23" => Ok(Self::Wuerttemberg),
            "24" => Ok(Self::Baden),
            "25" => Ok(Self::Berlin),
            "26" => Ok(Self::SchleswigHolstein),
            "28" => Ok(Self::OldenburgBremen),
            "29" => Ok(Self::Braunschweig),
            "38" => Ok(Self::KnappschaftBahnSeeBahn),
            "39" => Ok(Self::KnappschaftBahnSeeSeefahrt),
            "40" => Ok(Self::ZentraleAnlagestelleAltersvermoegen),
            "42" => Ok(Self::BundMecklenburgVorpommern),
            "43" => Ok(Self::BundThueringen),
            "44" => Ok(Self::BundBrandenburg),
            "48" => Ok(Self::BundSachsenAnhalt),
            "49" => Ok(Self::BundSachsen),
            "50" => Ok(Self::BundHannover),
            "51" => Ok(Self::BundWestfalen),
            "52" => Ok(Self::BundHessen),
            "53" => Ok(Self::BundRheinprovinz),
            "54" => Ok(Self::BundOberbayern),
            "55" => Ok(Self::BundNiederbayernOberpfalz),
            "56" => Ok(Self::BundRheinlandPfalz),
            "57" => Ok(Self::BundSaarland),
            "58" => Ok(Self::BundOberMittelFranken),
            "59" => Ok(Self::BundHamburg),
            "60" => Ok(Self::BundUnterfranken),
            "61" => Ok(Self::BundSchwaben),
            "63" => Ok(Self::BundWuerttemberg),
            "64" => Ok(Self::BundBaden),
            "65" => Ok(Self::BundBerlin),
            "66" => Ok(Self::BundSchleswigHolstein),
            "68" => Ok(Self::BundOldenburgBremen),
            "69" => Ok(Self::BundBraunschweig),
            "78" => Ok(Self::BundKanppschaftBahnSeeBahn),
            "79" => Ok(Self::BundKnappschaftBahnSeeSeefahrt),
            "80" => Ok(Self::KnappschaftBahnSeeBBHNWSH),
            "81" => Ok(Self::KnappschaftBahnSeeHR),
            "82" => Ok(Self::KnappschaftBahnSeeBWBRPS),
            "89" => Ok(Self::KnappschaftBahnSeeBMVSAST),
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
    fn should_correctly_print_the_social_security_number_for_bund_braunschweig_hannover() {
        let value = SocialSecurityNumber::try_from("50 010 101 N012").unwrap();

        assert_eq!(value.to_string(), "50 010101 N012")
    }

    #[test]
    fn should_correctly_print_the_social_security_number_for_schleswig_holstein() {
        let value = SocialSecurityNumber::try_from("26 010 101 N012").unwrap();

        assert_eq!(value.to_string(), "26 010101 N012")
    }
}
