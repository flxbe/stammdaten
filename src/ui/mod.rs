mod create_bank_account;
mod create_id_card;
pub mod create_post_number;
mod create_profile;
mod create_social_security_number;
mod create_tax_id;
mod main;
mod some_lens;

use crate::state::AppState;
use druid::widget::{Controller, Widget, WidgetExt};
use druid::{theme, Env, Event, EventCtx};

pub struct AppController;

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
                *data = AppState::from_profile(
                    not.get(create_profile::PROFILE_CREATED).unwrap().clone(),
                );
                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(main::PROFILE_UPDATED) => {
                *data = AppState::from_profile(not.get(main::PROFILE_UPDATED).unwrap().clone());
                ctx.submit_command(druid::commands::SAVE_FILE);
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
        //ViewSwitcher::new(
        //|state: &AppState, _env| get_current_location(state),
        //|location, _state, _env| match location {
        //Location::CreateProfile => Box::new(create_profile::build()),
        //Location::Main => Box::new(main::build().lens(AppState::main.then(SomeLens))),
        //Location::Process(process) => match process {
        //Process::CreateSocialSecurityNumber => Box::new(
        //create_social_security_number::build().lens(AppState::main.then(SomeLens)),
        //),
        //Process::CreateTaxId => {
        //Box::new(create_tax_id::build().lens(AppState::main.then(SomeLens)))
        //}
        //Process::CreatePostNumber => {
        //Box::new(create_post_number::build().lens(AppState::main.then(SomeLens)))
        //}
        //Process::CreateIdCard => {
        //Box::new(create_id_card::build().lens(AppState::main.then(SomeLens)))
        //}
        //Process::CreateBankAccount => {
        //Box::new(create_bank_account::build().lens(AppState::main.then(SomeLens)))
        //}
        //},
        //},
        //)
        .background(theme::BACKGROUND_DARK)
        .expand()
}
