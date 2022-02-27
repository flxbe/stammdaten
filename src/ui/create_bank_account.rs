use crate::data::BankAccount;
use crate::widgets::{input, Card, InputState, OutlineButton};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{Data, Lens, Selector};

pub const CANCELED: Selector<()> = Selector::new("main.create_bank_account.canceled");
pub const CREATED: Selector<BankAccount> = Selector::new("main.create_bank_account.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    pub name: InputState,
    pub iban: InputState,
    pub url: InputState,
}

pub fn build() -> impl Widget<FormState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(form())
}

pub fn form() -> impl Widget<FormState> {
    Card::new(
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(Label::new("Bankkonto erstellen").with_text_size(20.0))
            .with_spacer(20.0)
            .with_child(input("Name").lens(FormState::name))
            .with_default_spacer()
            .with_child(input("IBAN").lens(FormState::iban))
            .with_default_spacer()
            .with_child(input("URL").lens(FormState::url))
            .with_spacer(20.0)
            .with_child(
                Flex::row()
                    .with_child(OutlineButton::new("Erstellen").on_click(
                        |ctx, state: &mut FormState, _env| {
                            let mut has_error = false;

                            state.name.reset_error();
                            if state.name.value.is_empty() {
                                state.name.set_error(String::from("Name must not be empty"));
                                has_error = true;
                            }

                            state.iban.reset_error();
                            if state.iban.value.is_empty() {
                                state.iban.set_error(String::from("IBAN must not be empty"));
                                has_error = true;
                            }

                            state.url.reset_error();
                            if state.url.value.is_empty() {
                                state.url.set_error(String::from("URL must not be empty"));
                                has_error = true;
                            }

                            if !has_error {
                                let bank_account = BankAccount {
                                    name: state.name.value.as_str().into(),
                                    iban: state.iban.value.as_str().into(),
                                    url: state.iban.value.as_str().into(),
                                };

                                ctx.submit_notification(CREATED.with(bank_account));
                            }
                        },
                    ))
                    .with_default_spacer()
                    .with_child(
                        OutlineButton::new("Abbrechen").on_click(|ctx, _state, _env| {
                            ctx.submit_notification(CANCELED);
                        }),
                    ),
            ),
    )
    .fix_width(400.0)
}
