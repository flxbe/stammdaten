use super::some_lens::SomeLens;
use crate::data::{BankAccount, IdCard, Name, Profile};
use crate::state::{
    CreateBankAccountState, CreateIdCardState, CreatePostNumberState,
    CreateSocialSecurityNumberState, CreateTaxIdState, HomeState, MainState, Nav, ProfileState,
};
use crate::ui::create_bank_account;
use crate::ui::create_id_card;
use crate::ui::create_post_number;
use crate::ui::create_social_security_number;
use crate::ui::create_tax_id;
use crate::widgets::OutlineButton;
use druid::widget::{
    Controller, CrossAxisAlignment, Flex, Label, List, MainAxisAlignment, SizedBox, Split, Svg,
    SvgData, ViewSwitcher,
};
use druid::{theme, Application, Env, Event, EventCtx, LensExt, Selector, Widget, WidgetExt};
use webbrowser;

const START_PROCESS: Selector<Process> = Selector::new("app.start_process");
const GO_TO_HOME: Selector<HomeState> = Selector::new("app.main.go_to_home");
const NAVIGATE: Selector<Nav> = Selector::new("app.navigate");

const CLEAR_ID_CARD: Selector<()> = Selector::new("app.main.clear_id_card");
const CLEAR_SOCIAL_SECURITY_NUMBER: Selector<()> =
    Selector::new("app.main.clear_social_security_number");
const CLEAR_TAX_ID: Selector<()> = Selector::new("app.main.clear_tax_id");
const CLEAR_POST_NUMBER: Selector<()> = Selector::new("app.main.clear_post_number");
const CLEAR_BANK_ACCOUNT: Selector<String> = Selector::new("app.main.clear_bank_account");

pub const PROFILE_UPDATED: Selector<Profile> = Selector::new("app.main.profile_updated");

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Process {
    CreateTaxId,
    CreatePostNumber,
    CreateIdCard,
    CreateBankAccount,
    CreateSocialSecurityNumber,
}

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
            Event::Notification(notification) if notification.is(GO_TO_HOME) => {
                *data = MainState::Home(notification.get(GO_TO_HOME).unwrap().clone());
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(START_PROCESS) => {
                match data {
                    MainState::Home(state) => {
                        let process = cmd.get_unchecked(START_PROCESS);
                        *data = match process {
                            Process::CreateBankAccount => MainState::CreateBankAccount(
                                CreateBankAccountState::from(state.clone()),
                            ),
                            Process::CreateIdCard => {
                                MainState::CreateIdCard(CreateIdCardState::from(state.clone()))
                            }
                            Process::CreateTaxId => {
                                MainState::CreateTaxId(CreateTaxIdState::from(state.clone()))
                            }
                            Process::CreatePostNumber => MainState::CreatePostNumber(
                                CreatePostNumberState::from(state.clone()),
                            ),
                            Process::CreateSocialSecurityNumber => {
                                MainState::CreateSocialSecurityNumber(
                                    CreateSocialSecurityNumberState::from(state.clone()),
                                )
                            }
                        }
                    }
                    _ => panic!("Cannot start a process when not in MainState::Home"),
                }
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct CreatePostNumberController;

impl<W> Controller<CreatePostNumberState, W> for CreatePostNumberController
where
    W: Widget<CreatePostNumberState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CreatePostNumberState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_post_number::CANCELED) => {
                ctx.submit_notification(GO_TO_HOME.with(data.home_state.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(create_post_number::CREATED) => {
                let post_number = not.get(create_post_number::CREATED).unwrap();

                let mut state = data.home_state.clone();
                state.profile.post_number = Some(post_number.clone());
                ctx.submit_notification(PROFILE_UPDATED.with(state.profile.get_profile()));

                ctx.submit_notification(GO_TO_HOME.with(state));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct CreateTaxIdController;

impl<W> Controller<CreateTaxIdState, W> for CreateTaxIdController
where
    W: Widget<CreateTaxIdState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CreateTaxIdState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_tax_id::CANCELED) => {
                ctx.submit_notification(GO_TO_HOME.with(data.home_state.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(create_tax_id::CREATED) => {
                let tax_id = not.get(create_tax_id::CREATED).unwrap();

                let mut state = data.home_state.clone();
                state.profile.tax_id = Some(tax_id.clone());
                ctx.submit_notification(PROFILE_UPDATED.with(state.profile.get_profile()));

                ctx.submit_notification(GO_TO_HOME.with(state));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct CreateIdCardController;

impl<W> Controller<CreateIdCardState, W> for CreateIdCardController
where
    W: Widget<CreateIdCardState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CreateIdCardState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_id_card::CANCELED) => {
                ctx.submit_notification(GO_TO_HOME.with(data.home_state.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(create_id_card::CREATED) => {
                let id_card = not.get(create_id_card::CREATED).unwrap();

                let mut state = data.home_state.clone();
                state.profile.id_card = Some(id_card.clone());
                ctx.submit_notification(PROFILE_UPDATED.with(state.profile.get_profile()));

                ctx.submit_notification(GO_TO_HOME.with(state));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct CreateSocialSecurityNumberController;

impl<W> Controller<CreateSocialSecurityNumberState, W> for CreateSocialSecurityNumberController
where
    W: Widget<CreateSocialSecurityNumberState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CreateSocialSecurityNumberState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_social_security_number::CANCELED) => {
                ctx.submit_notification(GO_TO_HOME.with(data.home_state.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(create_social_security_number::CREATED) => {
                let social_security_number =
                    not.get(create_social_security_number::CREATED).unwrap();

                let mut state = data.home_state.clone();
                state.profile.social_security_number = Some(social_security_number.clone());
                ctx.submit_notification(PROFILE_UPDATED.with(state.profile.get_profile()));

                ctx.submit_notification(GO_TO_HOME.with(state));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct CreateBankAccountController;

impl<W> Controller<CreateBankAccountState, W> for CreateBankAccountController
where
    W: Widget<CreateBankAccountState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut CreateBankAccountState,
        env: &Env,
    ) {
        match event {
            Event::Notification(not) if not.is(create_bank_account::CANCELED) => {
                ctx.submit_notification(GO_TO_HOME.with(data.home_state.clone()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(create_bank_account::CREATED) => {
                let bank_account = not.get(create_bank_account::CREATED).unwrap();

                let mut state = data.home_state.clone();
                state.profile.bank_accounts.push_back(bank_account.clone());
                ctx.submit_notification(PROFILE_UPDATED.with(state.profile.get_profile()));

                ctx.submit_notification(GO_TO_HOME.with(state));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub struct HomeController;

impl<W> Controller<HomeState, W> for HomeController
where
    W: Widget<HomeState>,
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut HomeState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(NAVIGATE) => {
                let nav = cmd.get_unchecked(NAVIGATE);
                data.nav = *nav;
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_ID_CARD) => {
                data.profile.id_card = None;
                ctx.submit_notification(PROFILE_UPDATED.with(data.profile.get_profile()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_SOCIAL_SECURITY_NUMBER) => {
                data.profile.social_security_number = None;
                ctx.submit_notification(PROFILE_UPDATED.with(data.profile.get_profile()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_TAX_ID) => {
                data.profile.tax_id = None;
                ctx.submit_notification(PROFILE_UPDATED.with(data.profile.get_profile()));
                ctx.set_handled();
            }
            Event::Notification(not) if not.is(CLEAR_POST_NUMBER) => {
                data.profile.post_number = None;
                ctx.submit_notification(PROFILE_UPDATED.with(data.profile.get_profile()));
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

                ctx.submit_notification(PROFILE_UPDATED.with(data.profile.get_profile()));
                ctx.set_handled();
            }
            _ => {
                child.event(ctx, event, data, env);
            }
        }
    }
}

pub fn build() -> impl Widget<MainState> {
    MainState::matcher()
        .home(build_screen())
        .create_id_card(
            create_id_card::build()
                .lens(CreateIdCardState::form_state)
                .controller(CreateIdCardController),
        )
        .create_social_security_number(
            create_social_security_number::build()
                .lens(CreateSocialSecurityNumberState::form_state)
                .controller(CreateSocialSecurityNumberController),
        )
        .create_tax_id(
            create_tax_id::build()
                .lens(CreateTaxIdState::form_state)
                .controller(CreateTaxIdController),
        )
        .create_post_number(
            create_post_number::build()
                .lens(CreatePostNumberState::form_state)
                .controller(CreatePostNumberController),
        )
        .create_bank_account(
            create_bank_account::build()
                .lens(CreateBankAccountState::form_state)
                .controller(CreateBankAccountController),
        )
        .controller(MainController)
}

fn build_screen() -> impl Widget<HomeState> {
    let sidebar = Flex::column()
        .must_fill_main_axis(true)
        .with_child(build_sidebar_header().lens(HomeState::profile.then(ProfileState::name)))
        .with_child(build_sidebar_navigation())
        .background(theme::BACKGROUND_LIGHT);

    let main = ViewSwitcher::new(
        |state: &HomeState, _env| state.nav,
        |nav, _state, _env| match nav {
            Nav::Home => Box::new(build_home().lens(HomeState::profile)),
            Nav::BankAccounts => Box::new(build_bank_account_page().lens(HomeState::profile)),
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
        .controller(HomeController)
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

fn build_sidebar_navigation() -> impl Widget<HomeState> {
    Flex::column()
        .with_default_spacer()
        .with_child(sidebar_link_widget("Basisdaten", Nav::Home))
        .with_child(sidebar_link_widget("Konten", Nav::BankAccounts))
}

fn sidebar_link_widget(title: &str, link_nav: Nav) -> impl Widget<HomeState> {
    Label::new(title)
        .with_text_size(20.0)
        .expand_width()
        .lens(HomeState::nav)
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
                            "Personalsausweis - {} Tage gültig",
                            state.time_until_expiration().num_days()
                        )
                    })
                    .with_text_size(12.0),
                ),
        )
        .with_flex_spacer(1.0)
        .with_child(
            OutlineButton::new("Löschen")
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
            OutlineButton::new("Löschen")
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
            OutlineButton::new("Löschen").on_click(|ctx, account: &mut BankAccount, _| {
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
