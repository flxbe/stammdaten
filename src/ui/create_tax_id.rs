use crate::data::TaxId;
use crate::state::HomeState;
use crate::widgets::OutlineButton;
use crate::widgets::{cancel_form, submit_form, Form};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::{Data, Lens, Selector};
use std::sync::Arc;

pub const CANCELED: Selector<()> = Selector::new("main.create_tax_id.canceled");
pub const CREATED: Selector<TaxId> = Selector::new("main.create_tax_id.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default)]
pub struct FormState {
    pub value: Arc<String>,
    pub error: Option<String>,
}

pub fn build() -> impl Widget<HomeState> {
    let child = Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Steuer-ID erstellen"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Nummer")
                .lens(FormState::value),
        )
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
        value: String::from("").into(),
        error: None,
    };

    Form::new(initial_state, child)
        .on_cancel(|ctx, _state: &mut HomeState, _env| {
            ctx.submit_notification(CANCELED);
        })
        .on_submit(|ctx, _state: &mut HomeState, data: &mut FormState, _env| {
            let value = data.value.as_str();
            match TaxId::try_from(value) {
                Ok(value) => {
                    ctx.submit_notification(CREATED.with(value));
                }
                Err(error) => {
                    data.error = Some(format!("{:?}", error));
                }
            }
        })
}
