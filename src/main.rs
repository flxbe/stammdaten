mod data;
mod state;
mod widgets;

use crate::data::Profile;
use crate::state::{AppState, BankAccountState, Nav, ProfileState};
use crate::widgets::{NavController, OutlineButton, NAVIGATE};
use clipboard::{ClipboardContext, ClipboardProvider};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, SizedBox, Split, Svg, SvgData,
    ViewSwitcher,
};
use druid::{theme, AppLauncher, LensExt, PlatformError, Widget, WidgetExt, WindowDesc};
use webbrowser;

fn build_ui() -> impl Widget<AppState> {
    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(build_sidebar_header())
        .with_child(build_sidebar_navigation())
        .background(theme::BACKGROUND_LIGHT);

    let main = ViewSwitcher::new(
        |state: &AppState, _env| state.nav,
        |nav, _state, _env| match nav {
            Nav::Home => Box::new(build_home()),
            Nav::BankAccounts => Box::new(build_bank_account_page()),
        },
    )
    .background(theme::BACKGROUND_DARK)
    .expand();

    Split::columns(sidebar, main)
        .split_point(0.3)
        .bar_size(1.0)
        .min_size(150.0, 300.0)
        .min_bar_area(1.0)
        .solid_bar(true)
        .controller(NavController)
}

fn build_sidebar_header() -> impl Widget<AppState> {
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
                .with_child(Label::new(|state: &AppState, _env: &_| {
                    format!("{} {}", state.profile.first_name, state.profile.last_name)
                })),
        )
        .expand_width()
        .padding(30.0)
}

fn build_sidebar_navigation() -> impl Widget<AppState> {
    Flex::column()
        .with_default_spacer()
        .with_child(sidebar_link_widget("Basisdaten", Nav::Home))
        .with_child(sidebar_link_widget("Konten", Nav::BankAccounts))
}

fn sidebar_link_widget(title: &str, link_nav: Nav) -> impl Widget<AppState> {
    Label::new(title)
        .with_text_size(20.0)
        .expand_width()
        .lens(AppState::nav)
        .padding((25.0, 10.0))
        .on_click(move |ctx, _, _| ctx.submit_command(NAVIGATE.with(link_nav)))
}

fn build_home() -> impl Widget<AppState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_id_card_item())
        .with_default_spacer()
        .with_child(build_home_item("Sozialversicherungsnummer", |state| {
            state.profile.social_security_number.to_string()
        }))
        .with_default_spacer()
        .with_child(build_home_item("Steuer-ID", |state| {
            state.profile.tax_id.to_string()
        }))
        .with_default_spacer()
        .with_child(build_home_item("Postnummer", |state| {
            state.profile.post_number.to_string()
        }))
        .padding(10.0)
        .expand()
}

fn build_id_card_item() -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row().with_child(Label::dynamic(|state: &AppState, _env| {
                        state.profile.id_card.card_number.to_string()
                    })),
                )
                .with_child(
                    Label::dynamic(|state: &AppState, _env| {
                        format!(
                            "Personalsausweis - {} Tage gültig",
                            state.profile.id_card.time_until_expiration().num_days()
                        )
                    })
                    .with_text_size(12.0),
                ),
        )
        .with_child(OutlineButton::new("Kopieren").on_click(
            move |_ctx, state: &mut AppState, _env| {
                copy_to_clipboard(state.profile.id_card.card_number.to_string())
            },
        ))
        .padding(10.0)
}

fn build_home_item(
    title: &str,
    f: impl Fn(&AppState) -> String + 'static + Copy,
) -> impl Widget<AppState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row().with_child(Label::new(move |state: &AppState, _env: &_| f(state))),
                )
                .with_child(Label::new(title).with_text_size(12.0)),
        )
        .with_child(
            OutlineButton::new("Kopieren")
                .on_click(move |_ctx, state: &mut AppState, _env| copy_to_clipboard(f(state))),
        )
        .padding(10.0)
}

fn build_bank_account_page() -> impl Widget<AppState> {
    List::new(|| build_bank_account())
        .with_spacing(10.0)
        .lens(AppState::profile.then(ProfileState::bank_accounts))
        .padding(10.0)
        .expand()
}

fn build_bank_account() -> impl Widget<BankAccountState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::dynamic(|account: &BankAccountState, _env| {
                    account.iban.clone()
                }))
                .with_child(
                    Label::dynamic(|account: &BankAccountState, _env| account.name.clone())
                        .with_text_size(12.0),
                ),
        )
        .with_flex_spacer(1.0)
        .with_child(
            OutlineButton::new("Onlinebanking")
                .on_click(|_ctx, account: &mut BankAccountState, _env| open_url(&account.url)),
        )
        .with_default_spacer()
        .with_child(OutlineButton::new("IBAN Kopieren").on_click(
            |_ctx, account: &mut BankAccountState, _env| copy_to_clipboard(&account.iban),
        ))
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

    let initial_state = AppState {
        profile: ProfileState::from(profile),
        nav: Nav::Home,
    };

    AppLauncher::with_window(
        WindowDesc::new(build_ui)
            .title("Stammdaten")
            .window_size((800.0, 600.0))
            .resizable(false),
    )
    .use_simple_logger()
    .launch(initial_state)?;
    Ok(())
}
