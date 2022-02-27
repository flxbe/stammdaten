use druid::widget::prelude::*;
use druid::widget::Container;
use druid::{theme, Data, Widget, WidgetExt};

pub struct Card<T> {
    child: Container<T>,
}

impl<T: Data> Card<T> {
    pub fn new(child: impl Widget<T> + 'static) -> Card<T> {
        let container = child
            .padding(40.0)
            .border(theme::BORDER_LIGHT, 1.0)
            .rounded(4.0);

        Card { child: container }
    }
}

impl<T: Data> Widget<T> for Card<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        self.child.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        self.child.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.child.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        self.child.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        self.child.paint(ctx, data, env);
    }
}
