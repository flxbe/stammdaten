//! Specialized state structs to hold the profile data and additional
//! UI specific information in an easy-to-use format.
//!
//! The main advantage of this is to separate the data format between
//! the ser/de and the ui modules.

use crate::data::{BankAccount, IdCard, PostNumber, Profile, SocialSecurityNumber, TaxId};
use druid::im::Vector;
use druid::{Data, Lens};
use std::convert::From;

#[derive(Clone, Copy, Data, PartialEq)]
pub enum Nav {
    Home,
    BankAccounts,
}

#[derive(Clone, Data, Lens)]
pub struct BankAccountState {
    pub name: String,
    pub iban: String,
    pub url: String,
}

impl From<BankAccount> for BankAccountState {
    fn from(account: BankAccount) -> BankAccountState {
        BankAccountState {
            name: account.name,
            iban: account.iban,
            url: account.url,
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct ProfileState {
    pub first_name: String,
    pub last_name: String,
    #[data(same_fn = "PartialEq::eq")]
    pub id_card: IdCard,
    #[data(same_fn = "PartialEq::eq")]
    pub social_security_number: SocialSecurityNumber,
    #[data(same_fn = "PartialEq::eq")]
    pub tax_id: TaxId,
    pub post_number: Option<PostNumber>,
    pub bank_accounts: Vector<BankAccountState>,
}

impl Data for PostNumber {
    fn same(&self, right: &Self) -> bool {
        return self == right;
    }
}

impl From<Profile> for ProfileState {
    fn from(profile: Profile) -> ProfileState {
        ProfileState {
            first_name: profile.first_name,
            last_name: profile.last_name,
            id_card: profile.id_card,
            social_security_number: profile.social_security_number,
            tax_id: profile.tax_id,
            post_number: profile.post_number,
            bank_accounts: profile
                .bank_accounts
                .into_iter()
                .map(|account| BankAccountState::from(account))
                .collect(),
        }
    }
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub profile: ProfileState,
    pub nav: Nav,
}
