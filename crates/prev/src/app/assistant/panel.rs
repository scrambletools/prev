//! The assistant panel: a window's chat with the model in use, on the
//! right. The model's tool calls run one at a time through the same
//! registry and approvals as an outside agent's, on the panel's window
//! unless they name another.

use std::collections::VecDeque;

use base64::Engine;
use iced::widget::{Id, column, container, image, markdown, operation, row, space, text};
use iced::{Center, Element, Fill, Length, Padding, Task, window};
use prev::control::{Call, Error};
use prev::ui::button::Kind;
use prev::ui::{self, Icon, Type, component, style};
use prev_assist::{Chat, Content, Event, ToolCall, ToolResult};
use serde_json::{Value, json};

use super::super::{Message, Prev, tools};
use super::{choice_of, key_of, shown_name};
use crate::External;
use prev_store::settings::AssistantModel;

/// The panel's width.
const WIDTH: f32 = 380.0;

/// The widest a tool's picture shows in the panel.
const PICTURE_WIDTH: f32 = 300.0;

/// What the model is told about prev and the panel.
const INSTRUCTIONS: &str = "You are the assistant in prev, a PDF, image and Markdown viewer, \
chatting with the user in a panel beside their file. Use the tools to look at the file and act on \
it; they act on the user's window unless you give another window's number, so you need \
list_windows only when the user speaks of other windows. You get only what you \
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
    /// Talk to the model with this id from now on.
    Model(String),
    /// Open or close the model menu.
    Menu(bool),
    /// The key for a chat about to start: the chat's number, the model's
    /// id, and the key, if it has one.
    Keyed(u64, String, Result<Option<String>, String>),
    /// The key for the model the chat changes to.
    Switched(u64, String, Result<Option<String>, String>),
    Link(markdown::Uri),
    /// Unfold or fold the reasoning at this entry.
    ToggleReasoning(usize),
    OpenSettings,
}

enum Entry {
    User(String),
    /// What a reasoning model thought before it replied: shown faintly as
    /// it streams, then folded away once the reply or a tool call comes,
    /// unless the user opens it.
    Reasoning {
        text: String,
        done: bool,
        open: bool,
    },
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
    menu_open: bool,
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
            menu_open: false,
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

    /// What the model is told of the panel's window: its number and the
    /// file it shows, so it does not have to ask.
    fn window_context(&self, id: window::Id) -> String {
        use prev::filetype::FileKind;
        let number = tools::number(id);
        let file = self
            .windows
            .get(&id)
            .and_then(|window| match &window.content {
                super::super::Content::Document(document) => {
                    let path = document
                        .images
                        .as_ref()
                        .map_or(document.path.as_path(), |images| images.current_path());
                    let name = path.file_name()?.to_string_lossy().into_owned();
                    let kind = match &document.kind {
                        Ok(Some(FileKind::Pdf)) => "a PDF",
                        Ok(Some(FileKind::Image(_) | FileKind::Svg)) => "an image",
                        Ok(Some(FileKind::Markdown)) => "a Markdown document",
                        _ => "a file",
                    };
                    Some(format!("{name}, {kind}"))
                }
                super::super::Content::Start => None,
            });
        match file {
            Some(file) => format!(
                "The user's window is number {number} and shows {file}; the user means it unless \
                 they say otherwise."
            ),
            None => format!("The user's window is number {number}, with no file open yet."),
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
                    prev::image::editor::spawn(move || key_of(&model).map(|key| (model.id, key))),
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
                        format!("{INSTRUCTIONS} {}", self.window_context(id)),
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
            PanelMessage::Menu(open) => panel.menu_open = open,
            PanelMessage::Model(chosen) => {
                panel.menu_open = false;
                let Some(model) = self
                    .settings
                    .assistant_models
                    .iter()
                    .find(|model| model.id == chosen)
                    .cloned()
                else {
                    return Task::none();
                };
                if self.panel_model(panel).map(|model| &model.id) == Some(&chosen) {
                    panel.model = Some(chosen);
                    return Task::none();
                }
                panel.model = Some(chosen.clone());
                // The next panel opened starts with it too.
                self.settings.assistant_model = Some(chosen);
                self.save_settings();
                if panel.chat.is_none() {
                    return Task::none();
                }
                panel.entries.push(Entry::Note(prev::fl!(
                    "assistant-switched",
                    model = super::shown_name(&model)
                )));
                let number = panel.chat_number;
                return Task::perform(
                    prev::image::editor::spawn(move || key_of(&model).map(|key| (model.id, key))),
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
            PanelMessage::ToggleReasoning(index) => {
                if let Some(Entry::Reasoning { open, .. }) = panel.entries.get_mut(index) {
                    *open = !*open;
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
        if let Event::Reasoning(more) = &event {
            match panel.entries.last_mut() {
                Some(Entry::Reasoning {
                    text, done: false, ..
                }) => text.push_str(more),
                _ => panel.entries.push(Entry::Reasoning {
                    text: more.trim_start().to_owned(),
                    done: false,
                    open: false,
                }),
            }
            return Task::none();
        }
        // Anything else ends the thinking.
        if let Some(Entry::Reasoning { done, .. }) = panel.entries.last_mut() {
            *done = true;
        }
        match event {
            Event::Reasoning(_) => {}
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
        let has_models = !self.settings.assistant_models.is_empty();
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
            self.model_menu(id, panel)
        };
        // Under the field, as the model goes with what is typed; its menu
        // opens upward.
        let model_row = container(model_row).padding(Padding {
            top: 0.0,
            right: 16.0,
            bottom: 16.0,
            left: 24.0,
        });

        let theme = &self.theme;
        let mut entries = column![].spacing(12);
        for (index, entry) in panel.entries.iter().enumerate() {
            entries = entries.push(entry_view(entry, index, theme, id));
        }
        let thinking = matches!(
            panel.entries.last(),
            Some(Entry::Reasoning { done: false, .. })
        );
        if panel.busy && !panel.running && !thinking {
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
                bottom: 8.0,
                left: 24.0,
            });
        Some(
            container(column![header, entries, input, model_row])
                .width(WIDTH)
                .height(Fill)
                .style(style::chrome)
                .into(),
        )
    }
}

impl Prev {
    /// The menu of models, grouped by who serves them, under a button
    /// showing the one in use.
    fn model_menu<'a>(&'a self, id: window::Id, panel: &'a Panel) -> Element<'a, Message> {
        use prev::ui::popover::{self, popover};
        let message = move |message: PanelMessage| Message::Assistant(id, message);
        let models = &self.settings.assistant_models;
        let chosen = self.panel_model(panel);
        let group = |model: &AssistantModel| (model.provider.clone(), model.address.clone());
        // Names two models share, which then show their ids too.
        let clashes = |model: &AssistantModel| {
            models
                .iter()
                .filter(|other| shown_name(other) == shown_name(model))
                .count()
                > 1
        };
        let anchor_label = chosen.map_or_else(String::new, shown_name);
        let anchor_detail = chosen.map_or_else(String::new, group_label);
        let anchor = ui::button::custom(
            Kind::Outlined,
            row![
                column![
                    ui::styled(anchor_label, Type::LabelLarge).style(on_surface),
                    ui::styled(anchor_detail, Type::BodySmall).style(style::on_surface_variant),
                ]
                .spacing(2)
                .width(Fill),
                ui::icon(Icon::ArrowDropDown, 20.0),
            ]
            .spacing(8)
            .align_y(Center),
        )
        .shape(ui::button::Shape::Square)
        .width(Fill)
        .height(52.0)
        .on_press(message(PanelMessage::Menu(!panel.menu_open)));
        let mut items = column![].width(WIDTH - 48.0);
        let mut groups: Vec<(String, Option<String>)> = Vec::new();
        for model in models {
            if !groups.contains(&group(model)) {
                groups.push(group(model));
            }
        }
        for (provider, address) in groups {
            let members: Vec<&AssistantModel> = models
                .iter()
                .filter(|model| model.provider == provider && model.address == address)
                .collect();
            items = items.push(menu_heading(group_label(members[0])));
            for model in members {
                let mut label = shown_name(model);
                if clashes(model) {
                    label = format!("{label} ({})", model.model);
                }
                items = items.push(component::list_row(
                    None,
                    label,
                    0.0,
                    chosen.is_some_and(|chosen| chosen.id == model.id),
                    Some(message(PanelMessage::Model(model.id.clone()))),
                ));
            }
        }
        items = items.push(iced::widget::rule::horizontal(1));
        items = items.push(component::list_row(
            Some(Icon::Settings),
            prev::fl!("assistant-add-another"),
            0.0,
            false,
            Some(message(PanelMessage::OpenSettings)),
        ));
        popover(
            anchor,
            // No scroll area: one in a popover loses track of the pointer
            // near its end, and a hovered row stays lit after it leaves.
            panel.menu_open.then(|| popover::surface(items)),
            message(PanelMessage::Menu(false)),
        )
        .close_on_choice()
        .into()
    }
}

fn on_surface(theme: &iced::Theme) -> text::Style {
    text::Style {
        color: Some(ui::Scheme::of(theme).on_surface),
    }
}

/// A group's heading in the model menu: the provider, and for a server on
/// this computer, that it is on this computer.
fn group_label(model: &AssistantModel) -> String {
    let Some(provider) = super::provider_of(&model.provider) else {
        return model.provider.clone();
    };
    let label = super::provider_label(provider);
    if provider.needs_key() {
        return label;
    }
    match model.address.as_deref().map(host_of) {
        Some(host) if !is_local(host) => {
            prev::fl!(
                "assistant-group-remote",
                provider = label,
                host = host.to_owned()
            )
        }
        _ => prev::fl!("assistant-group-local", provider = label),
    }
}

/// The host in an address such as `http://192.168.4.61:11434/v1`.
fn host_of(address: &str) -> &str {
    let rest = address.split_once("://").map_or(address, |(_, rest)| rest);
    let authority = rest.split('/').next().unwrap_or(rest);
    match authority.strip_prefix('[') {
        // An IPv6 address, in brackets.
        Some(inner) => inner.split(']').next().unwrap_or(inner),
        None => authority.split(':').next().unwrap_or(authority),
    }
}

/// Whether `host` is this computer.
fn is_local(host: &str) -> bool {
    matches!(host, "localhost" | "127.0.0.1" | "::1" | "0.0.0.0") || host.ends_with(".localhost")
}

fn menu_heading<'a>(label: String) -> Element<'a, Message> {
    container(ui::styled(label, Type::LabelMedium).style(style::on_surface_variant))
        .padding(Padding {
            top: 8.0,
            right: 16.0,
            bottom: 4.0,
            left: 16.0,
        })
        .into()
}

fn entry_view<'a>(
    entry: &'a Entry,
    index: usize,
    theme: &'a iced::Theme,
    id: window::Id,
) -> Element<'a, Message> {
    match entry {
        Entry::Reasoning {
            text: thought,
            done,
            open,
        } => {
            let label = if *done {
                prev::fl!("assistant-thoughts")
            } else {
                prev::fl!("assistant-thinking")
            };
            let shown = *open || !*done;
            let header = ui::button::custom(
                Kind::Row,
                row![
                    ui::icon(
                        if shown {
                            Icon::ExpandLess
                        } else {
                            Icon::ExpandMore
                        },
                        16.0
                    )
                    .style(style::on_surface_variant),
                    ui::styled(label, Type::LabelMedium).style(style::on_surface_variant),
                ]
                .spacing(6)
                .align_y(Center),
            )
            .height(28.0)
            .on_press(Message::Assistant(id, PanelMessage::ToggleReasoning(index)));
            let mut lines = column![header].spacing(4);
            if shown {
                // While it streams, only the latest of it, so the panel
                // does not fill with thinking.
                let text = if *done {
                    thought.as_str()
                } else {
                    tail(thought, 600)
                };
                lines = lines.push(
                    container(
                        ui::styled(text.trim(), Type::BodySmall)
                            .style(style::on_surface_variant)
                            .wrapping(text::Wrapping::WordOrGlyph),
                    )
                    .padding(Padding {
                        top: 0.0,
                        right: 0.0,
                        bottom: 0.0,
                        left: 22.0,
                    }),
                );
            }
            lines.into()
        }
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

/// The last `chars` characters of `text`, from a word's start.
fn tail(text: &str, chars: usize) -> &str {
    let count = text.chars().count();
    if count <= chars {
        return text;
    }
    let start = text
        .char_indices()
        .nth(count - chars)
        .map_or(0, |(index, _)| index);
    let rest = &text[start..];
    rest.find(char::is_whitespace)
        .map_or(rest, |space| &rest[space..])
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
    fn servers_elsewhere_show_their_host() {
        assert_eq!(host_of("http://192.168.4.61:11434"), "192.168.4.61");
        assert_eq!(host_of("http://localhost:1234/v1"), "localhost");
        assert_eq!(host_of("http://[::1]:8080/v1"), "::1");
        assert_eq!(host_of("studio.lan:1234"), "studio.lan");
        assert!(is_local("localhost") && is_local("::1") && is_local("127.0.0.1"));
        assert!(!is_local("192.168.4.61"));
    }

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
