mod create;
mod main;
mod some_lens;

use crate::state::AppState;
use create::build_create_window;
use druid::widget::{ViewSwitcher, Widget, WidgetExt};
use druid::{theme, LensExt};
use main::build_main_window;
use some_lens::SomeLens;

pub fn build_ui() -> impl Widget<AppState> {
    ViewSwitcher::new(
        |state: &AppState, _env| state.main.is_some(),
        |nav, _state, _env| match nav {
            true => Box::new(build_main_window().lens(AppState::main.then(SomeLens))),
            false => Box::new(build_create_window()),
        },
    )
    .background(theme::BACKGROUND_DARK)
    .expand()
}
