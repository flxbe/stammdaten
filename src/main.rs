mod data;
mod state;
mod ui;
mod widgets;

use crate::data::Profile;
use crate::state::AppState;
use directories::ProjectDirs;
use druid::{
    commands, platform_menus, AppDelegate, AppLauncher, Command, Data, DelegateCtx, Env, Handled,
    LocalizedString, MenuDesc, PlatformError, Target, WindowDesc,
};
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
        WindowDesc::new(ui::build_ui)
            .title("Stammdaten")
            .menu(app_menu())
            .window_size((800.0, 600.0))
            .resizable(false),
    )
    .delegate(Delegate)
    .use_simple_logger()
    .launch(initial_state)?;
    Ok(())
}

#[allow(unreachable_code)]
fn app_menu<T: Data>() -> MenuDesc<T> {
    #[cfg(target_os = "macos")]
    {
        return MenuDesc::empty()
            .append(platform_menus::mac::application::default())
            .append(edit_menu());
    }

    return MenuDesc::empty();
}

#[warn(dead_code)]
fn edit_menu<T: Data>() -> MenuDesc<T> {
    MenuDesc::new(LocalizedString::new("common-menu-edit-menu"))
        .append(platform_menus::common::cut().disabled())
        .append(platform_menus::common::copy())
        .append(platform_menus::common::paste())
}

fn get_config_path() -> PathBuf {
    let project_dirs =
        ProjectDirs::from("io", "flxbe", "Stammdaten").expect("Could not load project directories");

    project_dirs.config_dir().to_path_buf()
}

/// Global command handler.
struct Delegate;

impl AppDelegate<AppState> for Delegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled {
        if cmd.is(commands::SAVE_FILE) {
            let config_path = get_config_path();
            std::fs::create_dir_all(&config_path).expect("Could not create data directory");

            let profile_path = config_path.join(PROFILE_FILENAME);
            let profile = data.get_profile();

            let mut file = File::create(profile_path).expect("Could not open file to save profile");
            profile
                .save_to_file(&mut file)
                .expect("Could not save profile");

            return Handled::Yes;
        }

        return Handled::No;
    }
}
