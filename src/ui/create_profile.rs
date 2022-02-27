use crate::data::Profile;
use crate::widgets::{input, Card, InputState, OutlineButton};
use druid::widget::{CrossAxisAlignment, Flex, Label, MainAxisAlignment, Widget, WidgetExt};
use druid::{Data, Lens, Selector};

pub const PROFILE_CREATED: Selector<Profile> = Selector::new("app.main.profile_created");

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct FormState {
    first_name: InputState,
    last_name: InputState,
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
            .must_fill_main_axis(true)
            .cross_axis_alignment(CrossAxisAlignment::Center)
            .main_axis_alignment(MainAxisAlignment::Center)
            .with_child(Label::new("Profil erstellen").with_text_size(20.0))
            .with_spacer(20.0)
            .with_child(input("Vorname").lens(FormState::first_name))
            .with_default_spacer()
            .with_child(input("Nachname").lens(FormState::last_name))
            .with_spacer(20.0)
            .with_child(OutlineButton::new("Erstellen").on_click(
                |ctx, state: &mut FormState, _env| {
                    let mut has_error = false;

                    state.first_name.reset_error();
                    if state.first_name.value.is_empty() {
                        state
                            .first_name
                            .set_error(String::from("Fist name must not be empty"));
                        has_error = true;
                    }

                    state.last_name.reset_error();
                    if state.last_name.value.is_empty() {
                        state
                            .last_name
                            .set_error(String::from("Last name must not be empty"));
                        has_error = true;
                    }

                    if !has_error {
                        let first_name = state.first_name.value.as_str();
                        let last_name = state.last_name.value.as_str();
                        let profile = Profile::new(first_name.into(), last_name.into());

                        ctx.submit_notification(PROFILE_CREATED.with(profile));
                    }
                },
            )),
    )
    .fix_width(400.0)
}
