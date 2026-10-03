//! The Settings dialog: a fixed header with the title, close button and
//! tabs, and below it the chosen tab's settings, which scroll.

use iced::widget::{container, mouse_area, opaque, space, toggler};
use iced::{Center, Element, Fill, Length, Theme};
use prev::ui::button::Kind;
use prev::ui::component::Backdrop;
use prev::ui::dir::row;
use prev::ui::{self, Icon, Type, component, style};
use prev::{column, row};
use prev_store::settings::Appearance;

use iced::window;

use super::{ACCENT_SWATCHES, Message, Prev, accent_swatch, chosen_accent, color_to_hex};

/// The tabs Settings is split into.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum SettingsTab {
    #[default]
    General,
    Appearance,
    Assistant,
    Agents,
    Storage,
}

/// How tall the dialog is at most, so it keeps one height across tabs.
const DIALOG_HEIGHT: f32 = 720.0;

/// A setting with a title, a note under it and a switch.
fn switch_row<'a>(
    title: String,
    note: String,
    on: bool,
    toggled: fn(bool) -> Message,
) -> Element<'a, Message> {
    row![
        column![
            ui::styled(title, Type::BodyLarge),
            ui::aligned(ui::styled(note, Type::BodyMedium).style(style::on_surface_variant)),
        ]
        .spacing(2)
        .width(Fill),
        toggler(on).on_toggle(toggled).size(28).style(style::switch),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

/// A setting with a title, a note, its value and a slider.
fn slider_row<'a>(
    title: String,
    note: String,
    value_label: String,
    range: std::ops::RangeInclusive<f32>,
    value: f32,
    step: f32,
    changed: fn(f32) -> Message,
) -> Element<'a, Message> {
    row![
        column![
            ui::styled(title, Type::BodyLarge),
            ui::aligned(ui::styled(note, Type::BodyMedium).style(style::on_surface_variant)),
        ]
        .spacing(2)
        .width(Fill),
        ui::styled(value_label, Type::LabelLarge).style(style::on_surface_variant),
        iced::widget::slider(range, value, changed)
            .step(step)
            .on_release(Message::SliderReleased)
            .width(160)
            .height(style::SLIDER_HEIGHT)
            .style(|theme: &Theme, status| {
                style::slider(Backdrop::ContainerHigh.color(&ui::Scheme::of(theme)))(theme, status)
            }),
    ]
    .spacing(16)
    .align_y(Center)
    .into()
}

impl Prev {
    pub(super) fn settings_dialog(&self, id: window::Id) -> Element<'_, Message> {
        let tab = |label: String, which: SettingsTab| component::Tab {
            label: prev::i18n::lasting(label),
            icon: None,
            selected: self.settings_tab == which,
            on_press: Message::SettingsTab(which),
        };
        let tabs = component::tabs(vec![
            tab(prev::fl!("settings-tab-general"), SettingsTab::General),
            tab(prev::fl!("settings-appearance"), SettingsTab::Appearance),
            tab(prev::fl!("settings-tab-assistant"), SettingsTab::Assistant),
            tab(prev::fl!("settings-tab-agents"), SettingsTab::Agents),
            tab(prev::fl!("settings-storage"), SettingsTab::Storage),
        ]);
        // The title, close button and tabs stay put while the settings
        // scroll.
        let header = column![
            row![
                ui::aligned(ui::styled(prev::fl!("settings-title"), Type::HeadlineSmall)),
                component::tip(
                    ui::icon_button(Icon::Close).on_press(Message::CloseSettings(id)),
                    prev::fl!("common-close")
                ),
            ]
            .align_y(Center),
            tabs,
        ]
        .spacing(12);
        let mut content = match self.settings_tab {
            SettingsTab::General => self.general_settings(),
            SettingsTab::Appearance => self.appearance_settings(),
            SettingsTab::Assistant => self.assistant_settings(),
            SettingsTab::Agents => self.agent_settings(),
            SettingsTab::Storage => self.storage_settings(),
        };
        if let Some(error) = &self.settings_error {
            content = content.push(ui::aligned(
                ui::styled(error, Type::BodyMedium).style(style::error_text),
            ));
        }
        let version = env!("CARGO_PKG_VERSION");
        let build = if prev_store::paths::PRODUCTION {
            prev::fl!(
                "settings-version",
                version = version,
                build = env!("PREV_COMMIT")
            )
        } else {
            let build = concat!(env!("PREV_COMMIT"), ", ", env!("PREV_BUILT"), " UTC");
            prev::fl!(
                "settings-version-development",
                version = version,
                build = build
            )
        };
        let footer = container(ui::styled(build, Type::BodySmall).style(style::on_surface_variant))
            .padding(iced::Padding {
                top: 12.0,
                right: 24.0,
                bottom: 16.0,
                left: 24.0,
            })
            .center_x(Fill);
        // One height for every tab, as tall as the window allows, so the
        // dialog does not jump when the tab changes.
        let window_height = self
            .windows
            .get(&id)
            .map_or(DIALOG_HEIGHT, |window| window.size.height);
        let height = (window_height - 48.0).clamp(240.0, DIALOG_HEIGHT);
        let card = container(column![
            container(header).padding(iced::Padding {
                top: 24.0,
                right: 24.0,
                bottom: 0.0,
                left: 24.0,
            }),
            container(component::scroll(container(content).padding(
                iced::Padding {
                    top: 16.0,
                    right: 24.0,
                    bottom: 8.0,
                    left: 24.0,
                }
            )))
            .height(Fill),
            footer,
        ])
        .width(Length::Fixed(520.0))
        .height(Length::Fixed(height))
        .style(style::dialog);
        // A click on the dimmed window around the dialog closes it.
        mouse_area(
            container(ui::enter::grow(opaque(card)))
                .padding(24)
                .center(Fill)
                .style(style::scrim),
        )
        .on_press(Message::CloseSettings(id))
        .into()
    }

    /// Languages and the default app.
    fn general_settings(&self) -> iced::widget::Column<'_, Message> {
        column![
            component::section(prev::fl!("settings-language")),
            self.language_field(),
            component::section(prev::fl!("settings-input-language")),
            self.input_language_field(),
            ui::aligned(
                ui::styled(prev::fl!("settings-input-language-note"), Type::BodySmall)
                    .style(style::on_surface_variant)
            ),
            component::section(prev::fl!("settings-default-app")),
            self.default_app_view(),
        ]
        .spacing(12)
    }

    /// Light or dark, colors, and how windows look and move.
    fn appearance_settings(&self) -> iced::widget::Column<'_, Message> {
        let appearance = self.settings.appearance;
        let choice = |glyph: Icon, label: String, value: Appearance| {
            ui::with_icon(Kind::Tonal, glyph, label)
                .selected(appearance == value)
                .on_press(Message::AppearanceSelected(value))
        };
        let accent_note = match (
            self.settings.system_accent,
            &self.omarchy,
            self.system_accent,
        ) {
            (false, _, _) => prev::fl!("settings-accent-chosen-note"),
            (true, Some(palette), _) => {
                prev::fl!("settings-omarchy-note", theme = palette.name.as_str())
            }
            (true, None, Some(_)) => prev::fl!("settings-system-accent-note"),
            (true, None, None) => prev::fl!("settings-system-accent-none"),
        };
        // The picked color is in use when the system accent is off or the
        // system has none.
        let picking = !self.settings.system_accent
            || (self.omarchy.is_none() && self.system_accent.is_none());
        let chosen = chosen_accent(&self.settings);
        let swatches = picking.then(|| {
            row(ACCENT_SWATCHES
                .iter()
                .map(|&swatch| accent_swatch(swatch, color_to_hex(swatch) == color_to_hex(chosen))))
            .spacing(4)
            .wrap()
        });
        let animations_note = if self.system_animations {
            prev::fl!("settings-animations-note")
        } else {
            prev::fl!("settings-animations-reduced")
        };
        column![
            component::connected(vec![
                choice(
                    Icon::Settings,
                    prev::fl!("settings-appearance-system"),
                    Appearance::System
                ),
                choice(
                    Icon::LightMode,
                    prev::fl!("settings-appearance-light"),
                    Appearance::Light
                ),
                choice(
                    Icon::DarkMode,
                    prev::fl!("settings-appearance-dark"),
                    Appearance::Dark
                ),
            ]),
            component::section(prev::fl!("settings-colors")),
            switch_row(
                prev::fl!("settings-system-accent"),
                accent_note,
                self.settings.system_accent,
                Message::SystemAccentToggled,
            ),
            swatches.map_or_else(|| Element::from(space()), Element::from),
            component::section(prev::fl!("settings-windows")),
            switch_row(
                prev::fl!("settings-auto-hide"),
                prev::fl!("settings-auto-hide-note"),
                self.settings.auto_hide_toolbar,
                Message::AutoHideToolbarToggled,
            ),
            switch_row(
                prev::fl!("settings-animations"),
                animations_note,
                self.settings.animations,
                Message::AnimationsToggled,
            ),
            slider_row(
                prev::fl!("settings-corner-radius"),
                prev::fl!("settings-corner-radius-note"),
                prev::fl!(
                    "settings-corner-radius-value",
                    radius = format!("{:.0}", self.settings.corner_radius)
                ),
                0.0..=32.0,
                self.settings.corner_radius,
                1.0,
                Message::CornerRadius,
            ),
            slider_row(
                prev::fl!("settings-overlay"),
                prev::fl!("settings-overlay-note"),
                prev::fl!(
                    "settings-overlay-value",
                    percent = format!("{:.0}", self.settings.overlay_transparency)
                ),
                0.0..=90.0,
                self.settings.overlay_transparency,
                5.0,
                Message::OverlayTransparency,
            ),
        ]
        .spacing(12)
    }

    /// Outside control: whether agents may control prev, and what prev
    /// asks about first.
    fn agent_settings(&self) -> iced::widget::Column<'_, Message> {
        column![
            switch_row(
                prev::fl!("settings-allow-outside-control"),
                prev::fl!("settings-allow-outside-control-note"),
                self.settings.outside_control,
                Message::OutsideControlToggled,
            ),
            self.allowed_agents_view(),
            self.ask_before_view(),
        ]
        .spacing(12)
    }

    /// The models the assistant panel can use, and adding one.
    fn assistant_settings(&self) -> iced::widget::Column<'_, Message> {
        use super::assistant::{
            ContextSize, ModelMessage, ProviderChoice, choice_of, context_sizes, provider_label,
        };
        let context_menu = |size: u32, on_pick: Box<dyn Fn(ContextSize) -> Message>| {
            iced::widget::pick_list(context_sizes(), Some(ContextSize(size)), on_pick)
                .font(ui::font::TEXT)
                .text_size(14)
                .padding([6, 10])
                .style(style::outlined_select)
                .menu_style(style::select_menu)
        };
        let send = |message: ModelMessage| Message::ModelSettings(message);
        let mut models = column![ui::aligned(
            ui::styled(prev::fl!("settings-assistant-note"), Type::BodyMedium)
                .style(style::on_surface_variant)
        )]
        .spacing(8);
        if self.settings.assistant_models.is_empty() {
            models = models.push(ui::aligned(
                ui::styled(prev::fl!("settings-assistant-none"), Type::BodyMedium)
                    .style(style::on_surface_variant),
            ));
        }
        for model in &self.settings.assistant_models {
            let used = self.settings.assistant_model.as_deref() == Some(model.id.as_str());
            let provider = choice_of(model).map_or_else(
                || model.provider.clone(),
                |choice| provider_label(choice.provider),
            );
            let detail = match &model.address {
                Some(address) => format!("{provider} · {address}"),
                None => provider,
            };
            let context: Element<'_, Message> = match choice_of(model) {
                Some(choice) if choice.provider.sets_context() => {
                    let id = model.id.clone();
                    component::tip(
                        context_menu(
                            model.context.unwrap_or(prev_assist::DEFAULT_CONTEXT),
                            Box::new(move |size| send(ModelMessage::Context(id.clone(), size))),
                        ),
                        prev::fl!("settings-assistant-context"),
                    )
                }
                _ => space().into(),
            };
            let action: Element<'_, Message> = if used {
                ui::styled(prev::fl!("settings-assistant-in-use"), Type::LabelLarge)
                    .style(style::on_surface_variant)
                    .into()
            } else {
                ui::button(Kind::Text, prev::fl!("settings-assistant-use"))
                    .on_press(send(ModelMessage::Use(model.id.clone())))
                    .into()
            };
            models = models.push(
                row![
                    column![
                        ui::styled(model.model.clone(), Type::BodyLarge),
                        ui::styled(detail, Type::BodyMedium).style(style::on_surface_variant),
                    ]
                    .spacing(2)
                    .width(Fill),
                    context,
                    action,
                    component::tip(
                        ui::icon_button(Icon::Delete)
                            .on_press(send(ModelMessage::Remove(model.id.clone()))),
                        prev::fl!("settings-assistant-remove")
                    ),
                ]
                .spacing(8)
                .align_y(Center),
            );
        }
        let form = &self.model_form;
        let provider = iced::widget::pick_list(
            prev_assist::Provider::ALL.map(ProviderChoice).to_vec(),
            Some(ProviderChoice(form.provider)),
            |choice| Message::ModelSettings(ModelMessage::Provider(choice)),
        )
        .font(ui::font::TEXT)
        .text_size(16)
        .padding([10, 12])
        .width(Fill)
        .style(style::outlined_select)
        .menu_style(style::select_menu);
        let mut add = column![
            component::section(prev::fl!("settings-assistant-add")),
            provider,
            component::text_field(
                prev::fl!(
                    "settings-assistant-model",
                    example = prev_assist::example_model(form.provider)
                ),
                &form.model,
                Backdrop::ContainerHigh,
                |input| input.on_input(|model| Message::ModelSettings(ModelMessage::Model(model))),
            ),
        ]
        .spacing(12);
        if form.provider.needs_key() {
            add = add.push(component::text_field(
                prev::fl!("settings-assistant-key"),
                &form.key,
                Backdrop::ContainerHigh,
                |input| {
                    input
                        .secure(true)
                        .on_input(|key| Message::ModelSettings(ModelMessage::Key(key)))
                },
            ));
        }
        if form.provider.needs_address() {
            add = add.push(component::text_field(
                prev::fl!(
                    "settings-assistant-address",
                    example = form.provider.default_address().unwrap_or_default()
                ),
                &form.address,
                Backdrop::ContainerHigh,
                |input| {
                    input.on_input(|address| Message::ModelSettings(ModelMessage::Address(address)))
                },
            ));
        }
        if form.provider.sets_context() {
            add = add.push(
                column![
                    row![
                        container(ui::aligned(ui::styled(
                            prev::fl!("settings-assistant-context"),
                            Type::BodyLarge
                        )))
                        .width(Fill),
                        context_menu(
                            form.context,
                            Box::new(move |size| send(ModelMessage::FormContext(size))),
                        ),
                    ]
                    .spacing(8)
                    .align_y(Center),
                    ui::aligned(
                        ui::styled(
                            prev::fl!("settings-assistant-context-note"),
                            Type::BodySmall
                        )
                        .style(style::on_surface_variant)
                    ),
                ]
                .spacing(4),
            );
        }
        let status: Element<'_, Message> = match &form.status {
            Some(Ok(note)) => ui::aligned(
                ui::styled(note.clone(), Type::BodyMedium).style(style::on_surface_variant),
            ),
            Some(Err(problem)) => {
                ui::aligned(ui::styled(problem.clone(), Type::BodyMedium).style(style::error_text))
            }
            None => space().into(),
        };
        let idle = !form.busy;
        add = add.push(
            row![
                container(status).width(Fill),
                ui::button(Kind::Text, prev::fl!("settings-assistant-test"))
                    .on_press_maybe(idle.then_some(send(ModelMessage::Test))),
                ui::button(Kind::Filled, prev::fl!("settings-assistant-add-button"))
                    .on_press_maybe(idle.then_some(send(ModelMessage::Add))),
            ]
            .spacing(8)
            .align_y(Center),
        );
        column![models, add].spacing(20)
    }

    /// Where prev keeps its files.
    fn storage_settings(&self) -> iced::widget::Column<'_, Message> {
        column![self.storage_view()].spacing(12)
    }
}
