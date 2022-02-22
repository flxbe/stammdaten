pub mod create_bank_account;
pub mod create_id_card;
pub mod create_post_number;
pub mod create_profile;
pub mod create_social_security_number;
pub mod create_tax_id;
mod main;
mod some_lens;

use crate::data::Profile;
use crate::state::AppState;
use druid::widget::{Controller, Widget, WidgetExt};
use druid::{theme, Env, Event, EventCtx, Selector};

pub use create_profile::PROFILE_CREATED;
pub use main::PROFILE_UPDATED;
pub const SAVE_PROFILE: Selector<Profile> = Selector::new("app.save_profile");

struct AppController;

impl<W> Controller<AppState, W> for AppController
where
    W: Widget<AppState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_profile::PROFILE_CREATED) => {
                let profile = not.get(create_profile::PROFILE_CREATED).unwrap();

                ctx.submit_command(SAVE_PROFILE.with(profile.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(main::PROFILE_UPDATED) => {
                let profile = not.get(main::PROFILE_UPDATED).unwrap();

                ctx.submit_command(SAVE_PROFILE.with(profile.clone()));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub fn build_ui() -> impl Widget<AppState> {
    AppState::matcher()
        .create(create_profile::build())
        .main(main::build())
        .background(theme::BACKGROUND_DARK)
        .expand()
        .controller(AppController)
}
