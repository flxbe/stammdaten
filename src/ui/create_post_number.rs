use crate::data::PostNumber;
use crate::widgets::{input, InputState, OutlineButton};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{theme, Color, Data, Lens, Selector};

pub const CANCELED: Selector<()> = Selector::new("app.main.create_post_number.canceled");
pub const CREATED: Selector<PostNumber> = Selector::new("app.main.create_post_number.created");

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
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(Label::new("Postnummer erstellen").with_text_size(20.0))
        .with_spacer(20.0)
        .with_child(input("Postnummer").lens(FormState::input))
        .with_spacer(20.0)
        .with_child(
            Flex::row()
                .with_child(OutlineButton::new("Erstellen").on_click(
                    |ctx, state: &mut FormState, _env| {
                        state.input.reset_error();

                        let value = state.input.value.as_str();
                        match PostNumber::try_from(value) {
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
        )
        .padding(40.0)
        .fix_width(400.0)
        .border(theme::BORDER_LIGHT, 1.0)
        .rounded(4.0)
}
