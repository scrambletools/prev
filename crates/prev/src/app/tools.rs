//! The tools prev offers agents. Each has a name, a kind, which the
//! approval switches go by, a description the model reads, an input
//! schema made from its input type, and a function that runs it against
//! a window. A tool answers through its [`Answer`], at once or once the
//! document thread or a render replies.

use std::sync::LazyLock;

use base64::Engine;
use iced::{Task, window};
use prev::control::{Call, Error, code};
use prev::filetype::FileKind;
use prev_store::settings;
use schemars::JsonSchema;
use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

use super::{Content, Message, Prev};

/// What a tool does, which decides whether prev asks first.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code, reason = "the tools of most kinds come in later steps")]
pub(crate) enum Kind {
    Read,
    View,
    Markup,
    Edit,
    Sign,
    Redact,
    Export,
}

impl Kind {
    pub(super) const ALL: [Kind; 7] = [
        Kind::Read,
        Kind::View,
        Kind::Markup,
        Kind::Edit,
        Kind::Sign,
        Kind::Redact,
        Kind::Export,
    ];

    /// Its switch in Settings: whether prev asks before a tool of this
    /// kind runs.
    pub(super) fn asks(self, ask: &settings::AskBefore) -> bool {
        match self {
            Kind::Read => ask.reading,
            Kind::View => ask.viewing,
            Kind::Markup => ask.marking_up,
            Kind::Edit => ask.editing,
            Kind::Sign => ask.signing,
            Kind::Redact => ask.redacting,
            Kind::Export => ask.exporting,
        }
    }

    pub(super) fn set_asks(self, ask: &mut settings::AskBefore, asks: bool) {
        *match self {
            Kind::Read => &mut ask.reading,
            Kind::View => &mut ask.viewing,
            Kind::Markup => &mut ask.marking_up,
            Kind::Edit => &mut ask.editing,
            Kind::Sign => &mut ask.signing,
            Kind::Redact => &mut ask.redacting,
            Kind::Export => &mut ask.exporting,
        } = asks;
    }

    /// Whether Undo cannot take a tool of this kind back.
    pub(super) fn is_final(self) -> bool {
        matches!(self, Kind::Sign | Kind::Redact | Kind::Export)
    }
}

/// What a tool gives back.
#[derive(Debug, Clone)]
#[allow(dead_code, reason = "pictures come with the render tools")]
pub(crate) enum Output {
    /// An object, as structured content and the same as text for agents
    /// that read only text.
    Json(Value),
    Text(String),
    Png(Vec<u8>),
}

impl Output {
    /// The output as an MCP tool result.
    fn into_result(self) -> Value {
        match self {
            Output::Json(value) => json!({
                "content": [{ "type": "text", "text": value.to_string() }],
                "structuredContent": value,
            }),
            Output::Text(text) => json!({ "content": [{ "type": "text", "text": text }] }),
            Output::Png(png) => json!({
                "content": [{
                    "type": "image",
                    "data": base64::engine::general_purpose::STANDARD.encode(png),
                    "mimeType": "image/png",
                }],
            }),
        }
    }
}

/// Where a tool's answer goes. A tool that waits on something keeps it
/// and answers later, with [`Answer::later`]; dropped unanswered, it tells
/// the agent prev did not answer.
#[derive(Debug, Clone)]
pub(crate) struct Answer(Call);

impl Answer {
    pub(super) fn send(&self, result: Result<Output, Error>) {
        self.0.reply(result.map(Output::into_result));
    }

    /// Answers with what `task` gives, once it finishes.
    #[allow(dead_code, reason = "for the tools that wait on a document")]
    pub(super) fn later(self, task: Task<Result<Output, Error>>) -> Task<Message> {
        task.map(move |result| Message::ToolAnswer(self.clone(), result))
    }
}

type Run = Box<dyn Fn(&mut Prev, Value, Answer) -> Task<Message> + Send + Sync>;

pub(super) struct Tool {
    pub(super) name: &'static str,
    pub(super) title: &'static str,
    pub(super) kind: Kind,
    description: &'static str,
    schema: fn() -> Value,
    run: Run,
}

impl Tool {
    /// Runs the tool on `arguments`; it answers through `answer`.
    pub(super) fn start(&self, app: &mut Prev, arguments: Value, answer: Answer) -> Task<Message> {
        (self.run)(app, arguments, answer)
    }
}

/// A tool whose input is `I`, read from the call's arguments.
fn tool<I: DeserializeOwned + JsonSchema + 'static>(
    name: &'static str,
    title: &'static str,
    kind: Kind,
    description: &'static str,
    run: fn(&mut Prev, I, &Answer) -> Task<Message>,
) -> Tool {
    Tool {
        name,
        title,
        kind,
        description,
        schema: || {
            let mut schema = serde_json::to_value(schemars::schema_for!(I)).unwrap_or_default();
            // The tool's own name and description say what these would.
            if let Some(schema) = schema.as_object_mut() {
                schema.remove("$schema");
                schema.remove("title");
                schema.remove("description");
            }
            schema
        },
        run: Box::new(
            move |app, arguments, answer| match serde_json::from_value::<I>(arguments) {
                Ok(input) => run(app, input, &answer),
                Err(error) => {
                    answer.send(Err(Error::new(
                        code::INVALID_PARAMS,
                        format!("{name}: {error}"),
                    )));
                    Task::none()
                }
            },
        ),
    }
}

/// Every tool, in the order agents see them.
static TOOLS: LazyLock<Vec<Tool>> = LazyLock::new(|| {
    vec![
        tool(
            "list_windows",
            "List windows",
            Kind::Read,
            "Lists prev's open windows: each one's number, title, kind (pdf, image, svg, \
             markdown, or start for a window with no file), file path, and whether it is the \
             one in front. Other tools take the number as `window`.",
            |app, _: Nothing, answer| {
                answer.send(Ok(Output::Json(json!({ "windows": app.window_list() }))));
                Task::none()
            },
        ),
        tool(
            "focus_window",
            "Focus a window",
            Kind::View,
            "Brings a prev window to the front and gives it the keyboard focus. Some \
             desktops, such as Wayland ones, only flag the window for the user instead.",
            |app, on: On, answer| match app.tool_window(on.window) {
                Ok(id) => {
                    answer.send(Ok(Output::Text(format!(
                        "Asked to bring window {} to the front. Some desktops flag the \
                         window for the user instead.",
                        number(id)
                    ))));
                    window::gain_focus(id)
                }
                Err(error) => {
                    answer.send(Err(error));
                    Task::none()
                }
            },
        ),
    ]
});

/// The input of a tool that takes none.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct Nothing {}

/// The input of a tool that acts on a window and takes nothing else.
#[derive(Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
struct On {
    /// The window's number from list_windows; the one in front if left out.
    window: Option<u64>,
}

/// The tools as MCP lists them.
pub(super) fn list() -> Value {
    TOOLS
        .iter()
        .map(|tool| {
            json!({
                "name": tool.name,
                "title": tool.title,
                "description": tool.description,
                "inputSchema": (tool.schema)(),
                "annotations": annotations(tool.kind),
            })
        })
        .collect()
}

/// MCP's hints for clients: what only reads, and what Undo cannot take
/// back.
fn annotations(kind: Kind) -> Value {
    match kind {
        Kind::Read => json!({ "readOnlyHint": true }),
        Kind::View | Kind::Markup | Kind::Edit => {
            json!({ "readOnlyHint": false, "destructiveHint": false })
        }
        Kind::Sign | Kind::Redact | Kind::Export => {
            json!({ "readOnlyHint": false, "destructiveHint": true })
        }
    }
}

/// The tool named `name`.
pub(super) fn find(name: &str) -> Option<&'static Tool> {
    TOOLS.iter().find(|tool| tool.name == name)
}

/// The number agents know window `id` by.
fn number(id: window::Id) -> u64 {
    id.to_string().parse().unwrap_or_default()
}

impl Prev {
    /// Runs the tool a `tools/call` names, which answers `call`, once
    /// the user allows it when its kind asks first. `agent` is the name
    /// the user knows the agent by.
    pub(super) fn run_tool(&mut self, call: Call, agent: String) -> Task<Message> {
        let name = call.params["name"].as_str().unwrap_or_default().to_owned();
        let arguments = match &call.params["arguments"] {
            Value::Null => json!({}),
            arguments => arguments.clone(),
        };
        match find(&name) {
            Some(tool) if tool.kind.asks(&self.settings.ask_before) => {
                // A window that cannot be found is the tool's to report.
                match self.tool_window(arguments["window"].as_u64()) {
                    Ok(id) => self.ask_to_run(id, agent, tool, arguments, Answer(call)),
                    Err(_) => tool.start(self, arguments, Answer(call)),
                }
            }
            Some(tool) => tool.start(self, arguments, Answer(call)),
            None => {
                call.reply(Err(Error::new(
                    code::INVALID_PARAMS,
                    format!("prev has no tool {name}"),
                )));
                Task::none()
            }
        }
    }

    /// The window a tool acts on: the one numbered `window`, else the one
    /// in front.
    pub(super) fn tool_window(&self, window: Option<u64>) -> Result<window::Id, Error> {
        let found = match window {
            Some(wanted) => self
                .windows
                .keys()
                .copied()
                .find(|id| number(*id) == wanted),
            None => self
                .focused
                .filter(|id| self.windows.contains_key(id))
                .or_else(|| self.windows.keys().next().copied()),
        };
        found.ok_or_else(|| {
            Error::new(
                code::INVALID_PARAMS,
                match window {
                    Some(wanted) => format!("prev has no window {wanted}; see list_windows"),
                    None => "prev has no window open".to_owned(),
                },
            )
        })
    }

    fn window_list(&self) -> Vec<Value> {
        self.windows
            .iter()
            .map(|(id, window)| {
                let (kind, file) = match &window.content {
                    Content::Start => ("start", None),
                    Content::Document(document) => {
                        let kind = match &document.kind {
                            Ok(Some(FileKind::Pdf)) => "pdf",
                            Ok(Some(FileKind::Image(_))) => "image",
                            Ok(Some(FileKind::Svg)) => "svg",
                            Ok(Some(FileKind::Markdown)) => "markdown",
                            Ok(None) | Err(_) => "unreadable",
                        };
                        let path = document
                            .images
                            .as_ref()
                            .map_or(document.path.as_path(), |images| images.current_path());
                        (kind, Some(path.display().to_string()))
                    }
                };
                json!({
                    "window": number(*id),
                    "title": self.base_title(*id),
                    "kind": kind,
                    "file": file,
                    "focused": self.focused == Some(*id),
                })
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_are_unique_and_snake_case() {
        let mut names: Vec<_> = TOOLS.iter().map(|tool| tool.name).collect();
        assert!(names.iter().all(|name| {
            !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '_')
        }));
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), TOOLS.len());
    }

    #[test]
    fn every_schema_is_an_object() {
        for tool in TOOLS.iter() {
            let schema = (tool.schema)();
            assert_eq!(schema["type"], "object", "{}: {schema}", tool.name);
            assert!(schema.get("$schema").is_none(), "{}", tool.name);
        }
    }

    #[test]
    fn window_input_is_described() {
        let schema = (find("focus_window").unwrap().schema)();
        let window = &schema["properties"]["window"];
        assert!(
            window["description"]
                .as_str()
                .is_some_and(|text| text.contains("list_windows")),
            "{schema}"
        );
    }

    #[test]
    fn list_has_what_mcp_needs() {
        let list = list();
        for tool in list.as_array().unwrap() {
            for field in ["name", "title", "description", "inputSchema", "annotations"] {
                assert!(tool.get(field).is_some(), "{field} in {tool}");
            }
        }
        assert_eq!(list[0]["annotations"]["readOnlyHint"], true);
    }

    #[test]
    fn outputs_become_mcp_results() {
        let json = Output::Json(json!({ "a": 1 })).into_result();
        assert_eq!(json["structuredContent"]["a"], 1);
        assert_eq!(json["content"][0]["text"], r#"{"a":1}"#);
        let png = Output::Png(vec![1, 2, 3]).into_result();
        assert_eq!(png["content"][0]["type"], "image");
        assert_eq!(png["content"][0]["data"], "AQID");
        assert_eq!(png["content"][0]["mimeType"], "image/png");
    }
}
