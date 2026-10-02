//! Outside control: the calls programs make over the control channel,
//! mostly AI agents through `prev --mcp`. prev asks the user before an
//! agent it has not seen gets to act.

use std::collections::BTreeSet;
use std::time::{Duration, Instant};

use iced::window;
use iced::{Element, Task};
use prev::control::{Call, Error, code};
use prev::ui::button::Kind;
use prev::ui::{self, Icon, component};
use serde_json::{Value, json};

use super::{Message, Prev};

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
            "tools/list" => Ok(json!({ "tools": super::tools::list() })),
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
                    return self.run_tool(call);
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
        let ready = Task::perform(
            prev::image::editor::spawn(|| std::thread::sleep(ANSWER_DELAY)),
            |_| Message::AgentPromptReady,
        );
        match self.prompt_window() {
            Some(id) => Task::batch([
                ready,
                window::request_user_attention(id, Some(window::UserAttention::Critical)),
            ]),
            None => ready,
        }
    }

    /// Whether the first prompt has been up long enough to take answers.
    fn prompt_ready(&self) -> bool {
        self.agents
            .shown_at
            .is_some_and(|shown| shown.elapsed() >= ANSWER_DELAY)
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
                tasks.push(self.run_tool(call));
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
        self.agents.shown_at = None;
    }

    /// The window that shows the prompt: the focused one, else the first.
    fn prompt_window(&self) -> Option<window::Id> {
        self.focused
            .filter(|id| self.windows.contains_key(id))
            .or_else(|| self.windows.keys().next().copied())
    }

    /// Whether window `id` shows a prompt, which Escape answers.
    pub(super) fn shows_agent_prompt(&self, id: window::Id) -> bool {
        !self.agents.prompts.is_empty() && self.prompt_window() == Some(id)
    }

    /// The prompt over window `id`, if it shows one.
    pub(super) fn agent_prompt(&self, id: window::Id) -> Option<Element<'_, Message>> {
        if !self.shows_agent_prompt(id) {
            return None;
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

fn declined(agent: &str) -> Error {
    Error::new(
        code::DECLINED,
        format!("The user did not allow {agent} to control prev."),
    )
}
