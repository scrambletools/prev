//! Outside control: the calls programs make over the control channel,
//! mostly AI agents through `prev --mcp`. prev asks the user before an
//! agent it has not seen gets to act, and before a tool runs whose kind
//! Settings asks about.

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use iced::window;
use iced::{Element, Task};
use prev::control::{Call, Error, code};
use prev::ui::button::Kind;
use prev::ui::{self, Icon, component};
use serde_json::{Value, json};

use super::tools::{self, Answer, Tool};
use super::{Message, Prev};

/// The notification a call sends while it waits for the user.
pub(super) const WAITING: &str = "prev/waiting";

/// What agents hear while outside control is off.
const OFF_MESSAGE: &str = "Outside control is off in prev's Settings.";

/// How long a new prompt's buttons wait before taking clicks, so a click
/// meant for the prompt before it does not answer it.
const ANSWER_DELAY: Duration = Duration::from_millis(700);

/// Agents waiting for the user to allow them, and the ones turned away.
#[derive(Default)]
pub(super) struct Agents {
    /// The agents to ask about, the first shown first.
    prompts: Vec<Prompt>,
    /// Agents the user did not allow, until prev quits.
    declined: BTreeSet<String>,
    /// When the first prompt came up.
    shown_at: Option<Instant>,
    /// Tool calls waiting for the user to allow them, the first for each
    /// window shown over it first.
    approvals: Vec<Approval>,
    /// Image exports agents asked for, by window, answered once done.
    pub(super) exports: Vec<(window::Id, Answer)>,
}

/// A tool call waiting for the user to allow it to run.
struct Approval {
    window: window::Id,
    /// The agent, by the name the user knows it by.
    agent: String,
    tool: &'static Tool,
    arguments: Value,
    answer: Answer,
    /// When it came up over its window, which takes answers a moment
    /// later.
    shown_at: Option<Instant>,
}

/// An agent waiting for the user's answer, with the calls it made
/// meanwhile.
struct Prompt {
    name: String,
    /// The name shown to the user: the agent's title when it gives one.
    shown: String,
    waiting: Vec<Call>,
}

/// The agent a call came from, as `prev --mcp` passes it on.
fn agent(params: &Value) -> (String, String) {
    let name = params["name"].as_str().unwrap_or_default().trim();
    let name = if name.is_empty() { "agent" } else { name };
    let shown = params["title"]
        .as_str()
        .map(str::trim)
        .filter(|title| !title.is_empty())
        .unwrap_or(name);
    (name.to_owned(), shown.to_owned())
}

impl Prev {
    pub(super) fn control(&mut self, call: Call) -> Task<Message> {
        let answer = match call.method.as_str() {
            "ping" => Ok(json!({
                "protocol": prev::control::PROTOCOL,
                "version": env!("CARGO_PKG_VERSION"),
                "build": env!("PREV_COMMIT"),
                "development": !prev_store::paths::PRODUCTION,
            })),
            // Listing does nothing, so agents see the tools while outside
            // control is off, and hear why at their first call.
            "tools/list" => Ok(json!({ "tools": super::tools::list(&self.settings.ask_before) })),
            _ if !self.settings.outside_control => Err(Error::new(code::OFF, OFF_MESSAGE)),
            // An agent connected: ask the user now, not at its first call.
            "agent/hello" => {
                let (name, shown) = agent(&call.params);
                call.reply(Ok(Value::Null));
                return self.ask_about(name, shown, None);
            }
            "tools/call" => {
                let (name, shown) = agent(&call.params["agent"]);
                if self.settings.allowed_agents.contains(&name) {
                    return self.run_tool(call, shown);
                } else if self.agents.declined.contains(&name) {
                    Err(declined(&shown))
                } else {
                    return self.ask_about(name, shown, Some(call));
                }
            }
            method => Err(Error::new(
                code::METHOD_NOT_FOUND,
                format!("prev has no method {method}"),
            )),
        };
        call.reply(answer);
        Task::none()
    }

    /// Asks the user whether agent `name` may control prev, unless they
    /// already answered; `call` waits for the answer.
    fn ask_about(&mut self, name: String, shown: String, call: Option<Call>) -> Task<Message> {
        if self.settings.allowed_agents.contains(&name) || self.agents.declined.contains(&name) {
            if let Some(call) = call {
                return self.control(call);
            }
            return Task::none();
        }
        if let Some(call) = &call {
            call.note(
                WAITING,
                json!({
                    "message": format!(
                        "Waiting for the user to allow {shown} to control prev; prev asks in \
                         its window."
                    ),
                }),
            );
        }
        if let Some(prompt) = self
            .agents
            .prompts
            .iter_mut()
            .find(|prompt| prompt.name == name)
        {
            prompt.waiting.extend(call);
            return Task::none();
        }
        self.agents.prompts.push(Prompt {
            name,
            shown,
            waiting: call.into_iter().collect(),
        });
        if self.agents.prompts.len() == 1 {
            self.prompt_shown()
        } else {
            Task::none()
        }
    }

    /// A prompt came up first: its buttons wait a moment, and the window
    /// asks for attention until the user comes to it, as prev may be
    /// behind other windows.
    fn prompt_shown(&mut self) -> Task<Message> {
        self.agents.shown_at = Some(Instant::now());
        match self.prompt_window() {
            Some(id) => notice_prompt(id),
            None => Task::none(),
        }
    }

    /// Whether the first prompt has been up long enough to take answers.
    fn prompt_ready(&self) -> bool {
        ready(self.agents.shown_at)
    }

    /// Holds a call of `tool` on window `id` until the user allows it.
    pub(super) fn ask_to_run(
        &mut self,
        id: window::Id,
        agent: String,
        tool: &'static Tool,
        arguments: Value,
        answer: Answer,
    ) -> Task<Message> {
        let first = !self.agents.approvals.iter().any(|asked| asked.window == id);
        answer.note(&format!(
            "Waiting for the user to allow {} in prev's window {}, \"{}\". Tell them prev \
             is asking.",
            tool.name,
            tools::number(id),
            self.base_title(id),
        ));
        self.agents.approvals.push(Approval {
            window: id,
            agent,
            tool,
            arguments,
            answer,
            shown_at: first.then(Instant::now),
        });
        if first {
            notice_prompt(id)
        } else {
            Task::none()
        }
    }

    /// The user answered the tool call shown over window `id`.
    pub(super) fn run_answered(&mut self, id: window::Id, allow: bool) -> Task<Message> {
        let Some(index) = self
            .agents
            .approvals
            .iter()
            .position(|asked| asked.window == id)
        else {
            return Task::none();
        };
        if !ready(self.agents.approvals[index].shown_at) {
            return Task::none();
        }
        let asked = self.agents.approvals.remove(index);
        let mut tasks = Vec::new();
        if allow {
            tasks.push(asked.tool.start(self, asked.arguments, asked.answer));
        } else {
            asked.answer.send(Err(Error::new(
                code::DECLINED,
                format!("The user did not allow {}.", asked.tool.name),
            )));
        }
        if let Some(next) = self
            .agents
            .approvals
            .iter_mut()
            .find(|asked| asked.window == id)
        {
            next.shown_at = Some(Instant::now());
            tasks.push(notice_prompt(id));
        }
        Task::batch(tasks)
    }

    /// Window `id` closed: the calls waiting on it hear so.
    pub(super) fn window_gone(&mut self, id: window::Id) {
        self.agents.exports.retain(|(window, answer)| {
            if *window == id {
                answer.send(Err(Error::new(code::INVALID_PARAMS, "The window closed.")));
            }
            *window != id
        });
        self.agents.approvals.retain(|asked| {
            if asked.window == id {
                asked.answer.send(Err(Error::new(
                    code::DECLINED,
                    format!(
                        "The window closed before the user allowed {}.",
                        asked.tool.name
                    ),
                )));
            }
            asked.window != id
        });
    }

    /// The user answered the first prompt.
    pub(super) fn agent_answered(&mut self, allow: bool) -> Task<Message> {
        if self.agents.prompts.is_empty() || !self.prompt_ready() {
            return Task::none();
        }
        let prompt = self.agents.prompts.remove(0);
        if allow {
            self.settings.allowed_agents.push(prompt.name);
            self.save_settings();
        } else {
            self.agents.declined.insert(prompt.name);
        }
        let mut tasks = Vec::new();
        for call in prompt.waiting {
            if allow {
                tasks.push(self.run_tool(call, prompt.shown.clone()));
            } else {
                call.reply(Err(declined(&prompt.shown)));
            }
        }
        if self.agents.prompts.is_empty() {
            self.agents.shown_at = None;
        } else {
            tasks.push(self.prompt_shown());
        }
        Task::batch(tasks)
    }

    /// Outside control was turned off: waiting agents hear so.
    pub(super) fn outside_control_off(&mut self) {
        for prompt in self.agents.prompts.drain(..) {
            for call in prompt.waiting {
                call.reply(Err(Error::new(code::OFF, OFF_MESSAGE)));
            }
        }
        for asked in self.agents.approvals.drain(..) {
            asked.answer.send(Err(Error::new(code::OFF, OFF_MESSAGE)));
        }
        self.agents.shown_at = None;
    }

    /// The window that shows the prompt: the focused one, else the first.
    fn prompt_window(&self) -> Option<window::Id> {
        self.focused
            .filter(|id| self.windows.contains_key(id))
            .or_else(|| self.windows.keys().next().copied())
    }

    /// Whether window `id` shows an agent's prompt to connect.
    fn shows_connect_prompt(&self, id: window::Id) -> bool {
        !self.agents.prompts.is_empty() && self.prompt_window() == Some(id)
    }

    /// Answers the prompt over window `id`, if it shows one, as Escape
    /// does with Don't Allow.
    pub(super) fn answer_prompt(&mut self, id: window::Id, allow: bool) -> Option<Task<Message>> {
        if self.shows_connect_prompt(id) {
            Some(self.agent_answered(allow))
        } else if self.agents.approvals.iter().any(|asked| asked.window == id) {
            Some(self.run_answered(id, allow))
        } else {
            None
        }
    }

    /// The prompt over window `id`, if it shows one: an agent asking to
    /// connect, else the first tool call waiting on the window.
    pub(super) fn agent_prompt(&self, id: window::Id) -> Option<Element<'_, Message>> {
        if !self.shows_connect_prompt(id) {
            return self.run_prompt(id);
        }
        let agent = self.agents.prompts[0].shown.as_str();
        let ready = self.prompt_ready();
        Some(component::dialog(
            iced::widget::space().width(iced::Fill).height(iced::Fill),
            Some(Icon::Lock),
            prev::fl!("agent-prompt-title", agent = agent),
            prev::fl!("agent-prompt-body", agent = agent),
            vec![
                ui::button(Kind::Text, prev::fl!("agent-prompt-deny"))
                    .on_press_maybe(ready.then_some(Message::AgentAnswered(false)))
                    .into(),
                ui::button(Kind::Filled, prev::fl!("agent-prompt-allow"))
                    .on_press_maybe(ready.then_some(Message::AgentAnswered(true)))
                    .into(),
            ],
        ))
    }
}

impl Prev {
    fn run_prompt(&self, id: window::Id) -> Option<Element<'_, Message>> {
        let asked = self
            .agents
            .approvals
            .iter()
            .find(|asked| asked.window == id)?;
        let agent = asked.agent.as_str();
        let title = match asked.tool.kind {
            tools::Kind::Read => prev::fl!("agent-ask-read", agent = agent),
            tools::Kind::View => prev::fl!("agent-ask-view", agent = agent),
            tools::Kind::Markup => prev::fl!("agent-ask-markup", agent = agent),
            tools::Kind::Edit => prev::fl!("agent-ask-edit", agent = agent),
            tools::Kind::Sign => prev::fl!("agent-ask-sign", agent = agent),
            tools::Kind::Redact => prev::fl!("agent-ask-redact", agent = agent),
            tools::Kind::Export => prev::fl!("agent-ask-export", agent = agent),
        };
        let mut body = prev::fl!("agent-ask-body", agent = agent, tool = asked.tool.title);
        if asked.tool.kind.is_final() {
            body = format!("{body} {}", prev::fl!("agent-ask-final"));
        }
        let ready = ready(asked.shown_at);
        Some(component::dialog(
            iced::widget::space().width(iced::Fill).height(iced::Fill),
            Some(Icon::Lock),
            title,
            body,
            vec![
                ui::button(Kind::Text, prev::fl!("agent-prompt-deny"))
                    .on_press_maybe(ready.then_some(Message::RunAnswered(id, false)))
                    .into(),
                ui::button(Kind::Filled, prev::fl!("agent-prompt-allow"))
                    .on_press_maybe(ready.then_some(Message::RunAnswered(id, true)))
                    .into(),
            ],
        ))
    }
}

/// Whether a prompt shown at `shown_at` takes answers yet.
fn ready(shown_at: Option<Instant>) -> bool {
    shown_at.is_some_and(|shown| shown.elapsed() >= ANSWER_DELAY)
}

/// A prompt came up over window `id`: its buttons take clicks after a
/// moment, and the window asks for attention until the user comes to it,
/// as prev may be behind other windows.
fn notice_prompt(id: window::Id) -> Task<Message> {
    Task::batch([
        Task::perform(
            prev::image::editor::spawn(|| std::thread::sleep(ANSWER_DELAY)),
            |_| Message::AgentPromptReady,
        ),
        window::request_user_attention(id, Some(window::UserAttention::Critical)),
    ])
}

fn declined(agent: &str) -> Error {
    Error::new(
        code::DECLINED,
        format!("The user did not allow {agent} to control prev."),
    )
}
