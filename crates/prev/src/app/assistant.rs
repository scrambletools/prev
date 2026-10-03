//! The assistant: the models Settings adds, with their keys in the system
//! keychain, and the chats the assistant panel holds with them.

use iced::Task;
use prev_assist::{ModelChoice, Problem, Provider};
use prev_store::settings::AssistantModel;

use super::{Message, Prev};

mod panel;

pub(crate) use panel::{Heard, Panel, PanelMessage};

/// The keychain service prev's keys are under.
fn keychain_service() -> &'static str {
    if prev_store::paths::PRODUCTION {
        "prev"
    } else {
        "prev-dev"
    }
}

/// The provider's name in `prev.toml`.
pub(super) fn provider_id(provider: Provider) -> &'static str {
    match provider {
        Provider::Anthropic => "anthropic",
        Provider::OpenAi => "open-ai",
        Provider::Gemini => "gemini",
        Provider::Ollama => "ollama",
        Provider::OpenAiCompatible => "open-ai-compatible",
    }
}

fn provider_of(id: &str) -> Option<Provider> {
    Provider::ALL
        .into_iter()
        .find(|provider| provider_id(*provider) == id)
}

/// A provider as the Settings menu lists it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ProviderChoice(pub(crate) Provider);

impl std::fmt::Display for ProviderChoice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&provider_label(self.0))
    }
}

pub(super) fn provider_label(provider: Provider) -> String {
    match provider {
        Provider::Anthropic => "Anthropic".to_owned(),
        Provider::OpenAi => "OpenAI".to_owned(),
        Provider::Gemini => "Google Gemini".to_owned(),
        Provider::Ollama => "Ollama".to_owned(),
        Provider::OpenAiCompatible => prev::fl!("settings-assistant-compatible"),
    }
}

/// A context size as Settings' menus list it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ContextSize(pub(crate) u32);

impl std::fmt::Display for ContextSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&prev::fl!(
            "settings-assistant-context-size",
            thousands = (self.0 / 1024).to_string()
        ))
    }
}

/// The sizes the menus offer.
pub(crate) fn context_sizes() -> Vec<ContextSize> {
    prev_assist::CONTEXT_SIZES.map(ContextSize).to_vec()
}

/// The model being added in Settings.
#[derive(Debug, Clone)]
pub(super) struct ModelForm {
    pub(super) provider: Provider,
    pub(super) model: String,
    pub(super) key: String,
    pub(super) address: String,
    pub(super) context: u32,
    /// What the last test or add said: `Ok` with a note, or the problem.
    pub(super) status: Option<Result<String, String>>,
    pub(super) busy: bool,
}

impl Default for ModelForm {
    fn default() -> Self {
        Self {
            provider: Provider::Anthropic,
            model: String::new(),
            key: String::new(),
            address: String::new(),
            context: prev_assist::DEFAULT_CONTEXT,
            status: None,
            busy: false,
        }
    }
}

impl ModelForm {
    fn choice(&self) -> ModelChoice {
        ModelChoice {
            provider: self.provider,
            model: self.model.trim().to_owned(),
            address: Some(self.address.trim().to_owned()).filter(|address| !address.is_empty()),
            context: self.provider.sets_context().then_some(self.context),
        }
    }

    fn key(&self) -> Option<String> {
        Some(self.key.trim().to_owned()).filter(|key| !key.is_empty())
    }
}

/// What the Assistant tab of Settings asks for.
#[derive(Debug, Clone)]
pub(crate) enum ModelMessage {
    Provider(ProviderChoice),
    Model(String),
    Key(String),
    Address(String),
    /// The context for the model being added.
    FormContext(ContextSize),
    /// A new context for the model with this id.
    Context(String, ContextSize),
    Test,
    Tested(Result<(), Problem>),
    Add,
    Added(AssistantModel, Result<(), String>),
    Use(String),
    Remove(String),
    /// The removed model's key is gone from the keychain, or could not go.
    Removed(Result<(), String>),
}

/// A model's settings entry as a choice to chat with.
pub(super) fn choice_of(model: &AssistantModel) -> Option<ModelChoice> {
    Some(ModelChoice {
        provider: provider_of(&model.provider)?,
        model: model.model.clone(),
        address: model.address.clone(),
        context: model.context,
    })
}

/// What went wrong with `model` from `provider`, said plainly.
pub(super) fn problem_text(problem: &Problem, provider: Provider, model: &str) -> String {
    let provider = provider_label(provider);
    match problem {
        Problem::ContextTooSmall => prev::fl!("assistant-problem-context", model = model),
        Problem::Key => prev::fl!("assistant-problem-key", provider = provider),
        Problem::RateLimited => prev::fl!("assistant-problem-rate", provider = provider),
        Problem::NoSuchModel => {
            prev::fl!(
                "assistant-problem-model",
                provider = provider,
                model = model
            )
        }
        Problem::Unavailable => prev::fl!("assistant-problem-unavailable", provider = provider),
        Problem::Unreachable => prev::fl!("assistant-problem-unreachable", provider = provider),
        Problem::Refused => prev::fl!("assistant-problem-refused", model = model),
        Problem::Other(message) => {
            prev::fl!(
                "assistant-problem-other",
                model = model,
                message = message.as_str()
            )
        }
    }
}

/// The key kept for model `id`. Blocks: call it off the interface thread.
pub(super) fn key_of(id: &str) -> Result<Option<String>, String> {
    prev_assist::keys::get(keychain_service(), id)
}

impl Prev {
    pub(super) fn model_settings(&mut self, message: ModelMessage) -> Task<Message> {
        let form = &mut self.model_form;
        match message {
            ModelMessage::Provider(ProviderChoice(provider)) => {
                form.provider = provider;
                form.status = None;
            }
            ModelMessage::Model(model) => form.model = model,
            ModelMessage::Key(key) => form.key = key,
            ModelMessage::Address(address) => form.address = address,
            ModelMessage::FormContext(ContextSize(size)) => form.context = size,
            ModelMessage::Context(id, ContextSize(size)) => {
                let Some(model) = self
                    .settings
                    .assistant_models
                    .iter_mut()
                    .find(|model| model.id == id)
                else {
                    return Task::none();
                };
                model.context = Some(size);
                self.save_settings();
                self.model_changed(&id);
            }
            ModelMessage::Test => {
                let (choice, key) = (form.choice(), form.key());
                form.busy = true;
                form.status = None;
                return Task::perform(
                    prev::image::editor::spawn(move || prev_assist::test(&choice, key.as_deref())),
                    |result| {
                        Message::ModelSettings(ModelMessage::Tested(result.unwrap_or_else(|_| {
                            Err(Problem::Other("The test stopped.".to_owned()))
                        })))
                    },
                );
            }
            ModelMessage::Tested(result) => {
                form.busy = false;
                let (provider, model) = (form.provider, form.model.trim().to_owned());
                form.status = Some(
                    result
                        .map(|()| prev::fl!("settings-assistant-works"))
                        .map_err(|problem| problem_text(&problem, provider, &model)),
                );
            }
            ModelMessage::Add => {
                let choice = form.choice();
                if choice.model.is_empty() {
                    form.status = Some(Err(prev::fl!("settings-assistant-name-needed")));
                    return Task::none();
                }
                if choice.provider.needs_key() && form.key().is_none() {
                    form.status = Some(Err(prev::fl!("settings-assistant-key-needed")));
                    return Task::none();
                }
                let base = format!("{}-{}", provider_id(choice.provider), choice.model);
                let mut id = base.clone();
                let mut n = 2;
                while self
                    .settings
                    .assistant_models
                    .iter()
                    .any(|model| model.id == id)
                {
                    id = format!("{base}-{n}");
                    n += 1;
                }
                let model = AssistantModel {
                    id,
                    provider: provider_id(choice.provider).to_owned(),
                    model: choice.model,
                    address: choice.address,
                    context: choice.context,
                };
                let key = form.key();
                form.busy = true;
                let stored = model.clone();
                return Task::perform(
                    prev::image::editor::spawn(move || match key {
                        Some(key) => prev_assist::keys::set(keychain_service(), &stored.id, &key),
                        None => Ok(()),
                    }),
                    move |result| {
                        Message::ModelSettings(ModelMessage::Added(
                            model.clone(),
                            result.unwrap_or_else(|_| Err("Keeping the key stopped.".to_owned())),
                        ))
                    },
                );
            }
            ModelMessage::Added(model, result) => {
                form.busy = false;
                if let Err(error) = result {
                    form.status = Some(Err(prev::fl!(
                        "settings-assistant-key-failed",
                        error = error
                    )));
                    return Task::none();
                }
                if self.settings.assistant_model.is_none() {
                    self.settings.assistant_model = Some(model.id.clone());
                }
                self.settings.assistant_models.push(model);
                self.model_form = ModelForm {
                    provider: self.model_form.provider,
                    ..ModelForm::default()
                };
                self.save_settings();
            }
            ModelMessage::Removed(result) => {
                if let Err(error) = result {
                    form.status = Some(Err(prev::fl!(
                        "settings-assistant-key-failed",
                        error = error
                    )));
                }
            }
            ModelMessage::Use(id) => {
                self.settings.assistant_model = Some(id);
                self.save_settings();
            }
            ModelMessage::Remove(id) => {
                self.settings
                    .assistant_models
                    .retain(|model| model.id != id);
                if self.settings.assistant_model.as_deref() == Some(id.as_str()) {
                    self.settings.assistant_model = self
                        .settings
                        .assistant_models
                        .first()
                        .map(|model| model.id.clone());
                }
                self.save_settings();
                return Task::perform(
                    prev::image::editor::spawn(move || {
                        prev_assist::keys::delete(keychain_service(), &id)
                    }),
                    |result| {
                        Message::ModelSettings(ModelMessage::Removed(
                            result.unwrap_or_else(|_| Err("Removing the key stopped.".to_owned())),
                        ))
                    },
                );
            }
        }
        Task::none()
    }
}
