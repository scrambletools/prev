//! Each provider's chat, played back: a local server answers as the
//! provider does, in its own streaming format, so these run anywhere,
//! without keys. Each chat asks a question, gets a tool call, sends the
//! tool's result and gets the answer; the server keeps what prev-assist
//! sent, to check the tools went out and the result came back.
//!
//! The replies follow each provider's documented streaming format. When a
//! provider changes its format, the live tests (`ollama.rs`, and a real
//! key in Settings) catch it first; update the replies here to match.

use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use prev_assist::{
    Chat, Content, Event, ModelChoice, Problem, Provider, ToolCall, ToolResult, ToolSpec,
};
use serde_json::{Value, json};

/// What the server answers a request with.
enum Reply {
    /// A streamed reply: server-sent events.
    Stream(String),
    /// A plain JSON reply.
    Json(Value),
    /// An error status, with its body.
    Failure(u16, &'static str),
}

/// A request the server took: its path, with the query, and its body.
#[derive(Debug, Clone)]
struct Request {
    path: String,
    body: Value,
}

/// Serves `replies` in turn, one per request, on a port of its own.
fn serve(replies: Vec<Reply>) -> (String, Arc<Mutex<Vec<Request>>>) {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let address = format!("http://{}", listener.local_addr().unwrap());
    let taken = Arc::new(Mutex::new(Vec::new()));
    let log = taken.clone();
    std::thread::spawn(move || {
        let mut replies = replies.into_iter();
        for stream in listener.incoming() {
            let Ok(mut stream) = stream else { return };
            let mut reader = BufReader::new(stream.try_clone().unwrap());
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            let path = line
                .split_whitespace()
                .nth(1)
                .unwrap_or_default()
                .to_owned();
            let mut length = 0;
            loop {
                let mut header = String::new();
                reader.read_line(&mut header).unwrap();
                if header.trim().is_empty() {
                    break;
                }
                if let Some((name, value)) = header.split_once(':')
                    && name.eq_ignore_ascii_case("content-length")
                {
                    length = value.trim().parse().unwrap_or(0);
                }
            }
            let mut body = vec![0; length];
            reader.read_exact(&mut body).unwrap();
            log.lock().unwrap().push(Request {
                path,
                body: serde_json::from_slice(&body).unwrap_or(Value::Null),
            });
            let (status, kind, body) = match replies.next() {
                Some(Reply::Stream(events)) => (200, "text/event-stream", events),
                Some(Reply::Json(value)) => (200, "application/json", value.to_string()),
                Some(Reply::Failure(status, body)) => (status, "application/json", body.to_owned()),
                None => (500, "application/json", "{}".to_owned()),
            };
            let reply = format!(
                "HTTP/1.1 {status} OK\r\nContent-Type: {kind}\r\nContent-Length: {}\r\n\
                 Connection: close\r\n\r\n{body}",
                body.len()
            );
            let _ = stream.write_all(reply.as_bytes());
        }
    });
    (address, taken)
}

/// Server-sent events from `(event name, data)` pairs; an empty name
/// leaves the `event:` line out, as OpenAI and Gemini do.
fn events(list: &[(&str, Value)]) -> String {
    let mut out = String::new();
    for (name, data) in list {
        if !name.is_empty() {
            out.push_str(&format!("event: {name}\n"));
        }
        out.push_str(&format!("data: {data}\n\n"));
    }
    out
}

fn tools() -> Vec<ToolSpec> {
    vec![ToolSpec {
        name: "page_count".to_owned(),
        description: "Gives the number of pages of the open document.".to_owned(),
        schema: json!({
            "type": "object",
            "properties": { "window": { "type": "integer" } },
        }),
    }]
}

/// What one chat heard and sent.
struct Heard {
    calls: Vec<ToolCall>,
    reasoning: String,
    reply: String,
    requests: Vec<Request>,
}

/// Asks how many pages there are, answers the tool call with 12, and
/// collects the answer.
fn chat(provider: Provider, model: &str, replies: Vec<Reply>) -> Heard {
    let (address, requests) = serve(replies);
    let address = match provider {
        Provider::OpenAi | Provider::OpenAiCompatible => format!("{address}/v1"),
        _ => address,
    };
    let (sender, events) = mpsc::channel();
    let chat = Chat::start(
        ModelChoice {
            provider,
            model: model.to_owned(),
            address: Some(address),
            context: None,
            vision: None,
        },
        Some("test-key".to_owned()),
        "You help with documents.".to_owned(),
        tools(),
        move |event| {
            let _ = sender.send(event);
        },
    )
    .unwrap();
    chat.say("How many pages?");
    let mut heard = Heard {
        calls: Vec::new(),
        reasoning: String::new(),
        reply: String::new(),
        requests: Vec::new(),
    };
    loop {
        match events.recv_timeout(Duration::from_secs(10)).unwrap() {
            Event::Text(text) => heard.reply.push_str(&text),
            Event::Reasoning(text) => heard.reasoning.push_str(&text),
            Event::ToolCalls(calls) => {
                chat.results(
                    calls
                        .iter()
                        .map(|call| ToolResult {
                            id: call.id.clone(),
                            content: vec![Content::Text("12".to_owned())],
                            failed: false,
                        })
                        .collect(),
                );
                heard.calls.extend(calls);
            }
            Event::Done => break,
            Event::Failed(problem) => panic!("{problem:?}"),
            Event::Stopped => panic!("stopped"),
        }
    }
    heard.requests = requests.lock().unwrap().clone();
    heard
}

/// What every provider's chat must show: the tool went out, the call came
/// back with its arguments, the result went with the next request, and the
/// answer streamed.
fn check(heard: &Heard, call_id: &str) {
    assert_eq!(heard.requests.len(), 2, "{:?}", heard.requests);
    let first = heard.requests[0].body.to_string();
    assert!(
        first.contains("page_count"),
        "the tool did not go out: {first}"
    );
    assert!(first.contains("How many pages?"), "{first}");
    assert_eq!(heard.calls.len(), 1, "{:?}", heard.calls);
    assert_eq!(heard.calls[0].name, "page_count");
    assert_eq!(heard.calls[0].arguments, json!({ "window": 1 }));
    let second = heard.requests[1].body.to_string();
    assert!(
        second.contains("12"),
        "the result did not go back: {second}"
    );
    assert!(
        second.contains(call_id),
        "the result lost its call: {second}"
    );
    assert_eq!(heard.reply, "It has 12 pages.");
}

fn anthropic_turns() -> Vec<Reply> {
    let start = |id: &str| {
        (
            "message_start",
            json!({ "type": "message_start", "message": {
                "id": id, "type": "message", "role": "assistant", "model": "claude-test",
                "content": [], "stop_reason": null, "stop_sequence": null,
                "usage": { "input_tokens": 10, "output_tokens": 1 } } }),
        )
    };
    let stop = |reason: &str| {
        [
            (
                "message_delta",
                json!({ "type": "message_delta",
                    "delta": { "stop_reason": reason, "stop_sequence": null },
                    "usage": { "output_tokens": 20 } }),
            ),
            ("message_stop", json!({ "type": "message_stop" })),
        ]
    };
    let mut asks = vec![
        start("msg_1"),
        (
            "content_block_start",
            json!({ "type": "content_block_start", "index": 0,
                "content_block": { "type": "thinking", "thinking": "", "signature": "" } }),
        ),
        (
            "content_block_delta",
            json!({ "type": "content_block_delta", "index": 0,
                "delta": { "type": "thinking_delta", "thinking": "Counting the pages." } }),
        ),
        (
            "content_block_delta",
            json!({ "type": "content_block_delta", "index": 0,
                "delta": { "type": "signature_delta", "signature": "c2lnbmVk" } }),
        ),
        (
            "content_block_stop",
            json!({ "type": "content_block_stop", "index": 0 }),
        ),
        (
            "content_block_start",
            json!({ "type": "content_block_start", "index": 1, "content_block": {
                "type": "tool_use", "id": "toolu_01", "name": "page_count", "input": {} } }),
        ),
        (
            "content_block_delta",
            json!({ "type": "content_block_delta", "index": 1,
                "delta": { "type": "input_json_delta", "partial_json": "{\"window\": 1}" } }),
        ),
        (
            "content_block_stop",
            json!({ "type": "content_block_stop", "index": 1 }),
        ),
    ];
    asks.extend(stop("tool_use"));
    let mut answers = vec![
        start("msg_2"),
        (
            "content_block_start",
            json!({ "type": "content_block_start", "index": 0,
                "content_block": { "type": "text", "text": "" } }),
        ),
        (
            "content_block_delta",
            json!({ "type": "content_block_delta", "index": 0,
                "delta": { "type": "text_delta", "text": "It has " } }),
        ),
        (
            "content_block_delta",
            json!({ "type": "content_block_delta", "index": 0,
                "delta": { "type": "text_delta", "text": "12 pages." } }),
        ),
        (
            "content_block_stop",
            json!({ "type": "content_block_stop", "index": 0 }),
        ),
    ];
    answers.extend(stop("end_turn"));
    vec![
        Reply::Stream(events(&asks)),
        Reply::Stream(events(&answers)),
    ]
}

/// OpenAI's Chat Completions chunks, which OpenAI-compatible servers
/// such as LM Studio, llama.cpp and vLLM send too.
fn openai_turns(model: &str) -> Vec<Reply> {
    let chunk = |delta: Value, finish: Value| {
        (
            "",
            json!({ "id": "chatcmpl-1", "object": "chat.completion.chunk", "created": 1,
                "model": model,
                "choices": [{ "index": 0, "delta": delta, "finish_reason": finish }] }),
        )
    };
    let done = || "data: [DONE]\n\n".to_owned();
    let asks = events(&[
        chunk(
            json!({ "role": "assistant", "content": null, "tool_calls": [{
                "index": 0, "id": "call_01", "type": "function",
                "function": { "name": "page_count", "arguments": "" } }] }),
            Value::Null,
        ),
        chunk(
            json!({ "tool_calls": [{ "index": 0,
                "function": { "arguments": "{\"window\":1}" } }] }),
            Value::Null,
        ),
        chunk(json!({}), json!("tool_calls")),
    ]) + &done();
    let answers = events(&[
        chunk(
            json!({ "role": "assistant", "content": "It has " }),
            Value::Null,
        ),
        chunk(json!({ "content": "12 pages." }), Value::Null),
        chunk(json!({}), json!("stop")),
    ]) + &done();
    vec![Reply::Stream(asks), Reply::Stream(answers)]
}

fn gemini_turns() -> Vec<Reply> {
    let chunk = |parts: Value, finish: Option<&str>| {
        let mut candidate = json!({ "content": { "parts": parts, "role": "model" }, "index": 0 });
        if let Some(finish) = finish {
            candidate["finishReason"] = json!(finish);
        }
        (
            "",
            json!({ "candidates": [candidate],
                "usageMetadata": { "promptTokenCount": 10, "candidatesTokenCount": 5,
                    "totalTokenCount": 15 },
                "modelVersion": "gemini-test" }),
        )
    };
    vec![
        Reply::Stream(events(&[
            chunk(
                json!([{ "text": "Counting the pages.", "thought": true }]),
                None,
            ),
            chunk(
                json!([{ "functionCall": { "name": "page_count", "args": { "window": 1 } } }]),
                Some("STOP"),
            ),
        ])),
        Reply::Stream(events(&[
            chunk(json!([{ "text": "It has " }]), None),
            chunk(json!([{ "text": "12 pages." }]), Some("STOP")),
        ])),
    ]
}

/// OpenAI's Responses events, which rig uses for OpenAI's own models.
fn responses_turns() -> Vec<Reply> {
    let completed = |sequence: u64| {
        (
            "response.completed",
            json!({ "type": "response.completed", "sequence_number": sequence, "response": {
                "id": "resp_1", "object": "response", "created_at": 0, "status": "completed",
                "error": null, "incomplete_details": null, "instructions": null,
                "max_output_tokens": null, "model": "gpt-test", "usage": null,
                "output": [], "tools": [] } }),
        )
    };
    vec![
        Reply::Stream(events(&[
            (
                "response.output_item.done",
                json!({ "type": "response.output_item.done", "output_index": 0,
                    "sequence_number": 1, "item": {
                        "type": "function_call", "id": "fc_01", "call_id": "call_01",
                        "name": "page_count", "arguments": "{\"window\":1}",
                        "status": "completed" } }),
            ),
            completed(2),
        ])),
        Reply::Stream(events(&[
            (
                "response.output_text.delta",
                json!({ "type": "response.output_text.delta", "item_id": "msg_1",
                    "output_index": 0, "content_index": 0, "sequence_number": 1,
                    "delta": "It has " }),
            ),
            (
                "response.output_text.delta",
                json!({ "type": "response.output_text.delta", "item_id": "msg_1",
                    "output_index": 0, "content_index": 0, "sequence_number": 2,
                    "delta": "12 pages." }),
            ),
            completed(3),
        ])),
    ]
}

#[test]
fn anthropic_calls_a_tool_and_answers() {
    let heard = chat(Provider::Anthropic, "claude-test", anthropic_turns());
    check(&heard, "toolu_01");
    assert!(heard.requests[0].path.starts_with("/v1/messages"));
    assert_eq!(heard.reasoning, "Counting the pages.");
    // A model rig does not know still gets the max_tokens Anthropic needs.
    assert!(heard.requests[0].body["max_tokens"].is_u64());
    // The thinking goes back with its signature, as Anthropic requires.
    assert!(heard.requests[1].body.to_string().contains("c2lnbmVk"));
}

#[test]
fn openai_calls_a_tool_and_answers() {
    let heard = chat(Provider::OpenAi, "gpt-test", responses_turns());
    check(&heard, "call_01");
    assert!(heard.requests[0].path.starts_with("/v1/responses"));
}

#[test]
fn compatible_servers_call_a_tool_and_answer() {
    let heard = chat(
        Provider::OpenAiCompatible,
        "local-model",
        openai_turns("local-model"),
    );
    check(&heard, "call_01");
    // Chat Completions, which LM Studio, llama.cpp and vLLM speak.
    assert!(heard.requests[0].path.starts_with("/v1/chat/completions"));
}

#[test]
fn gemini_calls_a_tool_and_answers() {
    let heard = chat(Provider::Gemini, "gemini-test", gemini_turns());
    check(&heard, "page_count");
    assert!(
        heard.requests[0]
            .path
            .contains("/models/gemini-test:streamGenerateContent"),
        "{}",
        heard.requests[0].path
    );
    assert_eq!(heard.reasoning, "Counting the pages.");
}

#[test]
fn a_key_turned_down_reads_as_a_key_problem() {
    for provider in [Provider::Anthropic, Provider::OpenAi, Provider::Gemini] {
        let (address, _) = serve(vec![Reply::Failure(
            401,
            r#"{"error":{"message":"invalid x-api-key"}}"#,
        )]);
        let address = match provider {
            Provider::OpenAi => format!("{address}/v1"),
            _ => address,
        };
        let choice = ModelChoice {
            provider,
            model: "test-model".to_owned(),
            address: Some(address),
            context: None,
            vision: None,
        };
        assert_eq!(
            prev_assist::test(&choice, Some("bad-key")),
            Err(Problem::Key),
            "{provider:?}"
        );
    }
}

#[test]
fn providers_list_their_chat_models() {
    let (address, _) = serve(vec![Reply::Json(json!({
        "data": [
            { "id": "claude-sonnet-5-5", "display_name": "Claude Sonnet 5.5", "type": "model",
              "created_at": "2026-08-01T00:00:00Z" },
            { "id": "claude-haiku-4-5", "display_name": "Claude Haiku 4.5", "type": "model",
              "created_at": "2025-10-01T00:00:00Z" },
        ],
        "has_more": false, "first_id": "claude-sonnet-5-5", "last_id": "claude-haiku-4-5",
    }))]);
    let models = prev_assist::list(Provider::Anthropic, Some("key"), Some(&address)).unwrap();
    let names: Vec<&str> = models.iter().map(|model| model.name.as_str()).collect();
    assert_eq!(names, ["Claude Sonnet 5.5", "Claude Haiku 4.5"]);
    assert!(models[0].recommended);

    let (address, _) = serve(vec![Reply::Json(json!({
        "object": "list",
        "data": [
            { "id": "gpt-5.6", "object": "model", "created": 20, "owned_by": "openai" },
            { "id": "text-embedding-3-large", "object": "model", "created": 30, "owned_by": "openai" },
            { "id": "gpt-image-2", "object": "model", "created": 40, "owned_by": "openai" },
            { "id": "gpt-5-mini", "object": "model", "created": 10, "owned_by": "openai" },
        ],
    }))]);
    let address = format!("{address}/v1");
    let models = prev_assist::list(Provider::OpenAi, Some("key"), Some(&address)).unwrap();
    let ids: Vec<&str> = models.iter().map(|model| model.id.as_str()).collect();
    assert_eq!(
        ids,
        ["gpt-5.6", "gpt-5-mini"],
        "only chat models, newest first"
    );

    let (address, _) = serve(vec![Reply::Json(json!({
        "models": [
            { "name": "models/gemini-3-flash-preview", "displayName": "Gemini 3 Flash",
              "supportedGenerationMethods": ["generateContent"] },
            { "name": "models/gemini-embedding-001", "displayName": "Gemini Embedding",
              "supportedGenerationMethods": ["embedContent"] },
        ],
    }))]);
    let models = prev_assist::list(Provider::Gemini, Some("key"), Some(&address)).unwrap();
    let ids: Vec<&str> = models.iter().map(|model| model.id.as_str()).collect();
    assert_eq!(ids, ["gemini-3-flash-preview"]);
    assert_eq!(models[0].name, "Gemini 3 Flash");
}
