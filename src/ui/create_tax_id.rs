use crate::data::TaxId;
use crate::widgets::{input, Card, InputState, OutlineButton};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{Data, Lens, Selector};

pub const CANCELED: Selector<()> = Selector::new("app.main.create_tax_id.canceled");
pub const CREATED: Selector<TaxId> = Selector::new("app.main.create_tax_id.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    input: InputState,
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
            .with_child(Label::new("Steuer-ID erstellen").with_text_size(20.0))
            .with_spacer(20.0)
            .with_child(input("Nummer").lens(FormState::input))
            .with_default_spacer()
            .with_spacer(20.0)
            .with_child(
                Flex::row()
                    .with_child(OutlineButton::new("Erstellen").on_click(
                        |ctx, state: &mut FormState, _env| {
                            state.input.reset_error();

                            match TaxId::try_from(state.input.value.as_str()) {
                                Ok(value) => {
                                    ctx.submit_notification(CREATED.with(value));
                                }
                                Err(error) => {
                                    state.input.set_error(format!("{:?}", error));
                                }
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
