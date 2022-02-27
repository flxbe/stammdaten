mod data;
mod state;
mod ui;
mod widgets;

use crate::data::Profile;
use crate::state::AppState;
use directories::ProjectDirs;
use druid::menu::Menu;
use druid::{
    platform_menus, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled,
    LocalizedString, PlatformError, Target, WindowDesc,
};
use std::env;
use std::fs::File;
use std::path::PathBuf;

const PROFILE_FILENAME: &str = "profile.json";

fn main() -> Result<(), PlatformError> {
    let profile_path = get_config_path().join(PROFILE_FILENAME);
    let profile = match File::open(profile_path) {
        Ok(mut file) => {
            Some(Profile::load_from_file(&mut file).expect("Could not load profile file"))
        }
        Err(_) => None,
    };

    let initial_state = match profile {
        Some(profile) => AppState::from_profile(profile),
        None => AppState::new(),
    };

    AppLauncher::with_window(
        WindowDesc::new(ui::build_ui())
            .title("Stammdaten")
            .menu(|_, _, _| app_menu())
            .window_size((800.0, 600.0))
            .resizable(false),
    )
    .delegate(Delegate)
    .log_to_console()
    .launch(initial_state)?;
    Ok(())
}

#[allow(unreachable_code)]
fn app_menu<T: Data>() -> Menu<T> {
    #[cfg(target_os = "macos")]
    {
        return Menu::empty()
            .entry(platform_menus::mac::application::default())
            .entry(edit_menu());
    }

    return Menu::empty();
}

#[warn(dead_code)]
fn edit_menu<T: Data>() -> Menu<T> {
    Menu::new(LocalizedString::new("common-menu-edit-menu"))
        .entry(platform_menus::common::cut().enabled(false))
        .entry(platform_menus::common::copy())
        .entry(platform_menus::common::paste())
}

fn get_config_path() -> PathBuf {
    match env::var("STAMMDATEN_DATA_DIR") {
        Ok(path) => PathBuf::from(path),
        Err(_) => {
            let project_dirs = ProjectDirs::from("io", "flxbe", "Stammdaten")
                .expect("Could not load project directories");

            project_dirs.config_dir().to_path_buf()
        }
    }
}

/// Global command handler.
struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        _data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(ui::SAVE_PROFILE) {
            let config_path = get_config_path();
            std::fs::create_dir_all(&config_path).expect("Could not create data directory");

            let profile_path = config_path.join(PROFILE_FILENAME);
            let profile = cmd.get_unchecked(ui::SAVE_PROFILE);

            let mut file = File::create(profile_path).expect("Could not open file to save profile");
            profile
                .save_to_file(&mut file)
                .expect("Could not save profile");

            return Handled::Yes;
        }

        return Handled::No;
    }
}
