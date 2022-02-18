use crate::data::BankAccount;
use crate::state::HomeState;
use crate::widgets::OutlineButton;
use crate::widgets::{cancel_form, submit_form, Form};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::{Data, Lens, Selector};
use std::sync::Arc;

pub const CANCELED: Selector<()> = Selector::new("main.create_bank_account.canceled");
pub const CREATED: Selector<BankAccount> = Selector::new("main.create_bank_account.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default)]
pub struct FormState {
    pub name: Arc<String>,
    pub iban: Arc<String>,
    pub url: Arc<String>,
    pub error: Option<String>,
}

pub fn build() -> impl Widget<HomeState> {
    let child = Flex::column()
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
                cancel_form(ctx);
            }),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Erstellen").on_click(|ctx, _state, _env| {
                submit_form(ctx);
            }),
        );

    let initial_state = FormState {
        name: String::from("").into(),
        iban: String::from("").into(),
        url: String::from("").into(),
        error: None,
    };

    Form::new(initial_state, child)
        .on_cancel(|ctx, _state: &mut HomeState, _env| {
            ctx.submit_notification(CANCELED);
        })
        .on_submit(|ctx, _state: &mut HomeState, data: &mut FormState, _env| {
            let name = data.name.as_str();
            if name.is_empty() {
                data.error = Some("Name must not be empty".into());
                return;
            }

            let iban = data.iban.as_str();
            if iban.is_empty() {
                data.error = Some("IBAN must not be empty".into());
                return;
            }

            let url = data.url.as_str();
            if url.is_empty() {
                data.error = Some("URL must not be empty".into());
                return;
            }

            let bank_account = BankAccount {
                name: name.into(),
                iban: iban.into(),
                url: url.into(),
            };

            ctx.submit_notification(CREATED.with(bank_account));
        })
}
