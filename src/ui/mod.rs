mod create_post_number;
mod create_profile;
mod create_social_security_number;
mod create_tax_id;
mod main;
mod some_lens;

use crate::state::{AppState, Process};
use druid::widget::{ViewSwitcher, Widget, WidgetExt};
use druid::{theme, Data, LensExt};
use some_lens::SomeLens;

#[derive(Data, Eq, PartialEq, Clone, Copy)]
enum Location {
    CreateProfile,
    Main,
    Process(Process),
}

fn get_current_location(state: &AppState) -> Location {
    match state.main.as_ref() {
        Some(main) => match main.active_process {
            Some(process) => Location::Process(process),
            None => Location::Main,
        },
        None => Location::CreateProfile,
    }
}

pub fn build_ui() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |state: &AppState, _env| get_current_location(state),
        |location, _state, _env| match location {
            Location::CreateProfile => Box::new(create_profile::build()),
            Location::Main => Box::new(main::build().lens(AppState::main.then(SomeLens))),
            Location::Process(process) => match process {
                Process::CreateSocialSecurityNumber => Box::new(
                    create_social_security_number::build().lens(AppState::main.then(SomeLens)),
                ),
                Process::CreateTaxId => {
                    Box::new(create_tax_id::build().lens(AppState::main.then(SomeLens)))
                }
                Process::CreatePostNumber => {
                    Box::new(create_post_number::build().lens(AppState::main.then(SomeLens)))
                }
            },
        },
    )
    .background(theme::BACKGROUND_DARK)
    .expand()
}
