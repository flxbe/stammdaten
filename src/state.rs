//! Specialized state structs to hold the profile data and additional
//! UI specific information in an easy-to-use format.
//!
//! The main advantage of this is to separate the data format between
//! the ser/de and the ui modules.

use crate::data::{
    BankAccount, IdCard, KeyValueItem, Name, PostNumber, Profile, SocialSecurityNumber, TaxId,
};
use crate::ui::{
    create_bank_account, create_id_card, create_key_value_item, create_post_number, create_profile,
    create_social_security_number, create_tax_id,
};
use druid::im::Vector;
use druid::{Data, Lens};
use druid_enums::Matcher;
use std::convert::From;

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
impl_data_simple!(KeyValueItem);

#[derive(Clone, Copy, Data, PartialEq, Eq, Debug)]
pub enum Nav {
    Home,
    BankAccounts,
    Miscellaneous,
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Debug)]
pub struct ProfileState {
    pub name: Name,
    pub id_card: Option<IdCard>,
    pub social_security_number: Option<SocialSecurityNumber>,
    pub tax_id: Option<TaxId>,
    pub post_number: Option<PostNumber>,
    pub bank_accounts: Vector<BankAccount>,
    pub key_value_items: Vector<KeyValueItem>,
}

impl ProfileState {
    pub fn get_profile(&self) -> Profile {
        Profile {
            name: self.name.clone(),
            id_card: self.id_card.clone(),
            social_security_number: self.social_security_number.clone(),
            tax_id: self.tax_id.clone(),
            post_number: self.post_number.clone(),
            bank_accounts: self.bank_accounts.clone().into_iter().collect(),
            key_value_items: self.key_value_items.clone().into_iter().collect(),
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
            key_value_items: profile.key_value_items.into_iter().collect(),
        }
    }
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Debug)]
pub struct HomeState {
    pub profile: ProfileState,
    pub nav: Nav,
}

#[derive(Clone, Data, Lens, PartialEq, Eq, Debug)]
pub struct ProcessState<F> {
    pub home_state: HomeState,
    pub form_state: F,
}

impl<F> From<HomeState> for ProcessState<F>
where
    F: Default,
{
    fn from(home_state: HomeState) -> ProcessState<F> {
        ProcessState {
            home_state,
            form_state: F::default(),
        }
    }
}

pub type CreatePostNumberState = ProcessState<create_post_number::FormState>;
pub type CreateTaxIdState = ProcessState<create_tax_id::FormState>;
pub type CreateIdCardState = ProcessState<create_id_card::FormState>;
pub type CreateSocialSecurityNumberState = ProcessState<create_social_security_number::FormState>;
pub type CreateBankAccountState = ProcessState<create_bank_account::FormState>;
pub type CreateKeyValueItemState = ProcessState<create_key_value_item::FormState>;

#[derive(Clone, PartialEq, Eq, Debug, Data, Matcher)]
pub enum MainState {
    Home(HomeState),
    CreateSocialSecurityNumber(CreateSocialSecurityNumberState),
    CreateTaxId(CreateTaxIdState),
    CreatePostNumber(CreatePostNumberState),
    CreateIdCard(CreateIdCardState),
    CreateBankAccount(CreateBankAccountState),
    CreateKeyValueItem(CreateKeyValueItemState),
}

#[derive(Clone, Data, Matcher)]
pub enum AppState {
    Create(create_profile::FormState),
    Main(MainState),
}

impl AppState {
    pub fn new() -> AppState {
        AppState::Create(create_profile::FormState::default())
    }

    pub fn from_profile(profile: Profile) -> AppState {
        AppState::Main(MainState::Home(HomeState {
            profile: ProfileState::from(profile),
            nav: Nav::Home,
        }))
    }
}
