//! A chat with a local Ollama model that has to call a tool. Ignored by
//! default, as it needs Ollama running with the model pulled:
//!
//!     PREV_ASSIST_MODEL=qwen3.8 cargo test -p prev-assist --test ollama -- --ignored

use std::sync::mpsc;
use std::time::Duration;

use prev_assist::{Chat, Content, Event, ModelChoice, Provider, ToolResult, ToolSpec};
use serde_json::json;

/// page_count, among as many other tools as prev has, whose definitions
/// take more than Ollama's default context.
fn tools() -> Vec<ToolSpec> {
    let mut tools = vec![ToolSpec {
        name: "page_count".to_owned(),
        description: "Gives the number of pages of the open document.".to_owned(),
        schema: json!({ "type": "object", "properties": {} }),
    }];
    let words = "Changes the open document in a way that has nothing to do with counting \
        its pages, and is described at length, as prev's tools are. "
        .repeat(8);
    tools.extend((0..45).map(|n| ToolSpec {
        name: format!("other_tool_{n}"),
        description: words.clone(),
        schema: json!({
            "type": "object",
            "properties": {
                "window": { "type": "integer", "description": "The window to act on." },
                "page": { "type": "integer", "description": "The page, counting from 1." },
            },
        }),
    }));
    tools
}

#[test]
#[ignore = "needs a local Ollama"]
fn a_local_model_calls_a_tool_and_answers() {
    let model = std::env::var("PREV_ASSIST_MODEL").unwrap_or_else(|_| "qwen3.8".to_owned());
    let (events, received) = mpsc::channel();
    let chat = Chat::start(
        ModelChoice {
            provider: Provider::Ollama,
            model,
            address: None,
            context: None,
            vision: None,
        },
        None,
        "You help with documents in prev. Use the tools to answer.".to_owned(),
        tools(),
        move |event| {
            let _ = events.send(event);
        },
    )
    .unwrap();
    chat.say("How many pages does the open document have? Answer with the number.");
    let mut called = false;
    let mut reply = String::new();
    let mut thought = String::new();
    loop {
        match received.recv_timeout(Duration::from_secs(300)).unwrap() {
            Event::Text(text) => reply.push_str(&text),
            Event::Reasoning(text) => thought.push_str(&text),
            Event::ToolCalls(calls) => {
                assert!(
                    calls.iter().all(|call| call.name == "page_count"),
                    "{calls:?}"
                );
                called = true;
                chat.results(
                    calls
                        .into_iter()
                        .map(|call| ToolResult {
                            id: call.id,
                            content: vec![Content::Text("12".to_owned())],
                            failed: false,
                        })
                        .collect(),
                );
            }
            Event::Done => break,
            Event::Failed(problem) => panic!("{problem:?}"),
            Event::Stopped => panic!("nothing stopped the chat"),
        }
    }
    eprintln!("thought {} characters", thought.len());
    assert!(called, "the model answered without the tool: {reply}");
    assert!(reply.contains("12"), "{reply}");
}

#[test]
#[ignore = "needs a local Ollama"]
fn local_models_are_found_with_what_they_can_do() {
    let address = prev_assist::find_server(Provider::Ollama, None).expect("Ollama is running");
    let models = prev_assist::list(Provider::Ollama, None, Some(&address)).unwrap();
    for model in &models {
        eprintln!(
            "{} ({}): tools {:?}, vision {:?}, context {:?}{}",
            model.name,
            model.id,
            model.tools,
            model.vision,
            model.context,
            if model.recommended {
                ", recommended"
            } else {
                ""
            }
        );
    }
    assert!(!models.is_empty());
    assert!(models[0].recommended);
}
