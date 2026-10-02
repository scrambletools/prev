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

    /// Where prev keeps its files.
    fn storage_settings(&self) -> iced::widget::Column<'_, Message> {
        column![self.storage_view()].spacing(12)
    }
}
