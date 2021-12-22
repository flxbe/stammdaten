mod profile;
mod social_security_number;
mod tax_id;

use crate::profile::Profile;
use clipboard::{ClipboardContext, ClipboardProvider};
use druid::widget::{
    Button, Container, CrossAxisAlignment, Flex, Label, MainAxisAlignment, Padding, SizedBox, Svg,
    SvgData,
};
use druid::{AppLauncher, Color, Data, Lens, PlatformError, Widget, WidgetExt, WindowDesc};

#[derive(Clone, Data, Lens)]
struct State {
    #[data(same_fn = "PartialEq::eq")]
    pub profile: Profile,
}

fn build_ui() -> impl Widget<State> {
    Flex::column()
        .must_fill_main_axis(true)
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .main_axis_alignment(MainAxisAlignment::Start)
        .with_child(build_header())
        .with_flex_child(build_body(), 1.0)
}

fn build_header() -> impl Widget<State> {
    let profile_svg = include_str!("profile-svgrepo-com.svg")
        .parse::<SvgData>()
        .unwrap();

    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(
            SizedBox::new(Svg::new(profile_svg))
                .height(50.0)
                .width(50.0),
        )
        .with_default_spacer()
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(|state: &State, _env: &_| {
                    format!("{} {}", state.profile.first_name, state.profile.last_name)
                })),
        )
        .expand_width()
        .padding(25.0)
}

fn build_body() -> impl Widget<State> {
    Container::new(
        SizedBox::new(Padding::new(
            10.0,
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(build_item("Sozialversicherungsnummer", |state| {
                    state.profile.social_security_number.to_string()
                }))
                .with_default_spacer()
                .with_child(build_item("Steuer-ID", |state| {
                    state.profile.tax_id.to_string()
                })),
        ))
        .expand(),
    )
    .background(Color::from_hex_str("353535").unwrap())
}

fn build_item(title: &str, f: impl Fn(&State) -> String + 'static + Copy) -> impl Widget<State> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row().with_child(Label::new(move |state: &State, _env: &_| f(state))),
                )
                .with_child(Label::new(title).with_text_size(12.0)),
        )
        .with_child(
            Button::new("Copy")
                .on_click(move |_ctx, state: &mut State, _env| copy_to_clipboard(f(state))),
        )
        .padding(10.0)
}

fn copy_to_clipboard(value: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value).unwrap();
}

fn main() -> Result<(), PlatformError> {
    let profile = include_str!("../data.json")
        .parse::<Profile>()
        .expect("Could not read the profile");

    let initial_state = State { profile };

    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .title("Stammdaten")
            .window_size((400.0, 300.0))
            .resizable(false),
    )
    .launch(initial_state)?;
    Ok(())
}
