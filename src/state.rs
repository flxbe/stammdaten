//! Specialized state structs to hold the profile data and additional
//! UI specific information in an easy-to-use format.
//!
//! The main advantage of this is to separate the data format between
//! the ser/de and the ui modules.

use crate::data::{BankAccount, IdCard, Name, PostNumber, Profile, SocialSecurityNumber, TaxId};
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
    pub name: Name,
    pub id_card: Option<IdCard>,
    pub social_security_number: Option<SocialSecurityNumber>,
    pub tax_id: Option<TaxId>,
    pub post_number: Option<PostNumber>,
    pub bank_accounts: Vector<BankAccountState>,
}

/// Macro to impl the `Data` trait for structs with the `Eq` trait.
macro_rules! impl_data_simple {
    ($t:ty) => {
        impl Data for $t {
            fn same(&self, other: &Self) -> bool {
                self == other
            }
        }
    };
}

impl_data_simple!(Name);
impl_data_simple!(IdCard);
impl_data_simple!(SocialSecurityNumber);
impl_data_simple!(TaxId);
impl_data_simple!(PostNumber);

impl From<Profile> for ProfileState {
    fn from(profile: Profile) -> ProfileState {
        ProfileState {
            name: profile.name,
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
