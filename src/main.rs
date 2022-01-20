mod data;
mod state;
mod ui;
mod widgets;

use crate::state::AppState;
use druid::{AppLauncher, PlatformError, WindowDesc};

fn main() -> Result<(), PlatformError> {
    // let profile = include_str!("../data.json")
    // .parse::<Profile>()
    // .expect("Could not read the profile");

    // let initial_state = AppState::from_profile(profile);
    let initial_state = AppState::new();

    AppLauncher::with_window(
        WindowDesc::new(ui::build_ui)
            .title("Stammdaten")
            .window_size((800.0, 600.0))
            .resizable(false),
    )
    .use_simple_logger()
    .launch(initial_state)?;
    Ok(())
}
