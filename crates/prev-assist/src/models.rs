//! Finding models to chat with: the ones a provider serves, what each can
//! do, and the local servers running on this computer, so Settings can
//! offer models to pick rather than names to type.

use std::time::Duration;

use serde_json::Value;

use crate::{ModelChoice, Problem, Provider, example_model};

/// How long a local server has to answer before prev takes it as not
/// running.
const LOCAL_WAIT: Duration = Duration::from_millis(800);

/// The addresses OpenAI-compatible servers listen on unless told
/// otherwise: LM Studio, llama.cpp's server, and vLLM.
const COMPATIBLE_ADDRESSES: [&str; 3] = [
    "http://localhost:1234/v1",
    "http://localhost:8080/v1",
    "http://localhost:8000/v1",
];

/// A model a provider serves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelInfo {
    /// The name requests use.
    pub id: String,
    /// The name to show.
    pub name: String,
    /// Whether it can call tools, when the provider says. prev's
    /// assistant does everything through tools, so one that cannot is no
    /// use.
    pub tools: Option<bool>,
    /// Whether it can see pictures, when the provider says.
    pub vision: Option<bool>,
    /// The most it reads at once, in tokens, when the provider says.
    pub context: Option<u32>,
    /// The one to suggest first.
    pub recommended: bool,
}

/// Where to get an API key for `provider`.
pub fn key_page(provider: Provider) -> Option<&'static str> {
    match provider {
        Provider::Anthropic => Some("https://console.anthropic.com/settings/keys"),
        Provider::OpenAi => Some("https://platform.openai.com/api-keys"),
        Provider::Gemini => Some("https://aistudio.google.com/apikey"),
        Provider::Ollama | Provider::OpenAiCompatible => None,
    }
}

/// Where to buy credit for `provider`.
pub fn billing_page(provider: Provider) -> Option<&'static str> {
    match provider {
        Provider::Anthropic => Some("https://console.anthropic.com/settings/billing"),
        Provider::OpenAi => Some("https://platform.openai.com/settings/organization/billing"),
        // Gemini's keys start on a free tier, which needs no credit.
        Provider::Gemini | Provider::Ollama | Provider::OpenAiCompatible => None,
    }
}

/// Where to get the server for a local provider.
pub fn server_page(provider: Provider) -> Option<&'static str> {
    match provider {
        Provider::Ollama => Some("https://ollama.com/download"),
        Provider::OpenAiCompatible => Some("https://lmstudio.ai"),
        _ => None,
    }
}

/// The address of the server for `provider` that answers: `given` alone,
/// when the user gave one, else Ollama's usual one, or those of
/// OpenAI-compatible servers. Blocks: call it off the interface thread.
pub fn find_server(provider: Provider, given: Option<&str>) -> Option<String> {
    let given = given
        .map(|address| address.trim().trim_end_matches('/'))
        .filter(|address| !address.is_empty());
    let addresses: Vec<String> = match (given, provider) {
        (Some(address), _) => vec![address.to_owned()],
        (None, Provider::Ollama) => provider
            .default_address()
            .map(str::to_owned)
            .into_iter()
            .collect(),
        (None, Provider::OpenAiCompatible) => COMPATIBLE_ADDRESSES.map(str::to_owned).to_vec(),
        (None, _) => return None,
    };
    let client = reqwest::blocking::Client::builder()
        .timeout(LOCAL_WAIT)
        .build()
        .ok()?;
    addresses.into_iter().find(|address| {
        let probe = match provider {
            Provider::Ollama => format!("{address}/api/tags"),
            _ => format!("{address}/models"),
        };
        client
            .get(probe)
            .send()
            .is_ok_and(|reply| reply.status().is_success())
    })
}

/// The chat models `provider` serves, the recommended one first. A local
/// provider is asked at `address`. Blocks: call it off the interface
/// thread.
pub fn list(
    provider: Provider,
    key: Option<&str>,
    address: Option<&str>,
) -> Result<Vec<ModelInfo>, Problem> {
    let mut models = match provider {
        Provider::Ollama => ollama_models(address.or(provider.default_address()))?,
        _ => listed_models(provider, key, address)?,
    };
    let suggested = example_model(provider);
    let first = models
        .iter()
        .position(|model| model.id == suggested || model.id == format!("{suggested}:latest"))
        .or_else(|| models.iter().position(|model| model.tools != Some(false)));
    if let Some(first) = first {
        let model = models.remove(first);
        models.insert(
            0,
            ModelInfo {
                recommended: true,
                ..model
            },
        );
    }
    Ok(models)
}

/// What rig lists for a cloud or compatible provider, kept to the models
/// that chat.
fn listed_models(
    provider: Provider,
    key: Option<&str>,
    address: Option<&str>,
) -> Result<Vec<ModelInfo>, Problem> {
    use rig_core::providers::{anthropic, gemini, openai};
    let key = key.unwrap_or_default().to_owned();
    if provider.needs_key() && key.trim().is_empty() {
        return Err(Problem::Key);
    }
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|error| Problem::Other(error.to_string()))?;
    let listed = runtime.block_on(async {
        match (provider, address) {
            (Provider::Anthropic, None) => anthropic::Anthropic::new(key).list_models().await,
            (Provider::OpenAi, None) => openai::OpenAI::new(key).list_models().await,
            (Provider::Gemini, None) => gemini::Gemini::new(key).list_models().await,
            // Elsewhere, as the recorded tests' server.
            (Provider::Anthropic, Some(address)) => {
                let mut config = anthropic::AnthropicConfig::new(key);
                config.base_url = address.to_owned();
                config.client().list_models().await
            }
            (Provider::Gemini, Some(address)) => {
                let mut config = gemini::GeminiConfig::new(key);
                config.base_url = address.to_owned();
                config.client().list_models().await
            }
            _ => {
                let mut config = openai::OpenAIConfig::new(key);
                if let Some(address) = address.or(provider.default_address()) {
                    config.base_url = address.to_owned();
                }
                config.client().list_models().await
            }
        }
    });
    let mut models: Vec<rig_core::model::ModelInfo> =
        listed.map_err(|error| Problem::of(&error))?.data;
    // Newest first, where the provider dates them.
    models.sort_by_key(|model| std::cmp::Reverse(model.created_at.unwrap_or(0)));
    Ok(models
        .into_iter()
        .filter(|model| chats(provider, &model.id))
        .map(|model| {
            let id = model.id.trim_start_matches("models/").to_owned();
            let name = model
                .name
                .clone()
                .filter(|name| !name.trim().is_empty())
                .unwrap_or_else(|| friendly_name(provider, &id));
            ModelInfo {
                // Every recent cloud model calls tools and sees; a
                // compatible server does not say.
                tools: provider.needs_key().then_some(true),
                vision: provider.needs_key().then_some(true),
                context: model.context_length,
                recommended: false,
                id,
                name,
            }
        })
        .collect())
}

/// Whether model `id` of `provider` is one to chat with, rather than one
/// that draws, speaks, listens or embeds.
fn chats(provider: Provider, id: &str) -> bool {
    const NOT_CHAT: [&str; 13] = [
        "embed",
        "embedding",
        "tts",
        "transcribe",
        "whisper",
        "audio",
        "realtime",
        "image",
        "imagen",
        "dall-e",
        "moderation",
        "veo",
        "aqa",
    ];
    let id = id.to_lowercase();
    if NOT_CHAT.iter().any(|word| id.contains(word)) {
        return false;
    }
    match provider {
        Provider::OpenAi => {
            id.starts_with("gpt-")
                || id.starts_with("chatgpt-")
                || id.starts_with('o') && id[1..].starts_with(|c: char| c.is_ascii_digit())
        }
        Provider::Gemini => id.contains("gemini"),
        _ => true,
    }
}

/// The models a local Ollama has, with what each can do.
fn ollama_models(address: Option<&str>) -> Result<Vec<ModelInfo>, Problem> {
    let address = address
        .unwrap_or("http://localhost:11434")
        .trim_end_matches('/');
    let client = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(|error| Problem::Other(error.to_string()))?;
    let tags: Value = client
        .get(format!("{address}/api/tags"))
        .send()
        .and_then(|reply| reply.error_for_status())
        .and_then(|reply| reply.json())
        .map_err(|_| Problem::Unreachable)?;
    let mut models = Vec::new();
    for model in tags["models"].as_array().into_iter().flatten() {
        let Some(id) = model["name"].as_str() else {
            continue;
        };
        let shown: Value = client
            .post(format!("{address}/api/show"))
            .json(&serde_json::json!({ "model": id }))
            .send()
            .and_then(|reply| reply.json())
            .unwrap_or(Value::Null);
        let capabilities: Option<Vec<&str>> = shown["capabilities"]
            .as_array()
            .map(|list| list.iter().filter_map(Value::as_str).collect());
        if capabilities
            .as_ref()
            .is_some_and(|list| list.contains(&"embedding") && !list.contains(&"completion"))
        {
            continue;
        }
        let context = shown["model_info"].as_object().and_then(|info| {
            info.iter()
                .find(|(key, _)| key.ends_with(".context_length"))
                .and_then(|(_, value)| value.as_u64())
                .and_then(|value| u32::try_from(value).ok())
        });
        models.push(ModelInfo {
            id: id.to_owned(),
            name: friendly_name(Provider::Ollama, id),
            tools: capabilities.as_ref().map(|list| list.contains(&"tools")),
            vision: capabilities.as_ref().map(|list| list.contains(&"vision")),
            context,
            recommended: false,
        });
    }
    Ok(models)
}

/// A model's name as people read it: Ollama's tags lose the registry and
/// `:latest`, and their size tag reads after a space, such as `gemma4 26b`.
pub fn friendly_name(provider: Provider, id: &str) -> String {
    match provider {
        Provider::Ollama => {
            let id = id.rsplit('/').next().unwrap_or(id);
            match id.split_once(':') {
                Some((name, "latest")) => name.to_owned(),
                Some((name, tag)) => format!("{name} {tag}"),
                None => id.to_owned(),
            }
        }
        _ => id.to_owned(),
    }
}

/// A model's choice, from what the provider listed.
impl ModelInfo {
    pub fn choice(&self, provider: Provider, address: Option<String>) -> ModelChoice {
        ModelChoice {
            provider,
            model: self.id.clone(),
            address,
            context: None,
            vision: self.vision,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_names_read_plainly() {
        let name = |id| friendly_name(Provider::Ollama, id);
        assert_eq!(name("qwen3.8:latest"), "qwen3.8");
        assert_eq!(name("gemma4:26b"), "gemma4 26b");
        assert_eq!(
            name("hf.co/unsloth/Qwen3.8-27B-GGUF:UD-Q4_K_XL"),
            "Qwen3.8-27B-GGUF UD-Q4_K_XL"
        );
        assert_eq!(name("llama3"), "llama3");
    }

    #[test]
    fn only_chat_models_are_offered() {
        assert!(chats(Provider::OpenAi, "gpt-5.6"));
        assert!(chats(Provider::OpenAi, "o4-mini"));
        assert!(!chats(Provider::OpenAi, "omni-moderation-latest"));
        assert!(!chats(Provider::OpenAi, "gpt-image-2"));
        assert!(!chats(Provider::OpenAi, "text-embedding-3-large"));
        assert!(!chats(Provider::OpenAi, "gpt-4o-realtime-preview"));
        assert!(chats(Provider::Gemini, "models/gemini-3-flash-preview"));
        assert!(!chats(Provider::Gemini, "models/gemini-embedding-001"));
        assert!(!chats(Provider::Gemini, "models/imagen-4"));
        assert!(chats(Provider::Anthropic, "claude-sonnet-5-5"));
    }
}
