use crate::data::PostNumber;
use crate::widgets::OutlineButton;
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::{Data, Lens, Selector};
use std::sync::Arc;

pub const CANCELED: Selector<()> = Selector::new("main.create_post_number.canceled");
pub const CREATED: Selector<PostNumber> = Selector::new("main.create_post_number.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    pub value: Arc<String>,
    pub error: Option<String>,
}

pub fn build() -> impl Widget<FormState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Postnummer erstellen"))
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
                ctx.submit_notification(CANCELED);
            }),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Erstellen").on_click(|ctx, state: &mut FormState, _env| {
                let value = state.value.as_str();
                match PostNumber::try_from(value) {
                    Ok(value) => {
                        ctx.submit_notification(CREATED.with(value));
                    }
                    Err(error) => {
                        state.error = Some(format!("{:?}", error));
                    }
                }
            }),
        )
}
