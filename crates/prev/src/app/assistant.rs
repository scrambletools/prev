//! The assistant: the models Settings adds, with their keys in the system
//! keychain, and the chats the assistant panel holds with them.

use iced::Task;
use prev_assist::{ModelChoice, ModelInfo, Problem, Provider};
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

/// Settings' search for a model to add.
#[derive(Debug, Clone, Default)]
pub(super) enum Found {
    #[default]
    Nothing,
    Looking,
    /// The provider's chat models, and the address they were found at.
    Models(Option<String>, Vec<ModelInfo>),
    /// No local server answered.
    NoServer,
    /// The provider would not list its models.
    Failed(String),
}

/// The model being added in Settings: a provider, then its models to pick
/// from, found with the key prev keeps for it or the address its server
/// answers at.
#[derive(Debug, Clone)]
pub(super) struct ModelForm {
    pub(super) provider: Provider,
    /// A key typed for a provider that has none kept.
    pub(super) key: String,
    /// Whether the keychain has the provider's key: `None` while asking.
    pub(super) key_kept: Option<bool>,
    /// The key is being changed.
    pub(super) changing_key: bool,
    /// An address typed for a local server.
    pub(super) address: String,
    pub(super) found: Found,
    /// A model name typed, for one the provider does not list.
    pub(super) typed: String,
    pub(super) context: u32,
    /// The model being tried before it is added.
    pub(super) adding: Option<String>,
    /// What the last add said: `Ok` with a note, or the problem.
    pub(super) status: Option<Result<String, String>>,
    /// Counts the searches, so an old one's answer is dropped.
    search: u64,
}

impl Default for ModelForm {
    fn default() -> Self {
        Self {
            provider: Provider::Ollama,
            key: String::new(),
            key_kept: None,
            changing_key: false,
            address: String::new(),
            found: Found::Nothing,
            typed: String::new(),
            context: prev_assist::DEFAULT_CONTEXT,
            adding: None,
            status: None,
            search: 0,
        }
    }
}

impl ModelForm {
    /// Whether the form asks for a key: the provider needs one and none is
    /// kept, or it is being changed.
    pub(super) fn asks_for_key(&self) -> bool {
        self.provider.needs_key() && (self.key_kept == Some(false) || self.changing_key)
    }

    fn typed_key(&self) -> Option<String> {
        Some(self.key.trim().to_owned()).filter(|key| !key.is_empty())
    }

    fn typed_address(&self) -> Option<String> {
        Some(self.address.trim().to_owned()).filter(|address| !address.is_empty())
    }
}

/// What the Assistant tab of Settings asks for.
#[derive(Debug, Clone)]
pub(crate) enum ModelMessage {
    Provider(ProviderChoice),
    Key(String),
    /// Look for the provider's models with the key typed.
    UseKey,
    ChangeKey,
    /// The keychain answered whether it has the provider's key: the
    /// search, and the key.
    KeyChecked(u64, Result<Option<String>, String>),
    Address(String),
    /// Look again, at the address typed or the usual ones.
    Search,
    /// The models found, with the address they were found at, and the
    /// key typed, to keep once it worked.
    Searched(
        u64,
        Option<String>,
        Option<String>,
        Result<Vec<ModelInfo>, Problem>,
    ),
    Typed(String),
    /// Try the model with this id, then add it.
    Add(String),
    Tried(AssistantModel, Result<(), Problem>),
    /// The context for the model being added.
    FormContext(ContextSize),
    /// A new context for the model with this id.
    Context(String, ContextSize),
    Use(String),
    Remove(String),
    /// The removed model's key is gone from the keychain, or could not go.
    Removed(Result<(), String>),
    OpenLink(&'static str),
}

/// A model's settings entry as a choice to chat with.
pub(super) fn choice_of(model: &AssistantModel) -> Option<ModelChoice> {
    Some(ModelChoice {
        provider: provider_of(&model.provider)?,
        model: model.model.clone(),
        address: model.address.clone(),
        context: model.context,
        vision: model.vision,
    })
}

/// The name a model shows by: the provider's for it, else a tidy form of
/// its id.
pub(super) fn shown_name(model: &AssistantModel) -> String {
    match (&model.name, provider_of(&model.provider)) {
        (Some(name), _) => name.clone(),
        (None, Some(provider)) => prev_assist::friendly_name(provider, &model.model),
        (None, None) => model.model.clone(),
    }
}

/// The keychain entry of a provider's key: one per cloud provider, and
/// one per compatible server's address.
fn key_id(provider: Provider, address: Option<&str>) -> String {
    match (provider, address) {
        (Provider::OpenAiCompatible, Some(address)) => {
            format!("{}@{address}", provider_id(provider))
        }
        _ => provider_id(provider).to_owned(),
    }
}

/// The key kept for `model`: its provider's, else one kept for the model
/// alone, as prev 2.0's first builds did. Blocks: call it off the
/// interface thread.
pub(super) fn key_of(model: &AssistantModel) -> Result<Option<String>, String> {
    let service = keychain_service();
    if let Some(provider) = provider_of(&model.provider) {
        let shared = prev_assist::keys::get(service, &key_id(provider, model.address.as_deref()))?;
        if shared.is_some() {
            return Ok(shared);
        }
    }
    prev_assist::keys::get(service, &model.id)
}

/// Runs `work` off the interface thread, then makes its message.
fn off_thread<T: Send + 'static>(
    work: impl FnOnce() -> T + Send + 'static,
    stopped: impl FnOnce() -> T + Send + 'static,
    message: impl FnOnce(T) -> ModelMessage + Send + 'static,
) -> Task<Message> {
    Task::perform(prev::image::editor::spawn(work), move |result| {
        Message::ModelSettings(message(result.unwrap_or_else(|_| stopped())))
    })
}

impl Prev {
    /// Looks for models to add when the Assistant tab shows and has not
    /// looked yet.
    pub(super) fn assistant_tab_shown(&mut self) -> Task<Message> {
        let fresh =
            matches!(self.model_form.found, Found::Nothing) && self.model_form.key_kept.is_none();
        if self.settings_tab == super::settings_view::SettingsTab::Assistant && fresh {
            self.search_models()
        } else {
            Task::none()
        }
    }

    /// Starts looking for the form's provider's models: with the key
    /// kept for it, or at its server.
    fn search_models(&mut self) -> Task<Message> {
        let form = &mut self.model_form;
        form.search += 1;
        form.status = None;
        let (search, provider) = (form.search, form.provider);
        if provider.needs_key() {
            if let Some(key) = form.typed_key().filter(|_| form.asks_for_key()) {
                form.found = Found::Looking;
                return off_thread(
                    move || prev_assist::list(provider, Some(&key), None).map(|list| (list, key)),
                    || Err(Problem::Other("The search stopped.".to_owned())),
                    move |result| match result {
                        Ok((list, key)) => {
                            ModelMessage::Searched(search, None, Some(key), Ok(list))
                        }
                        Err(problem) => ModelMessage::Searched(search, None, None, Err(problem)),
                    },
                );
            }
            form.key_kept = None;
            form.found = Found::Nothing;
            return off_thread(
                move || prev_assist::keys::get(keychain_service(), &key_id(provider, None)),
                || Err("The keychain stopped.".to_owned()),
                move |result| ModelMessage::KeyChecked(search, result),
            );
        }
        form.found = Found::Looking;
        let address = form.typed_address();
        off_thread(
            move || {
                let found = prev_assist::find_server(provider, address.as_deref());
                let models = match &found {
                    Some(address) => prev_assist::list(provider, None, Some(address)),
                    None => Err(Problem::Unreachable),
                };
                (found, models)
            },
            || (None, Err(Problem::Unreachable)),
            move |(address, models)| ModelMessage::Searched(search, address, None, models),
        )
    }

    pub(super) fn model_settings(&mut self, message: ModelMessage) -> Task<Message> {
        let form = &mut self.model_form;
        match message {
            ModelMessage::Provider(ProviderChoice(provider)) => {
                *form = ModelForm {
                    provider,
                    search: form.search,
                    ..ModelForm::default()
                };
                return self.search_models();
            }
            ModelMessage::Key(key) => form.key = key,
            ModelMessage::UseKey => {
                if form.typed_key().is_none() {
                    form.status = Some(Err(prev::fl!("settings-assistant-key-needed")));
                    return Task::none();
                }
                return self.search_models();
            }
            ModelMessage::ChangeKey => {
                form.changing_key = true;
                form.key.clear();
                form.found = Found::Nothing;
            }
            ModelMessage::KeyChecked(search, result) => {
                if search != form.search {
                    return Task::none();
                }
                match result {
                    Ok(Some(key)) => {
                        form.key_kept = Some(true);
                        form.found = Found::Looking;
                        let provider = form.provider;
                        return off_thread(
                            move || prev_assist::list(provider, Some(&key), None),
                            || Err(Problem::Other("The search stopped.".to_owned())),
                            move |models| ModelMessage::Searched(search, None, None, models),
                        );
                    }
                    Ok(None) => form.key_kept = Some(false),
                    Err(error) => {
                        form.key_kept = Some(false);
                        form.status = Some(Err(prev::fl!(
                            "settings-assistant-key-failed",
                            error = error
                        )));
                    }
                }
            }
            ModelMessage::Address(address) => form.address = address,
            ModelMessage::Search => return self.search_models(),
            ModelMessage::Searched(search, address, key, models) => {
                if search != form.search {
                    return Task::none();
                }
                let provider = form.provider;
                // Found somewhere other than the address typed: the typed
                // one did not answer, so it goes.
                if let (Ok(_), Some(found)) = (&models, &address)
                    && form
                        .typed_address()
                        .is_some_and(|typed| typed.trim_end_matches('/') != found)
                {
                    form.address.clear();
                }
                form.found = match models {
                    Ok(models) => Found::Models(address, models),
                    Err(Problem::Unreachable) if !provider.needs_key() => Found::NoServer,
                    // Settings is where the key is typed, so it says so.
                    Err(Problem::Key) => Found::Failed(prev::fl!(
                        "settings-assistant-key-refused",
                        provider = provider_label(provider)
                    )),
                    Err(problem) => {
                        let model = provider_label(provider);
                        Found::Failed(problem_text(&problem, provider, &model))
                    }
                };
                // A key that listed the models works: keep it.
                if let (Some(key), Found::Models(..)) = (key, &form.found) {
                    form.key_kept = Some(true);
                    form.changing_key = false;
                    form.key.clear();
                    return off_thread(
                        move || {
                            prev_assist::keys::set(
                                keychain_service(),
                                &key_id(provider, None),
                                &key,
                            )
                        },
                        || Err("The keychain stopped.".to_owned()),
                        ModelMessage::Removed,
                    );
                }
            }
            ModelMessage::Typed(name) => form.typed = name,
            ModelMessage::Add(id) => {
                let id = id.trim().to_owned();
                if id.is_empty() || form.adding.is_some() {
                    return Task::none();
                }
                let provider = form.provider;
                let (address, info) = match &form.found {
                    Found::Models(address, models) => (
                        // The usual address is left out, as prev finds it.
                        address
                            .clone()
                            .filter(|address| Some(address.as_str()) != provider.default_address()),
                        models.iter().find(|model| model.id == id).cloned(),
                    ),
                    _ => (form.typed_address(), None),
                };
                let base = format!("{}-{id}", provider_id(provider));
                let mut model_id = base.clone();
                let mut n = 2;
                while self
                    .settings
                    .assistant_models
                    .iter()
                    .any(|model| model.id == model_id)
                {
                    model_id = format!("{base}-{n}");
                    n += 1;
                }
                let model = AssistantModel {
                    id: model_id,
                    provider: provider_id(provider).to_owned(),
                    model: id.clone(),
                    address: address.clone(),
                    context: provider.sets_context().then_some(form.context),
                    name: info
                        .as_ref()
                        .map(|info| info.name.clone())
                        .filter(|name| *name != prev_assist::friendly_name(provider, &id)),
                    vision: info.as_ref().and_then(|info| info.vision),
                };
                form.adding = Some(id);
                form.status = None;
                let tried = model.clone();
                return off_thread(
                    move || {
                        let key = key_of(&tried).map_err(Problem::Other)?;
                        let choice = choice_of(&tried)
                            .ok_or_else(|| Problem::Other("Unknown provider.".to_owned()))?;
                        prev_assist::test(&choice, key.as_deref())
                    },
                    || Err(Problem::Other("The test stopped.".to_owned())),
                    move |result| ModelMessage::Tried(model, result),
                );
            }
            ModelMessage::Tried(model, result) => {
                form.adding = None;
                if let Err(problem) = result {
                    let provider = form.provider;
                    form.status = Some(Err(problem_text(&problem, provider, &model.model)));
                    return Task::none();
                }
                form.status = Some(Ok(prev::fl!(
                    "settings-assistant-added",
                    model = shown_name(&model)
                )));
                form.typed.clear();
                if self.settings.assistant_model.is_none() {
                    self.settings.assistant_model = Some(model.id.clone());
                }
                self.settings.assistant_models.push(model);
                self.save_settings();
            }
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
                let Some(index) = self
                    .settings
                    .assistant_models
                    .iter()
                    .position(|model| model.id == id)
                else {
                    return Task::none();
                };
                let removed = self.settings.assistant_models.remove(index);
                if self.settings.assistant_model.as_deref() == Some(id.as_str()) {
                    self.settings.assistant_model = self
                        .settings
                        .assistant_models
                        .first()
                        .map(|model| model.id.clone());
                }
                self.save_settings();
                // The provider's key stays while another of its models
                // needs it; a key kept for this model alone goes.
                let shared = provider_of(&removed.provider)
                    .map(|provider| (provider, key_id(provider, removed.address.as_deref())));
                let still_used = self.settings.assistant_models.iter().any(|model| {
                    model.provider == removed.provider && model.address == removed.address
                });
                return off_thread(
                    move || {
                        let service = keychain_service();
                        prev_assist::keys::delete(service, &removed.id)?;
                        match shared {
                            Some((_, key)) if !still_used => {
                                prev_assist::keys::delete(service, &key)
                            }
                            _ => Ok(()),
                        }
                    },
                    || Err("Removing the key stopped.".to_owned()),
                    ModelMessage::Removed,
                );
            }
            ModelMessage::OpenLink(url) => {
                return Task::perform(prev::portal::open_uri(url.to_owned()), |_| Message::Nothing);
            }
        }
        Task::none()
    }
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
