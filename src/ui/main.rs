use super::some_lens::SomeLens;
use crate::data::{BankAccount, IdCard, Name};
use crate::state::{MainState, Nav, Process, ProfileState};
use crate::widgets::OutlineButton;
use druid::widget::{
    Controller, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, SizedBox, Split, Svg,
    SvgData, ViewSwitcher,
};
use druid::{theme, Application, Env, Event, EventCtx, LensExt, Selector, Widget, WidgetExt};
use webbrowser;

pub const START_PROCESS: Selector<Process> = Selector::new("app.start_process");
pub const NAVIGATE: Selector<Nav> = Selector::new("app.navigate");

const CLEAR_ID_CARD: Selector<()> = Selector::new("app.main.clear_id_card");
const CLEAR_SOCIAL_SECURITY_NUMBER: Selector<()> =
    Selector::new("app.main.clear_social_security_number");
const CLEAR_TAX_ID: Selector<()> = Selector::new("app.main.clear_tax_id");
const CLEAR_POST_NUMBER: Selector<()> = Selector::new("app.main.clear_post_number");
const CLEAR_BANK_ACCOUNT: Selector<String> = Selector::new("app.main.clear_bank_account");

pub struct MainController;

impl<W> Controller<MainState, W> for MainController
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
            Event::Command(cmd) if cmd.is(START_PROCESS) => {
                // The current process should be None when a new process is started.
                assert_eq!(data.active_process, None);

                let process = cmd.get_unchecked(START_PROCESS);
                data.active_process = Some(*process);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_ID_CARD) => {
                data.profile.id_card = None;
                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_SOCIAL_SECURITY_NUMBER) => {
                data.profile.social_security_number = None;
                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_TAX_ID) => {
                data.profile.tax_id = None;
                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_POST_NUMBER) => {
                data.profile.post_number = None;
                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_BANK_ACCOUNT) => {
                let iban = not.get(CLEAR_BANK_ACCOUNT).unwrap();

                data.profile.bank_accounts = data
                    .profile
                    .bank_accounts
                    .iter()
                    .filter(|account| account.iban != *iban)
                    .map(|account| account.to_owned())
                    .collect();

                ctx.submit_command(druid::commands::SAVE_FILE);
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub fn build() -> impl Widget<MainState> {
    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(build_sidebar_header().lens(MainState::profile.then(ProfileState::name)))
        .with_child(build_sidebar_navigation())
        .background(theme::BACKGROUND_LIGHT);

    let main = ViewSwitcher::new(
        |state: &MainState, _env| state.nav,
        |nav, _state, _env| match nav {
            Nav::Home => Box::new(build_home().lens(MainState::profile)),
            Nav::BankAccounts => Box::new(build_bank_account_page().lens(MainState::profile)),
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
        .controller(MainController)
}

fn build_sidebar_header() -> impl Widget<Name> {
    let profile_svg = include_str!("../profile-svgrepo-com.svg")
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
                .with_child(Label::new(|state: &Name, _env: &_| {
                    format!("{} {}", state.first_name, state.last_name)
                })),
        )
        .expand_width()
        .padding(30.0)
}

fn build_sidebar_navigation() -> impl Widget<MainState> {
    Flex::column()
        .with_default_spacer()
        .with_child(sidebar_link_widget("Basisdaten", Nav::Home))
        .with_child(sidebar_link_widget("Konten", Nav::BankAccounts))
}

fn sidebar_link_widget(title: &str, link_nav: Nav) -> impl Widget<MainState> {
    Label::new(title)
        .with_text_size(20.0)
        .expand_width()
        .lens(MainState::nav)
        .padding((25.0, 10.0))
        .on_click(move |ctx, _, _| ctx.submit_command(NAVIGATE.with(link_nav)))
}

fn build_home() -> impl Widget<ProfileState> {
    Flex::column()
        .cross_axis_alignment(CrossAxisAlignment::Start)
        .with_child(build_optional_id_card().lens(ProfileState::id_card))
        .with_default_spacer()
        .with_child(
            build_optional_item(
                String::from("Sozialversichersungsnummer"),
                |ctx, _state, _env| {
                    ctx.submit_command(START_PROCESS.with(Process::CreateSocialSecurityNumber))
                },
                |ctx, _, _| ctx.submit_notification(CLEAR_SOCIAL_SECURITY_NUMBER),
            )
            .lens(ProfileState::social_security_number),
        )
        .with_default_spacer()
        .with_child(
            build_optional_item(
                String::from("Steuer-ID"),
                |ctx, _state, _env| ctx.submit_command(START_PROCESS.with(Process::CreateTaxId)),
                |ctx, _, _| ctx.submit_notification(CLEAR_TAX_ID),
            )
            .lens(ProfileState::tax_id),
        )
        .with_default_spacer()
        .with_child(
            build_optional_item(
                String::from("Postnummer"),
                |ctx, _state, _env| {
                    ctx.submit_command(START_PROCESS.with(Process::CreatePostNumber))
                },
                |ctx, _, _| ctx.submit_notification(CLEAR_POST_NUMBER),
            )
            .lens(ProfileState::post_number),
        )
        .padding(10.0)
        .expand()
}

fn build_optional_id_card() -> impl Widget<Option<IdCard>> {
    ViewSwitcher::new(
        |state: &Option<IdCard>, _env| state.is_some(),
        move |state, _state, _env| match state {
            true => Box::new(build_id_card_item().lens(SomeLens)),
            false => Box::new(build_add_button("Personalausweis".into(), |ctx, _, _| {
                ctx.submit_command(START_PROCESS.with(Process::CreateIdCard))
            })),
        },
    )
}

fn build_id_card_item() -> impl Widget<IdCard> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .main_axis_alignment(MainAxisAlignment::SpaceBetween)
        .must_fill_main_axis(true)
        .with_child(
            Flex::column()
                .cross_axis_alignment(CrossAxisAlignment::Start)
                .with_child(
                    Flex::row().with_child(Label::dynamic(|state: &IdCard, _env| {
                        state.card_number.to_string()
                    })),
                )
                .with_child(
                    Label::dynamic(|state: &IdCard, _env| {
                        format!(
                            "Personalsausweis - {} Tage g??ltig",
                            state.time_until_expiration().num_days()
                        )
                    })
                    .with_text_size(12.0),
                ),
        )
        .with_flex_spacer(1.0)
        .with_child(
            OutlineButton::new("L??schen")
                .on_click(|ctx, _, _| ctx.submit_notification(CLEAR_ID_CARD)),
        )
        .with_default_spacer()
        .with_child(OutlineButton::new("Kopieren").on_click(
            move |_ctx, state: &mut IdCard, _env| copy_to_clipboard(state.card_number.to_string()),
        ))
        .padding(10.0)
}

fn build_optional_item<T>(
    title: String,
    on_create: impl Fn(&mut EventCtx, &mut Option<T>, &Env) + 'static + Copy,
    on_delete: impl Fn(&mut EventCtx, &mut T, &Env) + 'static + Copy,
) -> impl Widget<Option<T>>
where
    T: Into<String> + druid::Data,
{
    ViewSwitcher::new(
        |state: &Option<T>, _env| state.is_some(),
        move |state, _state, _env| match state {
            true => Box::new(build_item(&title, on_delete).lens(SomeLens)),
            false => Box::new(build_add_button(&title, on_create)),
        },
    )
}

fn build_add_button<T>(
    title: &str,
    on_create: impl Fn(&mut EventCtx, &mut Option<T>, &Env) + 'static,
) -> impl Widget<Option<T>>
where
    T: druid::Data,
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

fn build_item<T>(
    title: &str,
    on_delete: impl Fn(&mut EventCtx, &mut T, &Env) + 'static,
) -> impl Widget<T>
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
        .with_flex_spacer(1.0)
        .with_child(
            OutlineButton::new("L??schen")
                .on_click(move |ctx, state: &mut T, env| on_delete(ctx, state, env)),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Kopieren")
                .on_click(move |_ctx, state: &mut T, _env| copy_to_clipboard(state.clone().into())),
        )
        .padding(10.0)
}

fn build_bank_account_page() -> impl Widget<ProfileState> {
    Flex::column()
        .with_flex_child(
            List::new(|| build_bank_account())
                .with_spacing(10.0)
                .lens(ProfileState::bank_accounts),
            1.0,
        )
        .with_default_spacer()
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Neues Konto Erstellen").on_click(|ctx, _, _| {
                ctx.submit_command(START_PROCESS.with(Process::CreateBankAccount))
            }),
        )
        .padding(10.0)
}

fn build_bank_account() -> impl Widget<BankAccount> {
    Flex::row()
        .cross_axis_alignment(CrossAxisAlignment::Center)
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
        .with_flex_spacer(1.0)
        .with_child(
            OutlineButton::new("L??schen").on_click(|ctx, account: &mut BankAccount, _| {
                ctx.submit_notification(CLEAR_BANK_ACCOUNT.with(account.iban.to_owned()))
            }),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("Banking")
                .on_click(|_ctx, account: &mut BankAccount, _env| open_url(&account.url)),
        )
        .with_default_spacer()
        .with_child(
            OutlineButton::new("IBAN Kopieren")
                .on_click(|_ctx, account: &mut BankAccount, _env| copy_to_clipboard(&account.iban)),
        )
        .padding(10.0)
}

fn copy_to_clipboard(value: impl Into<String>) {
    let mut clipboard = Application::global().clipboard();
    clipboard.put_string(value.into());
}

fn open_url(url: &str) {
    webbrowser::open(url).unwrap();
}
