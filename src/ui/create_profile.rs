use crate::data::Profile;
use crate::state::CreateState;
use crate::widgets::OutlineButton;
use druid::widget::{
    CrossAxisAlignment, Flex, Label, MainAxisAlignment, TextBox, Widget, WidgetExt,
};
use druid::Selector;

pub const PROFILE_CREATED: Selector<Profile> = Selector::new("app.main.profile_created");

pub fn build() -> impl Widget<CreateState> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::Center)
        .with_child(Label::new("Profil erstellen"))
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Vorname")
                .lens(CreateState::first_name),
        )
        .with_default_spacer()
        .with_child(
            TextBox::new()
                .with_placeholder("Nachname")
                .lens(CreateState::last_name),
        )
        .with_default_spacer()
        .with_child(OutlineButton::new("Erstellen").on_click(
            |ctx, state: &mut CreateState, _env| {
                let profile = Profile::new(
                    state.first_name.as_str().into(),
                    state.last_name.as_str().into(),
                );

                ctx.submit_notification(PROFILE_CREATED.with(profile));
            },
        ))
}
