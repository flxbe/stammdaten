mod data;
mod state;
mod widgets;

use crate::data::{PostNumber, Profile};
use crate::state::{AppState, BankAccountState, Nav, ProfileState};
use crate::widgets::{NavController, OutlineButton, NAVIGATE};
use clipboard::{ClipboardContext, ClipboardProvider};
use druid::widget::{
    CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, SizedBox, Split, Svg, SvgData,
    ViewSwitcher,
};
use druid::{
    theme, AppLauncher, Env, EventCtx, Lens, PlatformError, Widget, WidgetExt, WindowDesc,
};
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
            Nav::Home => Box::new(build_home().lens(AppState::profile)),
            Nav::BankAccounts => Box::new(build_bank_account_page().lens(AppState::profile)),
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

fn build_home() -> impl Widget<ProfileState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_id_card_item())
        .with_default_spacer()
        .with_child(build_home_item("Sozialversicherungsnummer", |state| {
            state.social_security_number.to_string()
        }))
        .with_default_spacer()
        .with_child(build_home_item("Steuer-ID", |state| {
            state.tax_id.to_string()
        }))
        .with_default_spacer()
        .with_child(
            build_optional_item(String::from("Postnummer"), |_ctx, state, _env| {
                *state = Some(PostNumber::try_from(123456789).unwrap())
            })
            .lens(ProfileState::post_number),
        )
        .padding(10.0)
        .expand()
}

fn build_id_card_item() -> impl Widget<ProfileState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row().with_child(Label::dynamic(|state: &ProfileState, _env| {
                        state.id_card.card_number.to_string()
                    })),
                )
                .with_child(
                    Label::dynamic(|state: &ProfileState, _env| {
                        format!(
                            "Personalsausweis - {} Tage gültig",
                            state.id_card.time_until_expiration().num_days()
                        )
                    })
                    .with_text_size(12.0),
                ),
        )
        .with_child(OutlineButton::new("Kopieren").on_click(
            move |_ctx, state: &mut ProfileState, _env| {
                state.first_name = String::from("lol");
                copy_to_clipboard(state.id_card.card_number.to_string())
            },
        ))
        .padding(10.0)
}

fn build_home_item(
    title: &str,
    f: impl Fn(&ProfileState) -> String + 'static + Copy,
) -> impl Widget<ProfileState> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(move |state: &ProfileState, _env: &_| f(state)))
                .with_child(Label::new(title).with_text_size(12.0)),
        )
        .with_child(
            OutlineButton::new("Kopieren")
                .on_click(move |_ctx, state: &mut ProfileState, _env| copy_to_clipboard(f(state))),
        )
        .padding(10.0)
}

struct SomeLens;

impl<T> Lens<Option<T>, T> for SomeLens {
    fn with<V, F: FnOnce(&T) -> V>(&self, data: &Option<T>, f: F) -> V {
        f(data.as_ref().unwrap())
    }

    fn with_mut<V, F: FnOnce(&mut T) -> V>(&self, data: &mut Option<T>, f: F) -> V {
        f(data.as_mut().unwrap())
    }
}

fn build_optional_item<T>(
    title: String,
    on_create: impl Fn(&mut EventCtx, &mut Option<T>, &Env) + 'static + Copy,
) -> impl Widget<Option<T>>
where
    T: Into<String> + druid::Data,
{
    ViewSwitcher::new(
        |state: &Option<T>, _env| state.is_some(),
        move |state, _state, _env| match state {
            true => Box::new(build_item(&title).lens(SomeLens)),
            false => Box::new(build_add_button(&title, on_create)),
        },
    )
}

fn build_add_button<T>(
    title: &str,
    on_create: impl Fn(&mut EventCtx, &mut Option<T>, &Env) + 'static,
) -> impl Widget<Option<T>>
where
    T: Into<String> + druid::Data,
{
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Flex::row().with_child(Label::new("...")))
                .with_child(Label::new(title).with_text_size(12.0)),
        )
        .with_child(OutlineButton::new("Erstellen").on_click(on_create))
        .padding(10.0)
}

fn build_item<T>(title: &str) -> impl Widget<T>
where
    T: Into<String> + druid::Data,
{
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(Label::new(move |state: &T, _env: &_| state.clone().into()))
                .with_child(Label::new(title).with_text_size(12.0)),
        )
        .with_child(
            OutlineButton::new("Kopieren")
                .on_click(move |_ctx, state: &mut T, _env| copy_to_clipboard(state.clone().into())),
        )
        .padding(10.0)
}

fn build_bank_account_page() -> impl Widget<ProfileState> {
    List::new(|| build_bank_account())
        .with_spacing(10.0)
        .lens(ProfileState::bank_accounts)
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
