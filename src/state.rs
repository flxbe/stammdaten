//! Specialized state structs to hold the profile data and additional
//! UI specific information in an easy-to-use format.
//!
//! The main advantage of this is to separate the data format between
//! the ser/de and the ui modules.

use crate::data::{BankAccount, IdCard, Name, PostNumber, Profile, SocialSecurityNumber, TaxId};
use druid::im::Vector;
use druid::{Data, Lens};
use std::convert::From;
use std::sync::Arc;

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
impl_data_simple!(BankAccount);

#[derive(Clone, Copy, Data, PartialEq)]
pub enum Nav {
    Home,
    BankAccounts,
}

#[derive(Clone, Data, Lens)]
pub struct ProfileState {
    pub name: Name,
    pub id_card: Option<IdCard>,
    pub social_security_number: Option<SocialSecurityNumber>,
    pub tax_id: Option<TaxId>,
    pub post_number: Option<PostNumber>,
    pub bank_accounts: Vector<BankAccount>,
}

impl ProfileState {
    fn get_profile(&self) -> Profile {
        Profile {
            name: self.name.clone(),
            id_card: self.id_card.clone(),
            social_security_number: self.social_security_number.clone(),
            tax_id: self.tax_id.clone(),
            post_number: self.post_number.clone(),
            bank_accounts: self.bank_accounts.clone().into_iter().collect(),
        }
    }
}

impl From<Profile> for ProfileState {
    fn from(profile: Profile) -> ProfileState {
        ProfileState {
            name: profile.name,
            id_card: profile.id_card,
            social_security_number: profile.social_security_number,
            tax_id: profile.tax_id,
            post_number: profile.post_number,
            bank_accounts: profile.bank_accounts.into_iter().collect(),
        }
    }
}

#[derive(Clone, Copy, Data, PartialEq, Eq, Debug)]
pub enum Process {
    CreateSocialSecurityNumber,
    CreateTaxId,
    CreatePostNumber,
}

#[derive(Clone, Data, Lens)]
pub struct MainState {
    pub profile: ProfileState,
    pub nav: Nav,
    pub active_process: Option<Process>,
}

#[derive(Clone, Data, Lens, Default)]
pub struct CreateState {
    pub first_name: Arc<String>,
    pub last_name: Arc<String>,
}

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub main: Option<MainState>,
    pub create: CreateState,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            main: None,
            create: CreateState::default(),
        }
    }

    pub fn from_profile(profile: Profile) -> AppState {
        AppState {
            main: Some(MainState {
                profile: ProfileState::from(profile),
                nav: Nav::Home,
                active_process: None,
            }),
            create: CreateState::default(),
        }
    }

    pub fn get_profile(&self) -> Profile {
        let main_state = self
            .main
            .as_ref()
            .expect("MainState should not be None when saving profile.");

        main_state.profile.get_profile()
    }
}
