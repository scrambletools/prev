//! The assistant panel: a window's chat with the model in use, on the
//! right. The model's tool calls run one at a time through the same
//! registry and approvals as an outside agent's, on the panel's window
//! unless they name another.

use std::collections::VecDeque;

use base64::Engine;
use iced::widget::{
    Id, column, container, image, markdown, operation, pick_list, row, space, text,
};
use iced::{Center, Element, Fill, Length, Padding, Task, window};
use prev::control::{Call, Error};
use prev::ui::button::Kind;
use prev::ui::{self, Icon, Type, component, style};
use prev_assist::{Chat, Content, Event, ToolCall, ToolResult};
use serde_json::{Value, json};

use super::super::{Message, Prev, tools};
use super::{choice_of, key_of};
use crate::External;

/// The panel's width.
const WIDTH: f32 = 380.0;

/// The widest a tool's picture shows in the panel.
const PICTURE_WIDTH: f32 = 300.0;

/// What the model is told about prev and the panel.
const INSTRUCTIONS: &str = "You are the assistant in prev, a PDF, image and Markdown viewer, \
chatting with the user in a panel beside their file. Use the tools to look at the file and act on \
it; they act on the user's window unless you give another window's number. You get only what you \
ask for, so read the file with page_text, search or render_page before you answer about it. \
Positions on PDF pages are in points from the page's top-left corner, with y growing down; on an \
image's markup, in image pixels. Every change saves to the file by itself, as edits in prev do, \
and is one step of the user's Undo; there is nothing to save. The user's settings may make prev \
ask them before a tool runs. Answer briefly, in the user's language; you may use Markdown.";

/// What the panel heard from its chat's thread or a tool call, through
/// [`crate::post`].
#[derive(Debug, Clone)]
pub(crate) enum Heard {
    Chat(u64, Event),
    /// A tool call's answer: the chat's and the turn's numbers, and the
    /// call's id.
    Tool(u64, u64, String, Result<Value, Error>),
}

/// What the panel asks for.
#[derive(Debug, Clone)]
pub(crate) enum PanelMessage {
    Input(String),
    Send,
    Stop,
    NewChat,
    Model(ModelPick),
    /// The key for a chat about to start: the chat's number, the model's
    /// id, and the key, if it has one.
    Keyed(u64, String, Result<Option<String>, String>),
    /// The key for the model the chat changes to.
    Switched(u64, String, Result<Option<String>, String>),
    Link(markdown::Uri),
    OpenSettings,
}

/// A model in the panel's menu.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ModelPick {
    id: String,
    name: String,
}

impl std::fmt::Display for ModelPick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.name)
    }
}

enum Entry {
    User(String),
    Reply(Box<markdown::Content>),
    Tool {
        id: String,
        title: String,
        state: ToolState,
        picture: Option<image::Handle>,
    },
    /// A line from prev, such as the model changing.
    Note(String),
    Problem(String),
}

enum ToolState {
    Waiting,
    Running,
    Done,
    Failed(String),
}

/// One window's panel, kept while it is closed so the chat goes on when
/// it opens again.
pub(crate) struct Panel {
    pub(crate) open: bool,
    input_id: Id,
    entries: Vec<Entry>,
    input: String,
    chat: Option<Chat>,
    /// The model the chat talks to, by its id in Settings.
    model: Option<String>,
    /// Counts the chats this panel started, so a chat's events reach only
    /// its own panel state, never a New Chat's.
    chat_number: u64,
    /// Counts the turns stopped, so a stopped turn's tool answers are
    /// dropped.
    turn: u64,
    /// A turn is under way, or the chat is starting.
    busy: bool,
    /// What the user said while the chat was starting.
    unsent: Option<String>,
    /// Tool calls waiting their turn, and the results so far.
    calls: VecDeque<ToolCall>,
    results: Vec<ToolResult>,
    running: bool,
}

impl Panel {
    pub(crate) fn new(model: Option<String>) -> Self {
        Self {
            open: true,
            input_id: Id::unique(),
            entries: Vec::new(),
            input: String::new(),
            chat: None,
            model,
            chat_number: 0,
            turn: 0,
            busy: false,
            unsent: None,
            calls: VecDeque::new(),
            results: Vec::new(),
            running: false,
        }
    }
}

impl Prev {
    /// Opens window `id`'s panel, with its chat so far, or closes it.
    pub(crate) fn toggle_assistant(&mut self, id: window::Id) -> Task<Message> {
        let model = self.settings.assistant_model.clone();
        let Some(window) = self.windows.get_mut(&id) else {
            return Task::none();
        };
        let panel = window.assistant.get_or_insert_with(|| {
            let mut panel = Panel::new(model);
            panel.open = false;
            panel
        });
        panel.open = !panel.open;
        let (open, input) = (panel.open, panel.input_id.clone());
        match &mut window.content {
            super::super::Content::Document(document) => {
                if let Some(pdf) = document.pdf.as_mut() {
                    pdf.set_assistant_shown(open);
                }
                if let Some(images) = document.images.as_mut() {
                    images.set_assistant_shown(open);
                }
                if let Some(markdown) = document.markdown.as_mut() {
                    markdown.set_assistant_shown(open);
                }
            }
            super::super::Content::Start => {}
        }
        if open {
            operation::focus(input)
        } else {
            Task::none()
        }
    }

    /// Tells the chats that talk to model `id` its settings changed, so
    /// their next turns use the new ones. Only local models change, which
    /// have no key.
    pub(crate) fn model_changed(&mut self, id: &str) {
        let Some(choice) = self
            .settings
            .assistant_models
            .iter()
            .find(|model| model.id == id)
            .and_then(choice_of)
        else {
            return;
        };
        for window in self.windows.values() {
            if let Some(panel) = &window.assistant
                && panel.model.as_deref() == Some(id)
                && let Some(chat) = &panel.chat
            {
                chat.set_model(choice.clone(), None);
            }
        }
    }

    /// The model the panel of window `id` uses: its own, if Settings still
    /// has it, else the one in use.
    fn panel_model(&self, panel: &Panel) -> Option<&prev_store::settings::AssistantModel> {
        let models = &self.settings.assistant_models;
        panel
            .model
            .as_deref()
            .and_then(|id| models.iter().find(|model| model.id == id))
            .or_else(|| {
                let id = self.settings.assistant_model.as_deref()?;
                models.iter().find(|model| model.id == id)
            })
            .or_else(|| models.first())
    }

    pub(crate) fn panel_update(&mut self, id: window::Id, message: PanelMessage) -> Task<Message> {
        let Some(mut panel) = self.windows.get_mut(&id).and_then(|w| w.assistant.take()) else {
            return Task::none();
        };
        let task = self.panel_message(id, &mut panel, message);
        if let Some(window) = self.windows.get_mut(&id) {
            window.assistant = Some(panel);
        }
        task
    }

    fn panel_message(
        &mut self,
        id: window::Id,
        panel: &mut Panel,
        message: PanelMessage,
    ) -> Task<Message> {
        match message {
            PanelMessage::Input(text) => panel.input = text,
            PanelMessage::Send => {
                let said = panel.input.trim().to_owned();
                if said.is_empty() || panel.busy {
                    return Task::none();
                }
                panel.input.clear();
                panel.entries.push(Entry::User(said.clone()));
                panel.busy = true;
                if let Some(chat) = &panel.chat {
                    chat.say(said);
                    return Task::none();
                }
                let Some(model) = self.panel_model(panel).cloned() else {
                    panel.busy = false;
                    panel
                        .entries
                        .push(Entry::Problem(prev::fl!("assistant-no-model")));
                    return Task::none();
                };
                panel.unsent = Some(said);
                panel.model = Some(model.id.clone());
                let number = panel.chat_number;
                return Task::perform(
                    prev::image::editor::spawn(move || {
                        key_of(&model.id).map(|key| (model.id, key))
                    }),
                    move |result| {
                        let (model, key) = match result {
                            Ok(Ok((model, key))) => (model, Ok(key)),
                            Ok(Err(error)) => (String::new(), Err(error)),
                            Err(_) => (String::new(), Err("The keychain stopped.".to_owned())),
                        };
                        Message::Assistant(id, PanelMessage::Keyed(number, model, key))
                    },
                );
            }
            PanelMessage::Keyed(number, model_id, key) => {
                if number != panel.chat_number {
                    return Task::none();
                }
                let said = panel.unsent.take().unwrap_or_default();
                let started = key.and_then(|key| {
                    let model = self
                        .settings
                        .assistant_models
                        .iter()
                        .find(|model| model.id == model_id)
                        .and_then(choice_of)
                        .ok_or_else(|| prev::fl!("assistant-no-model"))?;
                    Chat::start(
                        model,
                        key,
                        INSTRUCTIONS.to_owned(),
                        tools::specs(&self.settings.ask_before),
                        move |event| {
                            crate::post(External::Assistant(id, Heard::Chat(number, event)));
                        },
                    )
                });
                match started {
                    Ok(chat) => {
                        chat.say(said);
                        panel.chat = Some(chat);
                    }
                    Err(error) => {
                        panel.busy = false;
                        panel.entries.push(Entry::Problem(error));
                    }
                }
            }
            PanelMessage::Stop => {
                if let Some(chat) = &panel.chat {
                    chat.stop();
                }
                panel.turn += 1;
                panel.calls.clear();
                panel.results.clear();
                panel.running = false;
                panel.unsent = None;
                panel.busy = false;
                for entry in &mut panel.entries {
                    if let Entry::Tool { state, .. } = entry
                        && matches!(state, ToolState::Waiting | ToolState::Running)
                    {
                        *state = ToolState::Failed(prev::fl!("assistant-stopped"));
                    }
                }
            }
            PanelMessage::NewChat => {
                let model = panel.model.take();
                *panel = Panel {
                    chat_number: panel.chat_number + 1,
                    input_id: panel.input_id.clone(),
                    ..Panel::new(model)
                };
                return operation::focus(panel.input_id.clone());
            }
            PanelMessage::Model(pick) => {
                if panel.model.as_deref() == Some(pick.id.as_str()) {
                    return Task::none();
                }
                panel.model = Some(pick.id.clone());
                // The next panel opened starts with it too.
                self.settings.assistant_model = Some(pick.id.clone());
                self.save_settings();
                if panel.chat.is_none() {
                    return Task::none();
                }
                panel.entries.push(Entry::Note(prev::fl!(
                    "assistant-switched",
                    model = pick.name
                )));
                let number = panel.chat_number;
                return Task::perform(
                    prev::image::editor::spawn(move || key_of(&pick.id).map(|key| (pick.id, key))),
                    move |result| {
                        let (model, key) = match result {
                            Ok(Ok((model, key))) => (model, Ok(key)),
                            Ok(Err(error)) => (String::new(), Err(error)),
                            Err(_) => (String::new(), Err("The keychain stopped.".to_owned())),
                        };
                        Message::Assistant(id, PanelMessage::Switched(number, model, key))
                    },
                );
            }
            PanelMessage::Switched(number, model_id, key) => {
                if number != panel.chat_number {
                    return Task::none();
                }
                let choice = self
                    .settings
                    .assistant_models
                    .iter()
                    .find(|model| model.id == model_id)
                    .and_then(choice_of);
                match (key, choice, &panel.chat) {
                    (Ok(key), Some(choice), Some(chat)) => chat.set_model(choice, key),
                    (Err(error), ..) => panel.entries.push(Entry::Problem(error)),
                    _ => {}
                }
            }
            PanelMessage::Link(uri) => {
                return Task::perform(prev::portal::open_uri(uri), |_| Message::Nothing);
            }
            PanelMessage::OpenSettings => {
                self.settings_tab = super::super::settings_view::SettingsTab::Assistant;
                return self.perform(id, super::super::Action::Settings);
            }
        }
        Task::none()
    }

    /// What window `id`'s chat or a tool call said.
    pub(crate) fn panel_heard(&mut self, id: window::Id, heard: Heard) -> Task<Message> {
        let Some(mut panel) = self.windows.get_mut(&id).and_then(|w| w.assistant.take()) else {
            return Task::none();
        };
        let task = match heard {
            Heard::Chat(number, event) if number == panel.chat_number => {
                self.chat_event(id, &mut panel, event)
            }
            Heard::Tool(number, turn, call, outcome)
                if number == panel.chat_number && turn == panel.turn =>
            {
                self.tool_answered(id, &mut panel, call, outcome)
            }
            Heard::Chat(..) | Heard::Tool(..) => Task::none(),
        };
        if let Some(window) = self.windows.get_mut(&id) {
            window.assistant = Some(panel);
        }
        task
    }

    fn chat_event(&mut self, id: window::Id, panel: &mut Panel, event: Event) -> Task<Message> {
        match event {
            Event::Text(more) => match panel.entries.last_mut() {
                Some(Entry::Reply(reply)) => reply.push_str(&more),
                _ => panel
                    .entries
                    .push(Entry::Reply(Box::new(markdown::Content::parse(&more)))),
            },
            Event::ToolCalls(calls) => {
                for call in &calls {
                    panel.entries.push(Entry::Tool {
                        id: call.id.clone(),
                        title: tools::find(&call.name)
                            .map_or_else(|| call.name.clone(), |tool| tool.title.to_owned()),
                        state: ToolState::Waiting,
                        picture: None,
                    });
                }
                panel.calls.extend(calls);
                return self.next_call(id, panel);
            }
            Event::Done | Event::Stopped => panel.busy = false,
            Event::Failed(problem) => {
                panel.busy = false;
                let text = match self
                    .panel_model(panel)
                    .map(|model| (choice_of(model), model))
                {
                    Some((Some(choice), model)) => {
                        super::problem_text(&problem, choice.provider, &model.model)
                    }
                    _ => format!("{problem:?}"),
                };
                panel.entries.push(Entry::Problem(text));
            }
        }
        Task::none()
    }

    /// Runs the next tool call waiting, or sends the model the results
    /// once none are left.
    fn next_call(&mut self, id: window::Id, panel: &mut Panel) -> Task<Message> {
        if panel.running {
            return Task::none();
        }
        let Some(call) = panel.calls.pop_front() else {
            if let Some(chat) = &panel.chat
                && !panel.results.is_empty()
            {
                chat.results(std::mem::take(&mut panel.results));
            }
            return Task::none();
        };
        panel.running = true;
        set_state(panel, &call.id, ToolState::Running);
        let mut arguments = match call.arguments {
            Value::Object(arguments) => arguments,
            _ => serde_json::Map::new(),
        };
        if tools::takes_window(&call.name) && !arguments.contains_key("window") {
            arguments.insert("window".to_owned(), json!(tools::number(id)));
        }
        let (number, turn, call_id) = (panel.chat_number, panel.turn, call.id);
        let local = Call::local(
            "tools/call",
            json!({ "name": call.name, "arguments": arguments }),
            move |outcome| {
                crate::post(External::Assistant(
                    id,
                    Heard::Tool(number, turn, call_id, outcome),
                ));
            },
        );
        let agent = self
            .panel_model(panel)
            .map_or_else(|| prev::fl!("assistant-title"), |model| model.model.clone());
        self.run_tool(local, agent)
    }

    fn tool_answered(
        &mut self,
        id: window::Id,
        panel: &mut Panel,
        call: String,
        outcome: Result<Value, Error>,
    ) -> Task<Message> {
        panel.running = false;
        let result = tool_result(call.clone(), outcome);
        let picture = result.content.iter().find_map(|part| match part {
            Content::Png(png) => picture(png),
            Content::Text(_) => None,
        });
        let state = if result.failed {
            let text = result
                .content
                .iter()
                .find_map(|part| match part {
                    Content::Text(text) => Some(text.clone()),
                    Content::Png(_) => None,
                })
                .unwrap_or_default();
            ToolState::Failed(text)
        } else {
            ToolState::Done
        };
        set_state(panel, &call, state);
        if let Some(Entry::Tool { picture: shown, .. }) = panel
            .entries
            .iter_mut()
            .rev()
            .find(|entry| matches!(entry, Entry::Tool { id, .. } if *id == call))
        {
            *shown = picture;
        }
        panel.results.push(result);
        self.next_call(id, panel)
    }

    /// Window `id`'s panel, or nothing when it is closed.
    pub(crate) fn panel_view(&self, id: window::Id) -> Option<Element<'_, Message>> {
        let panel = self
            .windows
            .get(&id)?
            .assistant
            .as_ref()
            .filter(|panel| panel.open)?;
        let message = move |message: PanelMessage| Message::Assistant(id, message);
        let picks: Vec<ModelPick> = self
            .settings
            .assistant_models
            .iter()
            .map(|model| ModelPick {
                id: model.id.clone(),
                name: model.model.clone(),
            })
            .collect();
        let chosen = self.panel_model(panel).map(|model| ModelPick {
            id: model.id.clone(),
            name: model.model.clone(),
        });
        let header = row![
            ui::styled(prev::fl!("assistant-title"), Type::TitleLarge),
            space::horizontal(),
            component::tool(
                Icon::EditSquare,
                prev::fl!("assistant-new-chat"),
                (!panel.entries.is_empty()).then_some(message(PanelMessage::NewChat)),
            ),
            component::tool(
                Icon::Close,
                prev::fl!("common-close"),
                Some(Message::ToggleAssistant(id)),
            ),
        ]
        .spacing(4)
        .align_y(Center)
        .padding(ui::dir::padding(12.0, 12.0, 4.0, 24.0));
        let has_models = !picks.is_empty();
        let model_row: Element<'_, Message> = if !has_models {
            column![
                ui::aligned(
                    ui::styled(prev::fl!("assistant-add-model"), Type::BodyMedium)
                        .style(style::on_surface_variant)
                ),
                ui::button(Kind::Tonal, prev::fl!("assistant-open-settings"))
                    .on_press(message(PanelMessage::OpenSettings)),
            ]
            .spacing(8)
            .into()
        } else {
            pick_list(picks, chosen, move |pick| {
                message(PanelMessage::Model(pick))
            })
            .width(Fill)
            .into()
        };
        let model_row = container(model_row).padding([0, 24]);

        let theme = &self.theme;
        let mut entries = column![].spacing(12);
        for entry in &panel.entries {
            entries = entries.push(entry_view(entry, theme, id));
        }
        if panel.busy && !panel.running {
            entries = entries.push(ui::aligned(
                ui::styled(prev::fl!("assistant-thinking"), Type::BodySmall)
                    .style(style::on_surface_variant),
            ));
        }
        let entries = component::scroll(container(entries).padding(Padding {
            top: 12.0,
            right: 24.0,
            bottom: 12.0,
            left: 24.0,
        }))
        .anchor_bottom()
        .height(Fill);

        let field = container(component::text_field(
            prev::fl!("assistant-ask"),
            &panel.input,
            component::Backdrop::ContainerLow,
            |input| {
                input
                    .id(panel.input_id.clone())
                    .on_input(move |text| message(PanelMessage::Input(text)))
                    .on_submit(message(PanelMessage::Send))
            },
        ))
        .width(Fill);
        let action = if panel.busy {
            component::tool(
                Icon::Stop,
                prev::fl!("assistant-stop"),
                Some(message(PanelMessage::Stop)),
            )
        } else {
            component::tool(
                Icon::ArrowUpward,
                prev::fl!("assistant-send"),
                (!panel.input.trim().is_empty() && has_models)
                    .then_some(message(PanelMessage::Send)),
            )
        };
        let input = row![field, action]
            .spacing(8)
            .align_y(Center)
            .padding(Padding {
                top: 8.0,
                right: 16.0,
                bottom: 16.0,
                left: 24.0,
            });
        Some(
            container(column![header, model_row, entries, input])
                .width(WIDTH)
                .height(Fill)
                .style(style::surface_container_low)
                .into(),
        )
    }
}

fn entry_view<'a>(
    entry: &'a Entry,
    theme: &'a iced::Theme,
    id: window::Id,
) -> Element<'a, Message> {
    match entry {
        Entry::User(said) => container(
            container(
                ui::styled(said.as_str(), Type::BodyMedium).wrapping(text::Wrapping::WordOrGlyph),
            )
            .padding([8, 12])
            .style(bubble),
        )
        .width(Fill)
        .align_x(iced::alignment::Horizontal::Right)
        .into(),
        Entry::Reply(reply) => markdown::view(reply.items(), markdown_settings(theme))
            .map(move |uri| Message::Assistant(id, PanelMessage::Link(uri))),
        Entry::Tool {
            title,
            state,
            picture,
            ..
        } => {
            let (glyph, note) = match state {
                ToolState::Waiting => (Icon::Circle, None),
                ToolState::Running => (Icon::Circle, Some(prev::fl!("assistant-running"))),
                ToolState::Done => (Icon::Check, None),
                ToolState::Failed(why) => (Icon::Error, Some(why.clone())),
            };
            let failed = matches!(state, ToolState::Failed(_));
            let mut line = column![
                row![
                    ui::icon(glyph, 16.0).style(if failed {
                        style::error_text
                    } else {
                        style::on_surface_variant
                    }),
                    ui::aligned(
                        ui::styled(title.as_str(), Type::LabelLarge)
                            .style(style::on_surface_variant)
                    ),
                ]
                .spacing(8)
                .align_y(Center),
            ]
            .spacing(6);
            if let Some(note) = note {
                line = line.push(ui::aligned(
                    ui::styled(note, Type::BodySmall)
                        .style(if failed {
                            style::error_text
                        } else {
                            style::on_surface_variant
                        })
                        .wrapping(text::Wrapping::WordOrGlyph),
                ));
            }
            if let Some(picture) = picture {
                line = line.push(
                    image(picture.clone())
                        .width(Length::Shrink)
                        .height(Length::Shrink)
                        .content_fit(iced::ContentFit::ScaleDown),
                );
            }
            container(line).max_width(PICTURE_WIDTH + 24.0).into()
        }
        Entry::Note(note) => {
            ui::aligned(ui::styled(note.as_str(), Type::BodySmall).style(style::on_surface_variant))
        }
        Entry::Problem(problem) => ui::aligned(
            ui::styled(problem.as_str(), Type::BodyMedium)
                .style(style::error_text)
                .wrapping(text::Wrapping::WordOrGlyph),
        ),
    }
}

/// The user's messages, in a rounded bubble.
fn bubble(theme: &iced::Theme) -> container::Style {
    let scheme = ui::Scheme::of(theme);
    container::Style {
        background: Some(scheme.surface_container_highest.into()),
        text_color: Some(scheme.on_surface),
        border: iced::border::rounded(ui::shape::LARGE),
        ..container::Style::default()
    }
}

fn markdown_settings(theme: &iced::Theme) -> markdown::Settings {
    let scheme = ui::Scheme::of(theme);
    let style = markdown::Style {
        font: ui::font::TEXT,
        inline_code_highlight: iced::advanced::text::Highlight {
            background: scheme.surface_container_highest.into(),
            border: iced::border::rounded(ui::shape::EXTRA_SMALL),
        },
        inline_code_color: scheme.on_surface,
        link_color: scheme.primary,
        ..markdown::Style::from(theme)
    };
    markdown::Settings::with_text_size(14.0, style)
}

fn set_state(panel: &mut Panel, call: &str, new: ToolState) {
    for entry in panel.entries.iter_mut().rev() {
        if let Entry::Tool { id, state, .. } = entry
            && id == call
        {
            *state = new;
            return;
        }
    }
}

/// A tool's PNG, scaled down to fit the panel.
fn picture(png: &[u8]) -> Option<image::Handle> {
    let decoded = ::image::load_from_memory_with_format(png, ::image::ImageFormat::Png).ok()?;
    let decoded = if decoded.width() as f32 > PICTURE_WIDTH {
        decoded.resize(
            PICTURE_WIDTH as u32,
            u32::MAX,
            ::image::imageops::FilterType::Triangle,
        )
    } else {
        decoded
    };
    let rgba = decoded.into_rgba8();
    Some(image::Handle::from_rgba(
        rgba.width(),
        rgba.height(),
        rgba.into_raw(),
    ))
}

/// A tool's MCP result, or its error, as the chat sends it on.
fn tool_result(id: String, outcome: Result<Value, Error>) -> ToolResult {
    match outcome {
        Ok(result) => {
            let content = result["content"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|part| match part["type"].as_str()? {
                    "text" => Some(Content::Text(part["text"].as_str()?.to_owned())),
                    "image" => base64::engine::general_purpose::STANDARD
                        .decode(part["data"].as_str()?)
                        .ok()
                        .map(Content::Png),
                    _ => None,
                })
                .collect();
            ToolResult {
                id,
                content,
                failed: result["isError"].as_bool().unwrap_or(false),
            }
        }
        Err(error) => ToolResult {
            id,
            content: vec![Content::Text(error.message)],
            failed: true,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tool_results_keep_text_pictures_and_errors() {
        let png = vec![1, 2, 3];
        let result = tool_result(
            "a".to_owned(),
            Ok(json!({
                "content": [
                    { "type": "image", "data": base64::engine::general_purpose::STANDARD.encode(&png), "mimeType": "image/png" },
                    { "type": "text", "text": "Page 1" },
                ],
            })),
        );
        assert!(!result.failed);
        assert!(matches!(&result.content[0], Content::Png(bytes) if *bytes == png));
        assert!(matches!(&result.content[1], Content::Text(text) if text == "Page 1"));
        let failed = tool_result(
            "b".to_owned(),
            Err(Error::new(prev::control::code::INVALID_PARAMS, "no page 9")),
        );
        assert!(failed.failed);
        assert!(matches!(&failed.content[0], Content::Text(text) if text == "no page 9"));
    }
}
