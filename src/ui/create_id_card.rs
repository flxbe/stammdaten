use crate::data::{IdCard, IdCardNumber};
use crate::state::MainState;
use crate::widgets::OutlineButton;
use crate::widgets::{cancel_form, submit_form, Form};
use chrono::{NaiveDate, TimeZone, Utc};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::{Data, Lens};
use std::sync::Arc;

#[derive(Data, Lens, PartialEq, Eq, Clone, Default)]
pub struct FormState {
    pub id: Arc<String>,
    pub valid_until: Arc<String>,
    pub error: Option<String>,
}

pub fn build() -> impl Widget<MainState> {
    let child = Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Personalausweis erstellen"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Nummer")
                .lens(FormState::id),
        )
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Gültig bis")
                .lens(FormState::valid_until),
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
        id: String::from("").into(),
        valid_until: String::from("").into(),
        error: None,
    };

    Form::new(initial_state, child)
        .on_cancel(|_ctx, state: &mut MainState, _env| {
            state.active_process = None;
        })
        .on_submit(|ctx, state: &mut MainState, data: &mut FormState, _env| {
            let card_number = match IdCardNumber::try_from(data.id.as_str()) {
                Ok(value) => value,
                Err(error) => {
                    data.error = Some(format!("{:?}", error));
                    return;
                }
            };

            let valid_until = match NaiveDate::parse_from_str(data.valid_until.as_str(), "%d.%m.%Y")
            {
                Ok(value) => value,
                Err(error) => {
                    data.error = Some(format!("{:?}", error));
                    return;
                }
            };

            let id_card = IdCard {
                card_number,
                expires_after: Utc
                    .from_local_datetime(&valid_until.and_hms(0, 0, 0))
                    .unwrap(),
            };

            state.profile.id_card = Some(id_card);
            state.active_process = None;
            ctx.submit_command(druid::commands::SAVE_FILE);
        })
}
