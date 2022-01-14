use super::id_card::IdCard;
use super::post_number::PostNumber;
use super::social_security_number::SocialSecurityNumber;
use super::tax_id::TaxId;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};
use std::str::FromStr;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct BankAccount {
    pub name: String,
    pub iban: String,
    pub url: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Profile {
    pub first_name: String,
    pub last_name: String,
    pub id_card: IdCard,
    pub social_security_number: SocialSecurityNumber,
    pub post_number: PostNumber,
    pub tax_id: TaxId,
    pub bank_accounts: Vec<BankAccount>,
}

impl Profile {
    pub fn load_from_file(file: &mut File) -> io::Result<Profile> {
        let mut data = String::new();
        file.read_to_string(&mut data)?;

        data.parse()
    }

    pub fn save_to_file(&self, file: &mut File) -> io::Result<()> {
        let data = serde_json::to_string(self)?;
        file.write_all(data.as_ref())?;

        Ok(())
    }
}

impl FromStr for Profile {
    type Err = io::Error;

    fn from_str(json_data: &str) -> io::Result<Profile> {
        let profile: Profile = serde_json::from_str(json_data)?;
        Ok(profile)
    }
}

#[cfg(test)]
mod test {
    use super::super::id_card::IdCardNumber;
    use super::*;
    use chrono::prelude::*;
    use std::io::Seek;

    #[test]
    fn should_correctly_save_the_profile_to_disk() {
        let profile = Profile {
            first_name: "Test".into(),
            last_name: "Name".into(),
            id_card: IdCard {
                card_number: IdCardNumber::try_from("48328FGW9").unwrap(),
                expires_after: Utc::now(),
            },
            social_security_number: SocialSecurityNumber::try_from("50 010101 N012").unwrap(),
            tax_id: TaxId::try_from(12_123_456_789).unwrap(),
            post_number: PostNumber::try_from(123_456_789).unwrap(),
            bank_accounts: vec![BankAccount {
                name: "Some Account Name".into(),
                iban: "DE10 1010 1010 1010 1010 10".into(),
                url: "https://banking.bank.com".into(),
            }],
        };

        let mut file = tempfile::tempfile().unwrap();

        profile.save_to_file(&mut file).unwrap();
        file.rewind().unwrap();
        let loaded_profile = Profile::load_from_file(&mut file).unwrap();

        assert_eq!(profile, loaded_profile);
    }
}
