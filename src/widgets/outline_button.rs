use druid::widget::prelude::*;
use druid::widget::{Click, ControllerHost, Label, LabelText};
use druid::{theme, Affine, Data, Insets};

const LABEL_INSETS: Insets = Insets::uniform_xy(8., 3.);

pub struct OutlineButton<T> {
    label: Label<T>,
    label_size: Size,
}

impl<T: Data> OutlineButton<T> {
    pub fn new(text: impl Into<LabelText<T>>) -> OutlineButton<T> {
        OutlineButton::from_label(Label::new(text))
    }

    pub fn from_label(label: Label<T>) -> OutlineButton<T> {
        OutlineButton {
            label,
            label_size: Size::ZERO,
        }
    }

    pub fn dynamic(text: impl Fn(&T, &Env) -> String + 'static) -> Self {
        let text: LabelText<T> = text.into();
        OutlineButton::new(text)
    }

    pub fn on_click(
        self,
        f: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
    ) -> ControllerHost<Self, Click<T>> {
        ControllerHost::new(self, Click::new(f))
    }
}

impl<T: Data> Widget<T> for OutlineButton<T> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, _data: &mut T, _env: &Env) {
        match event {
            Event::MouseDown(_) => {
                ctx.set_active(true);
                ctx.request_paint();
            }
            Event::MouseUp(_) => {
                if ctx.is_active() {
                    ctx.request_paint();
                }
                ctx.set_active(false);
            }
            _ => (),
        }
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &T, env: &Env) {
        if let LifeCycle::HotChanged(_) = event {
            if ctx.is_hot() {
                self.label.set_text_color(env.get(theme::BACKGROUND_DARK));
            } else {
                self.label.set_text_color(env.get(theme::FOREGROUND_LIGHT));
            }
            ctx.request_layout();
            //ctx.request_paint();
        }
        self.label.lifecycle(ctx, event, data, env)
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &T, data: &T, env: &Env) {
        self.label.update(ctx, old_data, data, env)
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &T, env: &Env) -> Size {
        let padding = Size::new(LABEL_INSETS.x_value(), LABEL_INSETS.y_value());
        let label_bc = bc.shrink(padding).loosen();
        self.label_size = self.label.layout(ctx, &label_bc, data, env);
        // HACK: to make sure we look okay at default sizes when beside a textbox,
        // we make sure we will have at least the same height as the default textbox.
        let min_height = env.get(theme::BORDERED_WIDGET_HEIGHT);
        let baseline = self.label.baseline_offset();
        ctx.set_baseline_offset(baseline + LABEL_INSETS.y1);

        let button_size = bc.constrain(Size::new(
            self.label_size.width + padding.width,
            (self.label_size.height + padding.height).max(min_height),
        ));
        button_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &T, env: &Env) {
        let is_hot = ctx.is_hot();
        let size = ctx.size();
        let stroke_width = 1.;

        let rounded_rect = size
            .to_rect()
            .inset(-stroke_width / 2.0)
            .to_rounded_rect(env.get(theme::BUTTON_BORDER_RADIUS));

        if is_hot {
            ctx.fill(rounded_rect, &env.get(theme::FOREGROUND_LIGHT));
        }

        let border_color = env.get(theme::FOREGROUND_LIGHT);
        ctx.stroke(rounded_rect, &border_color, stroke_width);

        let label_offset = (size.to_vec2() - self.label_size.to_vec2()) / 2.0;

        ctx.with_save(|ctx| {
            ctx.transform(Affine::translate(label_offset));
            self.label.paint(ctx, data, env);
        });
    }
}
