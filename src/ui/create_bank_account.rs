use crate::data::BankAccount;
use crate::widgets::OutlineButton;
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::{Data, Lens, Selector};
use std::sync::Arc;

pub const CANCELED: Selector<()> = Selector::new("main.create_bank_account.canceled");
pub const CREATED: Selector<BankAccount> = Selector::new("main.create_bank_account.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    pub name: Arc<String>,
    pub iban: Arc<String>,
    pub url: Arc<String>,
    pub error: Option<String>,
}

pub fn build() -> impl Widget<FormState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Bankkonto erstellen"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Name")
                .lens(FormState::name),
        )
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("IBAN")
                .lens(FormState::iban),
        )
        .with_default_spacer()
        .with_child(TextBox::new().with_placeholder("URL").lens(FormState::url))
        .with_default_spacer()
        .with_child(Label::dynamic(|state: &FormState, _env| {
            state
                .error
                .as_ref()
                .map(|value| value.to_string())
                .unwrap_or_else(|| String::from(""))
        }))
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Abbrechen").on_click(|ctx, _state, _env| {
                ctx.submit_notification(CANCELED);
            }),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Erstellen").on_click(|ctx, state: &mut FormState, _env| {
                let name = state.name.as_str();
                if name.is_empty() {
                    state.error = Some("Name must not be empty".into());
                    return;
                }

                let iban = state.iban.as_str();
                if iban.is_empty() {
                    state.error = Some("IBAN must not be empty".into());
                    return;
                }

                let url = state.url.as_str();
                if url.is_empty() {
                    state.error = Some("URL must not be empty".into());
                    return;
                }

                let bank_account = BankAccount {
                    name: name.into(),
                    iban: iban.into(),
                    url: url.into(),
                };

                ctx.submit_notification(CREATED.with(bank_account));
            }),
        )
}
