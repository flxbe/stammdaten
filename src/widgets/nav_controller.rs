use crate::state::{MainState, Nav};
use druid::widget::{Controller, Widget};
use druid::{Env, Event, EventCtx, Selector};

pub const NAVIGATE: Selector<Nav> = Selector::new("app.navigate");

pub struct NavController;

impl<W> Controller<MainState, W> for NavController
where
    W: Widget<MainState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut MainState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NAVIGATE) => {
                let nav = cmd.get_unchecked(NAVIGATE);
                data.nav = *nav;
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}
