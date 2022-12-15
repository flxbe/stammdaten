use crate::data::KeyValueItem;
use crate::widgets::{input, Card, InputState, OutlineButton};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{Data, Lens, Selector};

pub const CANCELED: Selector<()> = Selector::new("app.main.create_key_value_item.canceled");
pub const CREATED: Selector<KeyValueItem> = Selector::new("app.main.create_key_value_item.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    key: InputState,
    value: InputState,
}

pub fn build() -> impl Widget<FormState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(form())
}

fn form() -> impl Widget<FormState> {
    Card::new(
        Flex::column()
            .cross_axis_alignment(CrossAxisAlignment::Start)
            .main_axis_alignment(MainAxisAlignment::Start)
            .with_child(Label::new("Sonstigen Eintrag erstellen").with_text_size(20.0))
            .with_spacer(20.0)
            .with_child(input("Name").lens(FormState::key))
            .with_default_spacer()
            .with_child(input("Wert").lens(FormState::value))
            .with_default_spacer()
            .with_spacer(20.0)
            .with_child(
                Flex::row()
                    .with_child(OutlineButton::new("Erstellen").on_click(
                        |ctx, state: &mut FormState, _env| {
                            let mut has_error = false;

                            state.key.reset_error();
                            if state.key.value.is_empty() {
                                state
                                    .key
                                    .set_error(String::from("Dies ist ein Pflichtfeld."));
                                has_error = true;
                            }

                            state.value.reset_error();
                            if state.value.value.is_empty() {
                                state
                                    .value
                                    .set_error(String::from("Dies ist ein Pflichtfeld."));
                                has_error = true;
                            }

                            if !has_error {
                                let item = KeyValueItem {
                                    key: state.key.value.as_str().into(),
                                    value: state.value.value.as_str().into(),
                                };

                                ctx.submit_notification(CREATED.with(item));
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
