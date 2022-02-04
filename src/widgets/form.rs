use druid::widget::prelude::*;
use druid::{Point, Selector, WidgetPod};

const CANCEL: Selector<()> = Selector::new("app.form.cancel");
const SUBMIT: Selector<()> = Selector::new("app.form.submit");

pub struct Form<T: Data, F: Data, W: Widget<F>> {
    state: F,
    child: WidgetPod<F, W>,
    cancel_callback: Option<Box<dyn Fn(&mut EventCtx, &mut T, &Env)>>,
    submit_callback: Option<Box<dyn Fn(&mut EventCtx, &mut T, &mut F, &Env)>>,
}

impl<T: Data, F: Data, W: Widget<F>> Form<T, F, W> {
    pub fn new(initial_state: F, child: W) -> Form<T, F, W> {
        Form {
            state: initial_state,
            child: WidgetPod::new(child),
            cancel_callback: None,
            submit_callback: None,
        }
    }

    pub fn on_cancel(mut self, callback: impl Fn(&mut EventCtx, &mut T, &Env) + 'static) -> Self {
        self.cancel_callback = Some(Box::new(callback));
        self
    }

    pub fn on_submit(
        mut self,
        callback: impl Fn(&mut EventCtx, &mut T, &mut F, &Env) + 'static,
    ) -> Self {
        self.submit_callback = Some(Box::new(callback));
        self
    }

    fn update_state(&mut self, ctx: &mut EventCtx, new_state: F) {
        if !new_state.same(&self.state) {
            self.state = new_state;

            // As druid cannot track the local data, we must explicitly request an update.
            ctx.request_update();
        }
    }
}

impl<T: Data, F: Data, W: Widget<F>> Widget<T> for Form<T, F, W> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        match event {
            Event::Notification(notification) if notification.is(CANCEL) => {
                if let Some(callback) = self.cancel_callback.as_ref() {
                    callback(ctx, data, env);
                }
                ctx.set_handled();
            }
            Event::Notification(notification) if notification.is(SUBMIT) => {
                if let Some(callback) = self.submit_callback.as_ref() {
                    let mut new_state = self.state.clone();
                    callback(ctx, data, &mut new_state, env);
                    self.update_state(ctx, new_state);
                }
                ctx.set_handled();
            }
            _ => {
                let mut new_state = self.state.clone();
                self.child.event(ctx, event, &mut new_state, env);
                self.update_state(ctx, new_state);
            }
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, _data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, &self.state, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, _old_data: &T, _data: &T, env: &Env) {
        self.child.update(ctx, &self.state, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &T, env: &Env) -> Size {
        let size = self.child.layout(ctx, bc, &self.state, env);
        self.child.set_origin(ctx, &self.state, env, Point::ORIGIN);

        return size;
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &T, env: &Env) {
        self.child.paint(ctx, &self.state, env);
    }
}

pub fn cancel_form(ctx: &mut EventCtx) {
    ctx.submit_notification(CANCEL);
}

pub fn submit_form(ctx: &mut EventCtx) {
    ctx.submit_notification(SUBMIT);
}
