//! Google Gemini with a real key: listing its models, the check Add
//! makes, and a chat that has to call a tool. Ignored by default, as it
//! needs a key and the network:
//!
//!     PREV_ASSIST_GEMINI_KEY=... cargo test -p prev-assist --test gemini -- --ignored
//!
//! PREV_ASSIST_MODEL picks the model; the recommended one otherwise.

use std::sync::mpsc;
use std::time::Duration;

use prev_assist::{Chat, Content, Event, ModelChoice, Provider, ToolResult, ToolSpec};
use serde_json::json;

fn key() -> String {
    std::env::var("PREV_ASSIST_GEMINI_KEY").expect("PREV_ASSIST_GEMINI_KEY is set")
}

/// The model to use: PREV_ASSIST_MODEL, or the one the list recommends.
fn model() -> String {
    std::env::var("PREV_ASSIST_MODEL").unwrap_or_else(|_| {
        prev_assist::list(Provider::Gemini, Some(&key()), None)
            .unwrap()
            .into_iter()
            .find(|model| model.recommended)
            .expect("a recommended model")
            .id
    })
}

fn choice() -> ModelChoice {
    ModelChoice {
        provider: Provider::Gemini,
        model: model(),
        address: None,
        context: None,
        vision: None,
    }
}

/// A one-pixel white PNG.
const PIXEL: [u8; 69] = [
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x02, 0x00, 0x00, 0x00, 0x90, 0x77, 0x53,
    0xde, 0x00, 0x00, 0x00, 0x0c, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xf8, 0xff, 0xff, 0x3f,
    0x00, 0x05, 0xfe, 0x02, 0xfe, 0x0d, 0xef, 0x46, 0xb8, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
    0x44, 0xae, 0x42, 0x60, 0x82,
];

#[test]
#[ignore = "needs a Gemini key"]
fn gemini_models_are_listed_with_one_recommended() {
    let models = prev_assist::list(Provider::Gemini, Some(&key()), None).unwrap();
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

#[test]
#[ignore = "needs a Gemini key"]
fn a_gemini_model_answers_the_check_add_makes() {
    prev_assist::test(&choice(), Some(&key())).unwrap();
}

#[test]
#[ignore = "needs a Gemini key"]
fn a_gemini_model_calls_a_tool_and_answers() {
    let (events, received) = mpsc::channel();
    let chat = Chat::start(
        choice(),
        Some(key()),
        "You help with documents in prev. Use the tools to answer.".to_owned(),
        vec![ToolSpec {
            name: "page_count".to_owned(),
            description: "Gives the number of pages of the open document.".to_owned(),
            schema: json!({ "type": "object", "properties": {} }),
        }],
        move |event| {
            let _ = events.send(event);
        },
    )
    .unwrap();
    chat.say("How many pages does the open document have? Answer with the number.");
    let mut called = false;
    let mut reply = String::new();
    loop {
        match received.recv_timeout(Duration::from_secs(120)).unwrap() {
            Event::Text(text) => reply.push_str(&text),
            Event::Reasoning(_) => {}
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
                            // With a picture, as render tools give.
                            content: vec![
                                Content::Text("12".to_owned()),
                                Content::Png(PIXEL.to_vec()),
                            ],
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
    assert!(called, "the model answered without the tool: {reply}");
    assert!(reply.contains("12"), "{reply}");
}
