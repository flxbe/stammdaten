use crate::profile::BankAccount;
use crate::social_security_number::SocialSecurityNumber;
use crate::tax_id::TaxId;
use druid::im::Vector;
use druid::{Data, Lens};

#[derive(Clone, Copy, Data, PartialEq)]
pub enum Nav {
    Home,
    BankAccounts,
}

#[derive(Clone, Data, Lens)]
pub struct State {
    pub first_name: String,
    pub last_name: String,
    #[data(same_fn = "PartialEq::eq")]
    pub social_security_number: SocialSecurityNumber,
    #[data(same_fn = "PartialEq::eq")]
    pub tax_id: TaxId,
    pub bank_accounts: Vector<BankAccount>,
    pub nav: Nav,
}
