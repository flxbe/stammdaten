mod profile;
mod social_security_number;
mod state;
mod tax_id;
mod widgets;

use crate::profile::{BankAccount, Profile};
use crate::state::{Nav, State};
use crate::widgets::{NavController, NAVIGATE};
use clipboard::{ClipboardContext, ClipboardProvider};
use druid::im::Vector;
use druid::widget::{
    Button, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, SizedBox, Split, Svg,
    SvgData, ViewSwitcher,
};
use druid::{theme, AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc};
use webbrowser;

fn build_ui() -> impl Widget<State> {
    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(build_sidebar_header())
        .with_child(build_sidebar_navigation())
        .background(theme::BACKGROUND_DARK);

    let main = ViewSwitcher::new(
        |state: &State, _env| state.nav,
        |nav, _state, _env| match nav {
            Nav::Home => Box::new(build_home()),
            Nav::BankAccounts => Box::new(build_bank_account_page()),
        },
    )
    .background(theme::BACKGROUND_LIGHT)
    .expand();

    Split::columns(sidebar, main)
        .split_point(0.3)
        .bar_size(1.0)
        .min_size(150.0, 300.0)
        .min_bar_area(1.0)
        .solid_bar(true)
        .controller(NavController)
}

fn build_sidebar_header() -> impl Widget<State> {
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
        .with_spacer(10.0)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(|state: &State, _env: &_| {
                    format!("{} {}", state.first_name, state.last_name)
                })),
        )
        .expand_width()
        .padding(30.0)
}

fn build_sidebar_navigation() -> impl Widget<State> {
    Flex::column()
        .with_default_spacer()
        .with_child(sidebar_link_widget("Basisdaten", Nav::Home))
        .with_child(sidebar_link_widget("Konten", Nav::BankAccounts))
}

fn sidebar_link_widget(title: &str, link_nav: Nav) -> impl Widget<State> {
    Label::new(title)
        .with_text_size(20.0)
        .expand_width()
        .lens(State::nav)
        .padding((25.0, 10.0))
        .on_click(move |ctx, _, _| ctx.submit_command(NAVIGATE.with(link_nav)))
}

fn build_home() -> impl Widget<State> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_home_item("Sozialversicherungsnummer", |state| {
            state.social_security_number.to_string()
        }))
        .with_default_spacer()
        .with_child(build_home_item("Steuer-ID", |state| {
            state.tax_id.to_string()
        }))
        .padding(10.0)
        .expand()
}

fn build_home_item(
    title: &str,
    f: impl Fn(&State) -> String + 'static + Copy,
) -> impl Widget<State> {
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
            Button::new("Kopieren")
                .on_click(move |_ctx, state: &mut State, _env| copy_to_clipboard(f(state))),
        )
        .padding(10.0)
}

fn build_bank_account_page() -> impl Widget<State> {
    List::new(|| build_bank_account())
        .with_spacing(10.0)
        .lens(State::bank_accounts)
        .padding(10.0)
        .expand()
}

fn build_bank_account() -> impl Widget<BankAccount> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::dynamic(|account: &BankAccount, _env| {
                    account.iban.clone()
                }))
                .with_child(
                    Label::dynamic(|account: &BankAccount, _env| account.name.clone())
                        .with_text_size(12.0),
                ),
        )
        .with_child(
            Button::new("Onlinebanking")
                .on_click(|_ctx, account: &mut BankAccount, _env| open_url(&account.url)),
        )
        .with_child(
            Button::new("IBAN Kopieren")
                .on_click(|_ctx, account: &mut BankAccount, _env| copy_to_clipboard(&account.iban)),
        )
        .padding(10.0)
}

fn copy_to_clipboard(value: impl Into<String>) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(value.into()).unwrap();
}

fn open_url(url: &str) {
    webbrowser::open(url).unwrap();
}

fn main() -> Result<(), PlatformError> {
    let profile = include_str!("../data.json")
        .parse::<Profile>()
        .expect("Could not read the profile");

    let initial_state = State {
        first_name: profile.first_name,
        last_name: profile.last_name,
        social_security_number: profile.social_security_number,
        tax_id: profile.tax_id,
        bank_accounts: Vector::from(profile.bank_accounts),
        nav: Nav::Home,
    };

    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .title("Stammdaten")
            .window_size((800.0, 600.0))
            .resizable(false),
    )
    .launch(initial_state)?;
    Ok(())
}
