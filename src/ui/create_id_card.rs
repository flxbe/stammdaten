use crate::data::{IdCard, IdCardNumber};
use crate::widgets::{input, Card, InputState, OutlineButton};
use chrono::{NaiveDate, TimeZone, Utc};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{Data, Lens, Selector};

pub const CANCELED: Selector<()> = Selector::new("main.create_id_card.canceled");
pub const CREATED: Selector<IdCard> = Selector::new("main.create_id_card.created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    id: InputState,
    valid_until: InputState,
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
            .with_child(Label::new("Personalausweis erstellen").with_text_size(20.0))
            .with_spacer(20.0)
            .with_child(input("Nummer").lens(FormState::id))
            .with_default_spacer()
            .with_child(input("Gültig bis").lens(FormState::valid_until))
            .with_spacer(20.0)
            .with_child(
                Flex::row()
                    .with_child(OutlineButton::new("Erstellen").on_click(
                        |ctx, state: &mut FormState, _env| {
                            let mut has_error = false;

                            state.id.reset_error();
                            let card_number = match IdCardNumber::try_from(state.id.value.as_str())
                            {
                                Ok(value) => Some(value),
                                Err(error) => {
                                    state.id.set_error(format!("{:?}", error));
                                    has_error = true;
                                    None
                                }
                            };

                            state.valid_until.reset_error();
                            let valid_until = match NaiveDate::parse_from_str(
                                state.valid_until.value.as_str(),
                                "%d.%m.%Y",
                            ) {
                                Ok(value) => Some(value),
                                Err(error) => {
                                    state.valid_until.set_error(format!("{:?}", error));
                                    has_error = true;
                                    None
                                }
                            };

                            if !has_error {
                                let id_card = IdCard {
                                    card_number: card_number.unwrap(),
                                    valid_until: Utc
                                        .from_local_datetime(&valid_until.unwrap().and_hms(0, 0, 0))
                                        .unwrap(),
                                };

                                ctx.submit_notification(CREATED.with(id_card));
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
