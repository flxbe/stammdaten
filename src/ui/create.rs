use crate::data::Profile;
use crate::state::{AppState, CreateState};
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::LensExt;

pub fn build_create_window() -> impl Widget<AppState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Profil erstellen"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Vorname")
                .lens(AppState::create.then(CreateState::first_name)),
        )
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Nachname")
                .lens(AppState::create.then(CreateState::last_name)),
        )
        .with_default_spacer()
        .with_child(
            Button::new("Erstellen").on_click(|_ctx, state: &mut AppState, _env| {
                let profile = Profile::new(
                    state.create.first_name.as_str().into(),
                    state.create.last_name.as_str().into(),
                );

                *state = AppState::from_profile(profile);
            }),
        )
}
