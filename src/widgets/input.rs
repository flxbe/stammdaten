use druid::widget::{CrossAxisAlignment, Flex, Label, Maybe, TextBox, Widget, WidgetExt};
use druid::{theme, Data, Lens};
use std::sync::Arc;

#[derive(Data, Lens, PartialEq, Eq, Clone, Default, Debug)]
pub struct InputState {
    pub value: Arc<String>,
    pub error: Option<String>,
}

impl InputState {
    pub fn reset_error(&mut self) {
        self.error = None;
    }

    pub fn set_error(&mut self, error: String) {
        self.error = Some(error);
    }
}

pub fn input(title: &str) -> impl Widget<InputState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(Label::new(title))
        .with_child(
            TextBox::new()
                .lens(InputState::value)
                .expand_width()
                .padding((0.0, 5.0, 0.0, 0.0)),
        )
        .with_child(
            Maybe::or_empty(|| {
                Label::dynamic(|state: &String, _| state.to_string())
                    .with_text_color(theme::DISABLED_TEXT_COLOR)
                    .with_text_size(12.0)
            })
            .padding((0.0, 5.0, 0.0, 0.0))
            .lens(InputState::error),
        )
}
