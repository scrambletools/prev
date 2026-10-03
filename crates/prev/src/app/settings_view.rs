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
            ContextSize, Found, ModelMessage, ProviderChoice, choice_of, context_sizes,
            provider_label, shown_name,
        };
        let send = |message: ModelMessage| Message::ModelSettings(message);
        let note = |text: String| {
            ui::aligned(ui::styled(text, Type::BodyMedium).style(style::on_surface_variant))
        };
        let small = |text: String| {
            ui::aligned(ui::styled(text, Type::BodySmall).style(style::on_surface_variant))
        };
        let context_menu = |size: u32, on_pick: Box<dyn Fn(ContextSize) -> Message>| {
            iced::widget::pick_list(context_sizes(), Some(ContextSize(size)), on_pick)
                .font(ui::font::TEXT)
                .text_size(14)
                .padding([6, 10])
                .style(style::outlined_select)
                .menu_style(style::select_menu)
        };
        let mut models = column![note(prev::fl!("settings-assistant-note"))].spacing(8);
        if self.settings.assistant_models.is_empty() {
            models = models.push(note(prev::fl!("settings-assistant-none")));
        }
        for model in &self.settings.assistant_models {
            let used = self.settings.assistant_model.as_deref() == Some(model.id.as_str());
            let choice = choice_of(model);
            let provider = choice.as_ref().map_or_else(
                || model.provider.clone(),
                |choice| provider_label(choice.provider),
            );
            let detail = match &model.address {
                Some(address) => format!("{provider} · {address}"),
                None => provider,
            };
            let context: Element<'_, Message> = match &choice {
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
            // One width, so the context menus line up.
            let action = container(action)
                .width(72)
                .align_x(iced::alignment::Horizontal::Center);
            models = models.push(
                row![
                    column![
                        ui::styled(shown_name(model), Type::BodyLarge),
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
        let provider = form.provider;
        let label = provider_label(provider);
        let provider_menu = iced::widget::pick_list(
            prev_assist::Provider::ALL.map(ProviderChoice).to_vec(),
            Some(ProviderChoice(provider)),
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
            provider_menu,
        ]
        .spacing(12);

        // How prev reaches the provider: its key, or its server.
        if provider.needs_key() {
            if form.asks_for_key() {
                add = add.push(
                    row![
                        container(component::text_field(
                            prev::fl!("settings-assistant-key"),
                            &form.key,
                            Backdrop::ContainerHigh,
                            |input| {
                                input
                                    .secure(true)
                                    .on_input(move |key| send(ModelMessage::Key(key)))
                                    .on_submit(send(ModelMessage::UseKey))
                            },
                        ))
                        .width(Fill),
                        ui::button(Kind::Filled, prev::fl!("settings-assistant-use-key"))
                            .on_press(send(ModelMessage::UseKey)),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
                if let Some(page) = prev_assist::key_page(provider) {
                    add = add.push(
                        row![
                            container(small(prev::fl!(
                                "settings-assistant-key-where",
                                provider = label.clone()
                            )))
                            .width(Fill),
                            ui::button(Kind::Text, prev::fl!("settings-assistant-get-key"))
                                .on_press(send(ModelMessage::OpenLink(page))),
                        ]
                        .spacing(8)
                        .align_y(Center),
                    );
                }
            } else if form.key_kept == Some(true) {
                add = add.push(
                    row![
                        container(note(prev::fl!(
                            "settings-assistant-key-kept",
                            provider = label.clone()
                        )))
                        .width(Fill),
                        ui::button(Kind::Text, prev::fl!("settings-assistant-change-key"))
                            .on_press(send(ModelMessage::ChangeKey)),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
            }
        } else {
            // The address field shows when no server answered, or one was
            // found at the address typed; it has its own Look again.
            let found = matches!(form.found, Found::Models(..) | Found::Looking);
            let field_shown = !(found && form.address.is_empty()) || form.other_address;
            let server: Element<'_, Message> = match &form.found {
                Found::Models(Some(address), _) => {
                    let mut line = row![
                        container(small(prev::fl!(
                            "settings-assistant-found-at",
                            provider = label.clone(),
                            address = address.clone()
                        )))
                        .width(Fill),
                    ]
                    .spacing(8)
                    .align_y(Center);
                    if !field_shown {
                        line = line
                            .push(
                                ui::button(
                                    Kind::Text,
                                    prev::fl!("settings-assistant-other-address"),
                                )
                                .on_press(send(ModelMessage::OtherAddress)),
                            )
                            .push(
                                ui::button(Kind::Text, prev::fl!("settings-assistant-look-again"))
                                    .on_press(send(ModelMessage::Search)),
                            );
                    }
                    line.into()
                }
                Found::NoServer => {
                    let mut lines = column![note(prev::fl!(
                        "settings-assistant-no-server",
                        provider = label.clone()
                    ))]
                    .spacing(8);
                    if let Some(page) = prev_assist::server_page(provider) {
                        lines = lines.push(
                            ui::button(
                                Kind::Tonal,
                                prev::fl!(
                                    "settings-assistant-get-server",
                                    provider = label.clone()
                                ),
                            )
                            .on_press(send(ModelMessage::OpenLink(page))),
                        );
                    }
                    lines.into()
                }
                _ => space().into(),
            };
            add = add.push(server);
            if field_shown {
                add = add.push(
                    row![
                        container(component::text_field(
                            prev::fl!(
                                "settings-assistant-address",
                                example = provider.default_address().unwrap_or_default()
                            ),
                            &form.address,
                            Backdrop::ContainerHigh,
                            |input| {
                                input
                                    .on_input(move |address| send(ModelMessage::Address(address)))
                                    .on_submit(send(ModelMessage::Search))
                            },
                        ))
                        .width(Fill),
                        ui::button(Kind::Text, prev::fl!("settings-assistant-look-again"))
                            .on_press_maybe(
                                (!matches!(form.found, Found::Looking))
                                    .then_some(send(ModelMessage::Search)),
                            ),
                    ]
                    .spacing(8)
                    .align_y(Center),
                );
            }
        }
        if provider.sets_context() && matches!(form.found, Found::Models(..)) {
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
                    small(prev::fl!("settings-assistant-context-note")),
                ]
                .spacing(4),
            );
        }

        // The models found, to add with one click.
        match &form.found {
            Found::Looking => add = add.push(note(prev::fl!("settings-assistant-looking"))),
            Found::Failed(problem) => {
                add = add.push(ui::aligned(
                    ui::styled(problem.clone(), Type::BodyMedium).style(style::error_text),
                ));
            }
            Found::Models(address, found) => {
                if found.is_empty() {
                    add = add.push(note(prev::fl!(
                        "settings-assistant-found-none",
                        provider = label.clone()
                    )));
                }
                for info in found {
                    // Ollama answers to a name with or without `:latest`.
                    let bare = info.id.strip_suffix(":latest").unwrap_or(&info.id);
                    let added = self.settings.assistant_models.iter().any(|model| {
                        let saved = model.model.strip_suffix(":latest").unwrap_or(&model.model);
                        saved == bare
                            && choice_of(model).is_some_and(|choice| choice.provider == provider)
                            && (model.address == *address
                                || model.address.is_none()
                                    && address.as_deref() == provider.default_address())
                    });
                    let mut tags = Vec::new();
                    if info.recommended {
                        tags.push(prev::fl!("settings-assistant-recommended"));
                    }
                    match info.tools {
                        Some(false) => tags.push(prev::fl!("settings-assistant-no-tools")),
                        Some(true) if !provider.needs_key() => {
                            tags.push(prev::fl!("settings-assistant-uses-tools"));
                        }
                        _ => {}
                    }
                    if info.vision == Some(true) && !provider.needs_key() {
                        tags.push(prev::fl!("settings-assistant-sees"));
                    }
                    let mut detail = info.id.clone();
                    if !tags.is_empty() {
                        detail = format!("{detail} · {}", tags.join(" · "));
                    }
                    let usable = info.tools != Some(false);
                    let button: Element<'_, Message> = if added {
                        ui::styled(prev::fl!("settings-assistant-added-tag"), Type::LabelLarge)
                            .style(style::on_surface_variant)
                            .into()
                    } else if form.adding.as_deref() == Some(info.id.as_str()) {
                        ui::styled(prev::fl!("settings-assistant-trying"), Type::LabelLarge)
                            .style(style::on_surface_variant)
                            .into()
                    } else {
                        ui::button(Kind::Tonal, prev::fl!("settings-assistant-add-button"))
                            .on_press_maybe(
                                (usable && form.adding.is_none())
                                    .then(|| send(ModelMessage::Add(info.id.clone()))),
                            )
                            .into()
                    };
                    let name = ui::styled(info.name.clone(), Type::BodyLarge);
                    let name = if usable {
                        name
                    } else {
                        name.style(style::on_surface_variant)
                    };
                    add = add.push(
                        row![column![name, small(detail)].spacing(2).width(Fill), button,]
                            .spacing(8)
                            .align_y(Center),
                    );
                    // Why its trial failed, on the row tried.
                    if let Some((_, why, help)) =
                        form.failed.as_ref().filter(|(id, ..)| *id == info.id)
                    {
                        let mut line = row![
                            container(ui::aligned(
                                ui::styled(why.clone(), Type::BodySmall)
                                    .style(style::error_text)
                                    .wrapping(iced::widget::text::Wrapping::WordOrGlyph),
                            ))
                            .width(Fill)
                        ]
                        .spacing(8)
                        .align_y(Center);
                        if let Some(page) = help {
                            line = line.push(
                                ui::button(Kind::Text, prev::fl!("settings-assistant-fix-it"))
                                    .on_press(send(ModelMessage::OpenLink(page))),
                            );
                        }
                        add = add.push(line);
                    }
                }
            }
            Found::Nothing | Found::NoServer => {}
        }

        // A model the provider does not list, by its name.
        let reachable = match &form.found {
            Found::Models(..) => true,
            Found::NoServer | Found::Looking => false,
            Found::Nothing | Found::Failed(_) => {
                !provider.needs_key() || form.key_kept == Some(true)
            }
        };
        if reachable {
            add = add.push(
                row![
                    container(component::text_field(
                        prev::fl!(
                            "settings-assistant-model",
                            example = prev_assist::example_model(provider)
                        ),
                        &form.typed,
                        Backdrop::ContainerHigh,
                        |input| {
                            input
                                .on_input(move |name| send(ModelMessage::Typed(name)))
                                .on_submit(send(ModelMessage::Add(form.typed.clone())))
                        },
                    ))
                    .width(Fill),
                    ui::button(Kind::Text, prev::fl!("settings-assistant-add-button"))
                        .on_press_maybe(
                            (!form.typed.trim().is_empty() && form.adding.is_none())
                                .then(|| send(ModelMessage::Add(form.typed.clone()))),
                        ),
                ]
                .spacing(8)
                .align_y(Center),
            );
        }
        match &form.status {
            Some(Ok(done)) => add = add.push(note(done.clone())),
            Some(Err(problem)) => {
                add = add.push(ui::aligned(
                    ui::styled(problem.clone(), Type::BodyMedium).style(style::error_text),
                ));
            }
            None => {}
        }
        column![models, add].spacing(20)
    }

    /// Where prev keeps its files.
    fn storage_settings(&self) -> iced::widget::Column<'_, Message> {
        column![self.storage_view()].spacing(12)
    }
}
