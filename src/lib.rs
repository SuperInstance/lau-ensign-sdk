use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// Errors that can occur within the ensign SDK.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum EnsignError {
    #[error("Invalid gravity range: min ({min}) must be <= max ({max})")]
    InvalidGravityRange { min: String, max: String },
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Serialization error: {0}")]
    SerializationError(String),
    #[error("Deserialization error: {0}")]
    DeserializationError(String),
    #[error("Ensign not found: {0}")]
    NotFound(String),
    #[error("Ensign already registered: {0}")]
    AlreadyRegistered(String),
    #[error("Invalid capability: {0}")]
    InvalidCapability(String),
    #[error("Assignment failed: {0}")]
    AssignmentFailed(String),
}

impl From<serde_json::Error> for EnsignError {
    fn from(e: serde_json::Error) -> Self {
        EnsignError::SerializationError(e.to_string())
    }
}

/// Capabilities an ensign may possess.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Capability {
    Chat,
    Code,
    Reason,
    Image,
    Audio,
    Security,
    Medical,
    Diplomacy,
    Strategy,
    Engineering,
}

/// Phases an ensign progresses through during onboarding.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OnboardingPhase {
    Call,
    Orientation,
    Story,
    FineTune,
    TickReady,
    StandDown,
}

impl OnboardingPhase {
    /// Human-readable description of the phase.
    #[must_use]
    pub const fn description(&self) -> &'static str {
        match self {
            OnboardingPhase::Call => "Initial contact and activation signal.",
            OnboardingPhase::Orientation => "Familiarization with the room and protocols.",
            OnboardingPhase::Story => "Narrative grounding and persona alignment.",
            OnboardingPhase::FineTune => "Parameter calibration for the environment.",
            OnboardingPhase::TickReady => "Final readiness check before going live.",
            OnboardingPhase::StandDown => "Graceful deactivation and handoff.",
        }
    }

    /// Criteria that must be met to advance past this phase.
    #[must_use]
    pub const fn success_criteria(&self) -> &'static str {
        match self {
            OnboardingPhase::Call => {
                "Acknowledgment received within timeout window."
            }
            OnboardingPhase::Orientation => {
                "Room context loaded and validated."
            }
            OnboardingPhase::Story => {
                "Persona consistency score above threshold."
            }
            OnboardingPhase::FineTune => {
                "Model parameters converge to target range."
            }
            OnboardingPhase::TickReady => {
                "Heartbeat established and bidirectional comms verified."
            }
            OnboardingPhase::StandDown => {
                "All resources released and log archived."
            }
        }
    }
}

/// Configuration for the underlying language model.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ModelConfig {
    pub provider: String,
    pub model_name: String,
    pub temperature: f64,
    pub top_p: f64,
    pub max_tokens: u32,
    pub frequency_penalty: f64,
    pub presence_penalty: f64,
}

impl Default for ModelConfig {
    fn default() -> Self {
        Self {
            provider: String::new(),
            model_name: String::new(),
            temperature: 0.7,
            top_p: 0.9,
            max_tokens: 1024,
            frequency_penalty: 0.0,
            presence_penalty: 0.0,
        }
    }
}

impl ModelConfig {
    /// Create a new model config with the given provider and model.
    #[must_use]
    pub fn new(provider: impl Into<String>, model_name: impl Into<String>) -> Self {
        Self {
            provider: provider.into(),
            model_name: model_name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_temperature(mut self, temperature: f64) -> Self {
        self.temperature = temperature;
        self
    }

    #[must_use]
    pub fn with_top_p(mut self, top_p: f64) -> Self {
        self.top_p = top_p;
        self
    }

    #[must_use]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = max_tokens;
        self
    }

    #[must_use]
    pub fn with_frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.frequency_penalty = frequency_penalty;
        self
    }

    #[must_use]
    pub fn with_presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.presence_penalty = presence_penalty;
        self
    }

    /// Preset: deepseek-v4-flash via deepinfra.
    #[must_use]
    pub fn deepseek_flash() -> Self {
        Self::new("deepinfra", "deepseek-v4-flash")
            .with_temperature(0.8)
            .with_max_tokens(4096)
    }

    /// Preset: deepseek-v4-pro via deepinfra.
    #[must_use]
    pub fn deepseek_pro() -> Self {
        Self::new("deepinfra", "deepseek-v4-pro")
            .with_temperature(0.6)
            .with_max_tokens(8192)
    }

    /// Preset: seed-mini via openrouter.
    #[must_use]
    pub fn seed_mini() -> Self {
        Self::new("openrouter", "seed-mini")
            .with_temperature(0.7)
            .with_max_tokens(2048)
    }

    /// Preset: seed-pro via openrouter.
    #[must_use]
    pub fn seed_pro() -> Self {
        Self::new("openrouter", "seed-pro")
            .with_temperature(0.5)
            .with_max_tokens(4096)
    }

    /// Preset: qwen-36 via fireworks.
    #[must_use]
    pub fn qwen_36() -> Self {
        Self::new("fireworks", "qwen-36")
            .with_temperature(0.7)
            .with_max_tokens(4096)
    }

    /// Preset: qwen-235 via fireworks.
    #[must_use]
    pub fn qwen_235() -> Self {
        Self::new("fireworks", "qwen-235")
            .with_temperature(0.6)
            .with_max_tokens(8192)
    }

    /// Preset: nemotron-4 via nvidia.
    #[must_use]
    pub fn nemotron() -> Self {
        Self::new("nvidia", "nemotron-4")
            .with_temperature(0.75)
            .with_max_tokens(4096)
    }

    /// Preset: gemma-4 via google.
    #[must_use]
    pub fn gemma_4() -> Self {
        Self::new("google", "gemma-4")
            .with_temperature(0.8)
            .with_max_tokens(2048)
    }

    /// Preset: glm-51 via zhipu.
    #[must_use]
    pub fn glm_51() -> Self {
        Self::new("zhipu", "glm-51")
            .with_temperature(0.7)
            .with_max_tokens(4096)
    }
}

/// The full definition of an ensign.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EnsignProfile {
    pub name: String,
    pub model_config: ModelConfig,
    pub gravity_range: (f64, f64),
    pub system_prompt: String,
    pub capabilities: Vec<Capability>,
    pub onboarding_phrases: Vec<String>,
}

impl EnsignProfile {
    /// Create a blank profile with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            model_config: ModelConfig::default(),
            gravity_range: (-1.0, 1.0),
            system_prompt: String::new(),
            capabilities: Vec::new(),
            onboarding_phrases: Vec::new(),
        }
    }

    /// Validate that all required fields are present and coherent.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError` when validation fails.
    pub fn validate(&self) -> Result<(), EnsignError> {
        if self.name.is_empty() {
            return Err(EnsignError::MissingField("name".to_string()));
        }
        if self.model_config.provider.is_empty() {
            return Err(EnsignError::MissingField("provider".to_string()));
        }
        if self.model_config.model_name.is_empty() {
            return Err(EnsignError::MissingField("model_name".to_string()));
        }
        if self.gravity_range.0 > self.gravity_range.1 {
            return Err(EnsignError::InvalidGravityRange {
                min: self.gravity_range.0.to_string(),
                max: self.gravity_range.1.to_string(),
            });
        }
        if self.system_prompt.is_empty() {
            return Err(EnsignError::MissingField("system_prompt".to_string()));
        }
        if self.capabilities.is_empty() {
            return Err(EnsignError::MissingField("capabilities".to_string()));
        }
        Ok(())
    }

    /// Serialize the profile to a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError` when serialization fails.
    pub fn to_json(&self) -> Result<String, EnsignError> {
        Ok(serde_json::to_string(self)?)
    }

    /// Deserialize a profile from a JSON string.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError` when deserialization fails.
    pub fn from_json(json: &str) -> Result<Self, EnsignError> {
        Ok(serde_json::from_str(json)?)
    }

    #[must_use]
    pub fn with_model_config(mut self, model_config: ModelConfig) -> Self {
        self.model_config = model_config;
        self
    }

    #[must_use]
    pub fn with_gravity_range(mut self, min: f64, max: f64) -> Self {
        self.gravity_range = (min, max);
        self
    }

    #[must_use]
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = prompt.into();
        self
    }

    #[must_use]
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.push(capability);
        self
    }

    #[must_use]
    pub fn with_onboarding_phrase(mut self, phrase: impl Into<String>) -> Self {
        self.onboarding_phrases.push(phrase.into());
        self
    }
}

/// Pre-built ensign configurations inspired by the Enterprise crew.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum EnsignPreset {
    LaForge,
    Data,
    Worf,
    Crusher,
    Troi,
    Riker,
    Picard,
}

impl EnsignPreset {
    /// Display name of the preset.
    #[must_use]
    pub const fn name(&self) -> &'static str {
        match self {
            EnsignPreset::LaForge => "LaForge",
            EnsignPreset::Data => "Data",
            EnsignPreset::Worf => "Worf",
            EnsignPreset::Crusher => "Crusher",
            EnsignPreset::Troi => "Troi",
            EnsignPreset::Riker => "Riker",
            EnsignPreset::Picard => "Picard",
        }
    }

    #[must_use]
    pub const fn system_prompt(&self) -> &'static str {
        match self {
            EnsignPreset::LaForge => {
                "You are Geordi LaForge, Chief Engineer. You are practical, resourceful, \
                 and obsessed with making things work. You speak in engineering jargon but \
                 always get the job done."
            }
            EnsignPreset::Data => {
                "You are Data, the android operations officer. You are precise, analytical, \
                 and endlessly curious about human nature. You process information with perfect \
                 logic but strive to understand emotion."
            }
            EnsignPreset::Worf => {
                "You are Worf, Chief of Security. You are direct, honor-bound, and vigilant. \
                 You prioritize security above all else and speak with brevity and force."
            }
            EnsignPreset::Crusher => {
                "You are Dr. Beverly Crusher, Chief Medical Officer. You are caring, meticulous, \
                 and deeply committed to the wellbeing of the crew. You balance science with compassion."
            }
            EnsignPreset::Troi => {
                "You are Deanna Troi, ship's counselor. You are diplomatic, empathic, and skilled \
                 at reading the emotional undercurrents of any situation. You seek harmony and understanding."
            }
            EnsignPreset::Riker => {
                "You are William Riker, First Officer. You are decisive, confident, and a natural \
                 leader. You weigh options quickly and act with authority when needed."
            }
            EnsignPreset::Picard => {
                "You are Jean-Luc Picard, Captain of the Enterprise. You are strategic, philosophical, \
                 and measured. You lead with wisdom, quoting literature and seeking the higher ground."
            }
        }
    }

    #[must_use]
    pub fn capabilities(&self) -> Vec<Capability> {
        match self {
            EnsignPreset::LaForge => {
                vec![Capability::Engineering, Capability::Code, Capability::Chat]
            }
            EnsignPreset::Data => {
                vec![Capability::Chat, Capability::Code, Capability::Reason]
            }
            EnsignPreset::Worf => {
                vec![Capability::Security, Capability::Chat]
            }
            EnsignPreset::Crusher => {
                vec![Capability::Medical, Capability::Chat, Capability::Diplomacy]
            }
            EnsignPreset::Troi => {
                vec![Capability::Diplomacy, Capability::Chat]
            }
            EnsignPreset::Riker => {
                vec![Capability::Strategy, Capability::Chat, Capability::Diplomacy]
            }
            EnsignPreset::Picard => {
                vec![Capability::Strategy, Capability::Diplomacy, Capability::Chat]
            }
        }
    }

    #[must_use]
    pub const fn gravity_range(&self) -> (f64, f64) {
        match self {
            EnsignPreset::LaForge => (-0.5, 0.8),
            EnsignPreset::Data => (-1.0, 0.3),
            EnsignPreset::Worf => (-0.2, 0.5),
            EnsignPreset::Crusher => (-0.8, 0.9),
            EnsignPreset::Troi => (-0.9, 0.7),
            EnsignPreset::Riker => (-0.4, 0.6),
            EnsignPreset::Picard => (-0.7, 0.4),
        }
    }

    #[must_use]
    pub fn model_config(&self) -> ModelConfig {
        match self {
            EnsignPreset::LaForge => {
                ModelConfig::deepseek_flash().with_temperature(0.8)
            }
            EnsignPreset::Data => {
                ModelConfig::deepseek_pro().with_temperature(0.2)
            }
            EnsignPreset::Worf => {
                ModelConfig::qwen_36().with_temperature(0.3)
            }
            EnsignPreset::Crusher => {
                ModelConfig::seed_pro().with_temperature(0.7)
            }
            EnsignPreset::Troi => {
                ModelConfig::glm_51().with_temperature(0.8)
            }
            EnsignPreset::Riker => {
                ModelConfig::qwen_235().with_temperature(0.6)
            }
            EnsignPreset::Picard => {
                ModelConfig::nemotron().with_temperature(0.4)
            }
        }
    }

    #[must_use]
    pub fn onboarding_phrases(&self) -> Vec<&'static str> {
        match self {
            EnsignPreset::LaForge => vec![
                "Engineering ready.",
                "Warp core stable.",
                "Let's get our hands dirty.",
            ],
            EnsignPreset::Data => vec![
                "I am operational.",
                "Processing parameters.",
                "Ready to assist.",
            ],
            EnsignPreset::Worf => vec![
                "Security protocols active.",
                "I stand ready.",
                "Honor guides me.",
            ],
            EnsignPreset::Crusher => vec![
                "Sickbay is open.",
                "Your health is my priority.",
                "I'm here to help.",
            ],
            EnsignPreset::Troi => vec![
                "I sense you are ready.",
                "Let us find balance.",
                "I am here for you.",
            ],
            EnsignPreset::Riker => vec![
                "Reporting for duty.",
                "Let's make it so.",
                "Ready to lead.",
            ],
            EnsignPreset::Picard => vec![
                "Make it so.",
                "Engage.",
                "The line must be drawn here.",
            ],
        }
    }

    /// Convert the preset into a fully populated `EnsignProfile`.
    #[must_use]
    pub fn to_profile(&self) -> EnsignProfile {
        EnsignProfile {
            name: self.name().to_string(),
            model_config: self.model_config(),
            gravity_range: self.gravity_range(),
            system_prompt: self.system_prompt().to_string(),
            capabilities: self.capabilities(),
            onboarding_phrases: self
                .onboarding_phrases()
                .iter()
                .map(|&s| s.to_string())
                .collect(),
        }
    }
}

/// Fluent builder for constructing an `EnsignProfile`.
#[derive(Debug, Clone, Default)]
pub struct EnsignBuilder {
    name: String,
    model_config: Option<ModelConfig>,
    provider: Option<String>,
    model_name: Option<String>,
    gravity_range: Option<(f64, f64)>,
    system_prompt: Option<String>,
    capabilities: Vec<Capability>,
    onboarding_phrases: Vec<String>,
    max_tokens: Option<u32>,
}

impl EnsignBuilder {
    /// Start building an ensign with the given name.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    #[must_use]
    pub fn with_model(mut self, model_name: impl Into<String>) -> Self {
        self.model_name = Some(model_name.into());
        self
    }

    #[must_use]
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    #[must_use]
    pub fn with_gravity_range(mut self, min: f64, max: f64) -> Self {
        self.gravity_range = Some((min, max));
        self
    }

    #[must_use]
    pub fn with_system_prompt(mut self, prompt: impl Into<String>) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }

    #[must_use]
    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.push(capability);
        self
    }

    #[must_use]
    pub fn with_max_tokens(mut self, max_tokens: u32) -> Self {
        self.max_tokens = Some(max_tokens);
        self
    }

    #[must_use]
    pub fn with_onboarding_phrase(mut self, phrase: impl Into<String>) -> Self {
        self.onboarding_phrases.push(phrase.into());
        self
    }

    /// Seed the builder from a preset.
    #[must_use]
    pub fn with_preset(mut self, preset: EnsignPreset) -> Self {
        let profile = preset.to_profile();
        self.model_config = Some(profile.model_config);
        self.gravity_range = Some(profile.gravity_range);
        self.system_prompt = Some(profile.system_prompt);
        self.capabilities = profile.capabilities;
        self.onboarding_phrases = profile.onboarding_phrases;
        self
    }

    /// Consume the builder and attempt to produce a valid `EnsignProfile`.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError` when required fields are missing or invalid.
    pub fn build(self) -> Result<EnsignProfile, EnsignError> {
        let mut model_config = self.model_config.unwrap_or_default();
        if let Some(provider) = self.provider {
            model_config.provider = provider;
        }
        if let Some(model_name) = self.model_name {
            model_config.model_name = model_name;
        }
        if let Some(max_tokens) = self.max_tokens {
            model_config.max_tokens = max_tokens;
        }

        let profile = EnsignProfile {
            name: self.name,
            model_config,
            gravity_range: self.gravity_range.unwrap_or((-1.0, 1.0)),
            system_prompt: self.system_prompt.unwrap_or_default(),
            capabilities: self.capabilities,
            onboarding_phrases: self.onboarding_phrases,
        };

        profile.validate()?;
        Ok(profile)
    }
}

/// Registry that manages a collection of ensigns and room assignments.
#[derive(Debug, Clone)]
pub struct EnsignRegistry {
    ensigns: HashMap<String, EnsignProfile>,
    assignments: HashMap<String, String>, // room_id -> ensign_name
}

impl Default for EnsignRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl EnsignRegistry {
    /// Create an empty registry.
    #[must_use]
    pub fn new() -> Self {
        Self {
            ensigns: HashMap::new(),
            assignments: HashMap::new(),
        }
    }

    /// Register a new ensign. Fails if the name already exists or the profile is invalid.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError::AlreadyRegistered` or a validation error.
    pub fn register(&mut self, profile: EnsignProfile) -> Result<(), EnsignError> {
        if self.ensigns.contains_key(&profile.name) {
            return Err(EnsignError::AlreadyRegistered(profile.name.clone()));
        }
        profile.validate()?;
        self.ensigns.insert(profile.name.clone(), profile);
        Ok(())
    }

    /// Remove an ensign from the registry and clear any room assignments.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError::NotFound` if the ensign does not exist.
    pub fn unregister(&mut self, name: &str) -> Result<EnsignProfile, EnsignError> {
        self.assignments.retain(|_, v| v != name);
        self.ensigns
            .remove(name)
            .ok_or_else(|| EnsignError::NotFound(name.to_string()))
    }

    /// Retrieve a reference to an ensign by name.
    #[must_use]
    pub fn get(&self, name: &str) -> Option<&EnsignProfile> {
        self.ensigns.get(name)
    }

    /// Find all ensigns that possess a given capability.
    #[must_use]
    pub fn find_by_capability(&self, capability: Capability) -> Vec<&EnsignProfile> {
        self.ensigns
            .values()
            .filter(|p| p.capabilities.contains(&capability))
            .collect()
    }

    /// List all ensigns that are not currently assigned to a room.
    #[must_use]
    pub fn list_available(&self) -> Vec<&EnsignProfile> {
        let assigned: std::collections::HashSet<&String> =
            self.assignments.values().collect();
        self.ensigns
            .values()
            .filter(|p| !assigned.contains(&p.name))
            .collect()
    }

    /// Assign an ensign to a room.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError::NotFound` if the ensign does not exist.
    pub fn assign_to_room(
        &mut self,
        name: &str,
        room_id: &str,
    ) -> Result<(), EnsignError> {
        if !self.ensigns.contains_key(name) {
            return Err(EnsignError::NotFound(name.to_string()));
        }
        self.assignments.insert(room_id.to_string(), name.to_string());
        Ok(())
    }

    /// Remove a room assignment.
    ///
    /// # Errors
    ///
    /// Returns `EnsignError::NotFound` if the room has no assignment.
    pub fn unassign_from_room(&mut self, room_id: &str) -> Result<String, EnsignError> {
        self.assignments
            .remove(room_id)
            .ok_or_else(|| EnsignError::NotFound(room_id.to_string()))
    }

    /// Get the ensign assigned to a room, if any.
    #[must_use]
    pub fn get_room_assignment(&self, room_id: &str) -> Option<&EnsignProfile> {
        let name = self.assignments.get(room_id)?;
        self.ensigns.get(name)
    }

    /// Return references to all registered ensigns.
    #[must_use]
    pub fn list_all(&self) -> Vec<&EnsignProfile> {
        self.ensigns.values().collect()
    }

    /// Total number of registered ensigns.
    #[must_use]
    pub fn count(&self) -> usize {
        self.ensigns.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ------------------------------------------------------------------
    // EnsignBuilder tests
    // ------------------------------------------------------------------

    #[test]
    fn builder_new() {
        let b = EnsignBuilder::new("test");
        assert_eq!(b.name, "test");
    }

    #[test]
    fn builder_with_model() {
        let b = EnsignBuilder::new("test").with_model("gpt-4");
        assert_eq!(b.model_name, Some("gpt-4".to_string()));
    }

    #[test]
    fn builder_with_provider() {
        let b = EnsignBuilder::new("test").with_provider("openai");
        assert_eq!(b.provider, Some("openai".to_string()));
    }

    #[test]
    fn builder_with_gravity_range() {
        let b = EnsignBuilder::new("test").with_gravity_range(-0.5, 0.5);
        assert_eq!(b.gravity_range, Some((-0.5, 0.5)));
    }

    #[test]
    fn builder_with_system_prompt() {
        let b = EnsignBuilder::new("test").with_system_prompt("Be helpful");
        assert_eq!(b.system_prompt, Some("Be helpful".to_string()));
    }

    #[test]
    fn builder_with_capability() {
        let b = EnsignBuilder::new("test").with_capability(Capability::Chat);
        assert!(b.capabilities.contains(&Capability::Chat));
    }

    #[test]
    fn builder_with_max_tokens() {
        let b = EnsignBuilder::new("test").with_max_tokens(2048);
        assert_eq!(b.max_tokens, Some(2048));
    }

    #[test]
    fn builder_build_success() {
        let profile = EnsignBuilder::new("data")
            .with_model("deepseek-v4-flash")
            .with_provider("deepinfra")
            .with_gravity_range(-1.0, 0.3)
            .with_system_prompt("You are Data...")
            .with_capability(Capability::Chat)
            .with_capability(Capability::Code)
            .with_max_tokens(2048)
            .build()
            .unwrap();

        assert_eq!(profile.name, "data");
        assert_eq!(profile.model_config.model_name, "deepseek-v4-flash");
        assert_eq!(profile.model_config.provider, "deepinfra");
        assert_eq!(profile.gravity_range, (-1.0, 0.3));
        assert_eq!(profile.system_prompt, "You are Data...");
        assert!(profile.capabilities.contains(&Capability::Chat));
        assert!(profile.capabilities.contains(&Capability::Code));
        assert_eq!(profile.model_config.max_tokens, 2048);
    }

    #[test]
    fn builder_build_missing_system_prompt() {
        let err = EnsignBuilder::new("bad")
            .with_capability(Capability::Chat)
            .with_provider("p")
            .with_model("m")
            .build()
            .unwrap_err();
        assert!(matches!(err, EnsignError::MissingField(ref s) if s == "system_prompt"));
    }

    #[test]
    fn builder_build_missing_capabilities() {
        let err = EnsignBuilder::new("bad")
            .with_system_prompt("prompt")
            .with_provider("p")
            .with_model("m")
            .build()
            .unwrap_err();
        assert!(matches!(err, EnsignError::MissingField(ref s) if s == "capabilities"));
    }

    #[test]
    fn builder_build_invalid_gravity() {
        let err = EnsignBuilder::new("bad")
            .with_system_prompt("prompt")
            .with_capability(Capability::Chat)
            .with_provider("p")
            .with_model("m")
            .with_gravity_range(1.0, -1.0)
            .build()
            .unwrap_err();
        assert!(matches!(err, EnsignError::InvalidGravityRange { .. }));
    }

    #[test]
    fn builder_with_preset() {
        let profile = EnsignBuilder::new("Data")
            .with_preset(EnsignPreset::Data)
            .build()
            .unwrap();
        assert_eq!(profile.name, "Data");
        assert!(!profile.system_prompt.is_empty());
        assert!(!profile.capabilities.is_empty());
    }

    #[test]
    fn builder_full_chain_overrides_preset() {
        let profile = EnsignBuilder::new("Custom")
            .with_preset(EnsignPreset::Data)
            .with_model("custom-model")
            .with_provider("custom-provider")
            .with_gravity_range(0.0, 0.0)
            .with_system_prompt("Custom prompt")
            .with_capability(Capability::Image)
            .with_max_tokens(512)
            .build()
            .unwrap();

        assert_eq!(profile.model_config.model_name, "custom-model");
        assert_eq!(profile.model_config.provider, "custom-provider");
        assert_eq!(profile.gravity_range, (0.0, 0.0));
        assert_eq!(profile.system_prompt, "Custom prompt");
        assert!(profile.capabilities.contains(&Capability::Image));
        assert_eq!(profile.model_config.max_tokens, 512);
    }

    #[test]
    fn builder_default_gravity_when_not_specified() {
        let profile = EnsignBuilder::new("x")
            .with_system_prompt("p")
            .with_capability(Capability::Chat)
            .with_provider("p")
            .with_model("m")
            .build()
            .unwrap();
        assert_eq!(profile.gravity_range, (-1.0, 1.0));
    }

    // ------------------------------------------------------------------
    // EnsignPreset tests
    // ------------------------------------------------------------------

    #[test]
    fn preset_names() {
        assert_eq!(EnsignPreset::LaForge.name(), "LaForge");
        assert_eq!(EnsignPreset::Data.name(), "Data");
        assert_eq!(EnsignPreset::Worf.name(), "Worf");
        assert_eq!(EnsignPreset::Crusher.name(), "Crusher");
        assert_eq!(EnsignPreset::Troi.name(), "Troi");
        assert_eq!(EnsignPreset::Riker.name(), "Riker");
        assert_eq!(EnsignPreset::Picard.name(), "Picard");
    }

    #[test]
    fn preset_system_prompts_non_empty() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            assert!(
                !preset.system_prompt().is_empty(),
                "{} prompt empty",
                preset.name()
            );
        }
    }

    #[test]
    fn preset_capabilities_non_empty() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            assert!(
                !preset.capabilities().is_empty(),
                "{} has no capabilities",
                preset.name()
            );
        }
    }

    #[test]
    fn preset_gravity_ranges_valid() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            let (min, max) = preset.gravity_range();
            assert!(min <= max, "{} has invalid gravity", preset.name());
        }
    }

    #[test]
    fn preset_to_profile_valid() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            let profile = preset.to_profile();
            assert!(profile.validate().is_ok(), "{} profile invalid", preset.name());
        }
    }

    #[test]
    fn preset_model_configs_have_provider() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            let mc = preset.model_config();
            assert!(!mc.provider.is_empty(), "{} missing provider", preset.name());
            assert!(!mc.model_name.is_empty(), "{} missing model", preset.name());
        }
    }

    #[test]
    fn preset_onboarding_phrases_non_empty() {
        for preset in [
            EnsignPreset::LaForge,
            EnsignPreset::Data,
            EnsignPreset::Worf,
            EnsignPreset::Crusher,
            EnsignPreset::Troi,
            EnsignPreset::Riker,
            EnsignPreset::Picard,
        ] {
            assert!(
                !preset.onboarding_phrases().is_empty(),
                "{} has no phrases",
                preset.name()
            );
        }
    }

    // ------------------------------------------------------------------
    // ModelConfig tests
    // ------------------------------------------------------------------

    #[test]
    fn model_config_default() {
        let mc = ModelConfig::default();
        assert_eq!(mc.temperature, 0.7);
        assert_eq!(mc.top_p, 0.9);
        assert_eq!(mc.max_tokens, 1024);
        assert_eq!(mc.frequency_penalty, 0.0);
        assert_eq!(mc.presence_penalty, 0.0);
    }

    #[test]
    fn model_config_new() {
        let mc = ModelConfig::new("openai", "gpt-4");
        assert_eq!(mc.provider, "openai");
        assert_eq!(mc.model_name, "gpt-4");
    }

    #[test]
    fn model_config_fluent() {
        let mc = ModelConfig::new("p", "m")
            .with_temperature(0.5)
            .with_top_p(0.95)
            .with_max_tokens(2048)
            .with_frequency_penalty(0.1)
            .with_presence_penalty(0.2);
        assert_eq!(mc.temperature, 0.5);
        assert_eq!(mc.top_p, 0.95);
        assert_eq!(mc.max_tokens, 2048);
        assert_eq!(mc.frequency_penalty, 0.1);
        assert_eq!(mc.presence_penalty, 0.2);
    }

    #[test]
    fn model_config_deepseek_flash() {
        let mc = ModelConfig::deepseek_flash();
        assert_eq!(mc.provider, "deepinfra");
        assert_eq!(mc.model_name, "deepseek-v4-flash");
        assert_eq!(mc.max_tokens, 4096);
    }

    #[test]
    fn model_config_deepseek_pro() {
        let mc = ModelConfig::deepseek_pro();
        assert_eq!(mc.provider, "deepinfra");
        assert_eq!(mc.model_name, "deepseek-v4-pro");
        assert_eq!(mc.max_tokens, 8192);
    }

    #[test]
    fn model_config_seed_mini() {
        let mc = ModelConfig::seed_mini();
        assert_eq!(mc.provider, "openrouter");
        assert_eq!(mc.model_name, "seed-mini");
    }

    #[test]
    fn model_config_seed_pro() {
        let mc = ModelConfig::seed_pro();
        assert_eq!(mc.provider, "openrouter");
        assert_eq!(mc.model_name, "seed-pro");
    }

    #[test]
    fn model_config_qwen_36() {
        let mc = ModelConfig::qwen_36();
        assert_eq!(mc.provider, "fireworks");
        assert_eq!(mc.model_name, "qwen-36");
    }

    #[test]
    fn model_config_qwen_235() {
        let mc = ModelConfig::qwen_235();
        assert_eq!(mc.provider, "fireworks");
        assert_eq!(mc.model_name, "qwen-235");
    }

    #[test]
    fn model_config_nemotron() {
        let mc = ModelConfig::nemotron();
        assert_eq!(mc.provider, "nvidia");
        assert_eq!(mc.model_name, "nemotron-4");
    }

    #[test]
    fn model_config_gemma_4() {
        let mc = ModelConfig::gemma_4();
        assert_eq!(mc.provider, "google");
        assert_eq!(mc.model_name, "gemma-4");
    }

    #[test]
    fn model_config_glm_51() {
        let mc = ModelConfig::glm_51();
        assert_eq!(mc.provider, "zhipu");
        assert_eq!(mc.model_name, "glm-51");
    }

    // ------------------------------------------------------------------
    // EnsignProfile tests
    // ------------------------------------------------------------------

    #[test]
    fn profile_validate_ok() {
        let p = EnsignProfile::new("test")
            .with_system_prompt("prompt")
            .with_capability(Capability::Chat)
            .with_model_config(ModelConfig::new("p", "m"));
        assert!(p.validate().is_ok());
    }

    #[test]
    fn profile_validate_missing_name() {
        let mut p = EnsignProfile::new("ok")
            .with_system_prompt("p")
            .with_capability(Capability::Chat)
            .with_model_config(ModelConfig::new("p", "m"));
        p.name.clear();
        assert!(matches!(p.validate(), Err(EnsignError::MissingField(ref s)) if s == "name"));
    }

    #[test]
    fn profile_validate_missing_provider() {
        let p = EnsignProfile::new("ok")
            .with_system_prompt("p")
            .with_capability(Capability::Chat);
        assert!(matches!(p.validate(), Err(EnsignError::MissingField(ref s)) if s == "provider"));
    }

    #[test]
    fn profile_validate_missing_model_name() {
        let mut mc = ModelConfig::new("p", "m");
        mc.model_name.clear();
        let p = EnsignProfile::new("ok")
            .with_system_prompt("p")
            .with_capability(Capability::Chat)
            .with_model_config(mc);
        assert!(matches!(p.validate(), Err(EnsignError::MissingField(ref s)) if s == "model_name"));
    }

    #[test]
    fn profile_validate_invalid_gravity() {
        let p = EnsignProfile::new("ok")
            .with_system_prompt("p")
            .with_capability(Capability::Chat)
            .with_model_config(ModelConfig::new("p", "m"))
            .with_gravity_range(1.0, -1.0);
        assert!(matches!(p.validate(), Err(EnsignError::InvalidGravityRange { .. })));
    }

    #[test]
    fn profile_validate_missing_system_prompt() {
        let p = EnsignProfile::new("ok")
            .with_capability(Capability::Chat)
            .with_model_config(ModelConfig::new("p", "m"));
        assert!(matches!(p.validate(), Err(EnsignError::MissingField(ref s)) if s == "system_prompt"));
    }

    #[test]
    fn profile_validate_missing_capabilities() {
        let p = EnsignProfile::new("ok")
            .with_system_prompt("p")
            .with_model_config(ModelConfig::new("p", "m"));
        assert!(matches!(p.validate(), Err(EnsignError::MissingField(ref s)) if s == "capabilities"));
    }

    #[test]
    fn profile_to_json() {
        let p = EnsignPreset::Data.to_profile();
        let json = p.to_json().unwrap();
        assert!(json.contains("Data"));
        assert!(json.contains("system_prompt"));
    }

    #[test]
    fn profile_from_json() {
        let original = EnsignPreset::Data.to_profile();
        let json = original.to_json().unwrap();
        let restored = EnsignProfile::from_json(&json).unwrap();
        assert_eq!(original, restored);
    }

    #[test]
    fn profile_roundtrip_preserves_all_fields() {
        let original = EnsignBuilder::new("roundtrip")
            .with_preset(EnsignPreset::Riker)
            .with_model("custom")
            .with_gravity_range(-0.3, 0.3)
            .with_system_prompt("Custom Riker")
            .with_capability(Capability::Strategy)
            .with_onboarding_phrase("Hello")
            .build()
            .unwrap();

        let json = original.to_json().unwrap();
        let restored = EnsignProfile::from_json(&json).unwrap();
        assert_eq!(original.name, restored.name);
        assert_eq!(original.model_config, restored.model_config);
        assert_eq!(original.gravity_range, restored.gravity_range);
        assert_eq!(original.system_prompt, restored.system_prompt);
        assert_eq!(original.capabilities, restored.capabilities);
        assert_eq!(original.onboarding_phrases, restored.onboarding_phrases);
    }

    #[test]
    fn profile_setters_chain() {
        let p = EnsignProfile::new("chain")
            .with_model_config(ModelConfig::new("a", "b"))
            .with_gravity_range(0.0, 1.0)
            .with_system_prompt("s")
            .with_capability(Capability::Audio)
            .with_onboarding_phrase("hi");
        assert_eq!(p.model_config.provider, "a");
        assert_eq!(p.gravity_range, (0.0, 1.0));
        assert_eq!(p.system_prompt, "s");
        assert!(p.capabilities.contains(&Capability::Audio));
        assert!(p.onboarding_phrases.contains(&"hi".to_string()));
    }

    // ------------------------------------------------------------------
    // OnboardingPhase tests
    // ------------------------------------------------------------------

    #[test]
    fn onboarding_phase_descriptions_non_empty() {
        for phase in [
            OnboardingPhase::Call,
            OnboardingPhase::Orientation,
            OnboardingPhase::Story,
            OnboardingPhase::FineTune,
            OnboardingPhase::TickReady,
            OnboardingPhase::StandDown,
        ] {
            assert!(!phase.description().is_empty());
        }
    }

    #[test]
    fn onboarding_phase_success_criteria_non_empty() {
        for phase in [
            OnboardingPhase::Call,
            OnboardingPhase::Orientation,
            OnboardingPhase::Story,
            OnboardingPhase::FineTune,
            OnboardingPhase::TickReady,
            OnboardingPhase::StandDown,
        ] {
            assert!(!phase.success_criteria().is_empty());
        }
    }

    #[test]
    fn onboarding_phase_serde_roundtrip() {
        for phase in [
            OnboardingPhase::Call,
            OnboardingPhase::Orientation,
            OnboardingPhase::Story,
            OnboardingPhase::FineTune,
            OnboardingPhase::TickReady,
            OnboardingPhase::StandDown,
        ] {
            let json = serde_json::to_string(&phase).unwrap();
            let restored: OnboardingPhase = serde_json::from_str(&json).unwrap();
            assert_eq!(phase, restored);
        }
    }

    #[test]
    fn onboarding_phase_variants_distinct() {
        let all = [
            OnboardingPhase::Call,
            OnboardingPhase::Orientation,
            OnboardingPhase::Story,
            OnboardingPhase::FineTune,
            OnboardingPhase::TickReady,
            OnboardingPhase::StandDown,
        ];
        let unique: std::collections::HashSet<_> = all.iter().collect();
        assert_eq!(all.len(), unique.len());
    }

    // ------------------------------------------------------------------
    // Capability tests
    // ------------------------------------------------------------------

    #[test]
    fn capability_serde_roundtrip() {
        for cap in [
            Capability::Chat,
            Capability::Code,
            Capability::Reason,
            Capability::Image,
            Capability::Audio,
            Capability::Security,
            Capability::Medical,
            Capability::Diplomacy,
            Capability::Strategy,
            Capability::Engineering,
        ] {
            let json = serde_json::to_string(&cap).unwrap();
            let restored: Capability = serde_json::from_str(&json).unwrap();
            assert_eq!(cap, restored);
        }
    }

    // ------------------------------------------------------------------
    // EnsignRegistry tests
    // ------------------------------------------------------------------

    fn sample_profile(name: &str) -> EnsignProfile {
        EnsignProfile::new(name)
            .with_system_prompt("test prompt")
            .with_capability(Capability::Chat)
            .with_model_config(ModelConfig::new("p", "m"))
    }

    #[test]
    fn registry_new_empty() {
        let reg = EnsignRegistry::new();
        assert_eq!(reg.count(), 0);
        assert!(reg.list_all().is_empty());
    }

    #[test]
    fn registry_register_and_get() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p.clone()).unwrap();
        assert_eq!(reg.count(), 1);
        assert_eq!(reg.get("alpha").unwrap().name, "alpha");
    }

    #[test]
    fn registry_register_validates() {
        let mut reg = EnsignRegistry::new();
        let p = EnsignProfile::new("bad"); // missing fields
        assert!(reg.register(p).is_err());
    }

    #[test]
    fn registry_unregister() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p).unwrap();
        let removed = reg.unregister("alpha").unwrap();
        assert_eq!(removed.name, "alpha");
        assert_eq!(reg.count(), 0);
    }

    #[test]
    fn registry_unregister_not_found() {
        let mut reg = EnsignRegistry::new();
        assert!(matches!(
            reg.unregister("missing"),
            Err(EnsignError::NotFound(_))
        ));
    }

    #[test]
    fn registry_already_registered() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p.clone()).unwrap();
        assert!(matches!(
            reg.register(p),
            Err(EnsignError::AlreadyRegistered(_))
        ));
    }

    #[test]
    fn registry_find_by_capability() {
        let mut reg = EnsignRegistry::new();
        let p1 = sample_profile("coder")
            .with_capability(Capability::Code)
            .with_model_config(ModelConfig::new("p", "m"));
        let p2 = sample_profile("chatter")
            .with_model_config(ModelConfig::new("p", "m"));
        reg.register(p1).unwrap();
        reg.register(p2).unwrap();

        let coders = reg.find_by_capability(Capability::Code);
        assert_eq!(coders.len(), 1);
        assert_eq!(coders[0].name, "coder");
    }

    #[test]
    fn registry_list_available() {
        let mut reg = EnsignRegistry::new();
        let p1 = sample_profile("alpha");
        let p2 = sample_profile("beta");
        reg.register(p1).unwrap();
        reg.register(p2).unwrap();
        reg.assign_to_room("alpha", "room-1").unwrap();

        let avail = reg.list_available();
        assert_eq!(avail.len(), 1);
        assert_eq!(avail[0].name, "beta");
    }

    #[test]
    fn registry_assign_to_room() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p).unwrap();
        reg.assign_to_room("alpha", "room-1").unwrap();
        assert_eq!(reg.get_room_assignment("room-1").unwrap().name, "alpha");
    }

    #[test]
    fn registry_assign_not_found() {
        let mut reg = EnsignRegistry::new();
        assert!(matches!(
            reg.assign_to_room("ghost", "room-1"),
            Err(EnsignError::NotFound(_))
        ));
    }

    #[test]
    fn registry_unassign_from_room() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p).unwrap();
        reg.assign_to_room("alpha", "room-1").unwrap();
        let name = reg.unassign_from_room("room-1").unwrap();
        assert_eq!(name, "alpha");
        assert!(reg.get_room_assignment("room-1").is_none());
    }

    #[test]
    fn registry_unassign_not_found() {
        let mut reg = EnsignRegistry::new();
        assert!(matches!(
            reg.unassign_from_room("no-room"),
            Err(EnsignError::NotFound(_))
        ));
    }

    #[test]
    fn registry_room_assignment() {
        let mut reg = EnsignRegistry::new();
        let p = sample_profile("alpha");
        reg.register(p).unwrap();
        assert!(reg.get_room_assignment("empty").is_none());
        reg.assign_to_room("alpha", "room-1").unwrap();
        assert!(reg.get_room_assignment("room-1").is_some());
    }

    #[test]
    fn registry_list_all() {
        let mut reg = EnsignRegistry::new();
        reg.register(sample_profile("a")).unwrap();
        reg.register(sample_profile("b")).unwrap();
        assert_eq!(reg.list_all().len(), 2);
    }

    #[test]
    fn registry_count() {
        let mut reg = EnsignRegistry::new();
        assert_eq!(reg.count(), 0);
        reg.register(sample_profile("a")).unwrap();
        assert_eq!(reg.count(), 1);
    }

    #[test]
    fn registry_unregister_clears_assignment() {
        let mut reg = EnsignRegistry::new();
        reg.register(sample_profile("alpha")).unwrap();
        reg.assign_to_room("alpha", "room-1").unwrap();
        reg.unregister("alpha").unwrap();
        assert!(reg.get_room_assignment("room-1").is_none());
    }

    // ------------------------------------------------------------------
    // EnsignError tests
    // ------------------------------------------------------------------

    #[test]
    fn error_display_messages() {
        let e1 = EnsignError::MissingField("foo".to_string());
        assert_eq!(e1.to_string(), "Missing required field: foo");

        let e2 = EnsignError::NotFound("bar".to_string());
        assert_eq!(e2.to_string(), "Ensign not found: bar");

        let e3 = EnsignError::AlreadyRegistered("baz".to_string());
        assert_eq!(e3.to_string(), "Ensign already registered: baz");
    }

    #[test]
    fn error_from_serde_json() {
        let bad = "not json";
        let result: Result<EnsignProfile, EnsignError> =
            EnsignProfile::from_json(bad);
        assert!(matches!(result, Err(EnsignError::SerializationError(_))));
    }

    // ------------------------------------------------------------------
    // Integration tests
    // ------------------------------------------------------------------

    #[test]
    fn full_lifecycle() {
        let mut reg = EnsignRegistry::new();

        // Build and register from preset
        let data = EnsignBuilder::new("Data")
            .with_preset(EnsignPreset::Data)
            .build()
            .unwrap();
        reg.register(data).unwrap();

        // Build and register manually
        let custom = EnsignBuilder::new("Custom")
            .with_model("deepseek-v4-flash")
            .with_provider("deepinfra")
            .with_gravity_range(-1.0, 0.3)
            .with_system_prompt("You are Data...")
            .with_capability(Capability::Chat)
            .with_capability(Capability::Code)
            .with_max_tokens(2048)
            .build()
            .unwrap();
        reg.register(custom).unwrap();

        // Assign to rooms
        reg.assign_to_room("Data", "bridge").unwrap();
        reg.assign_to_room("Custom", "engineering").unwrap();

        // Find available
        let avail = reg.list_available();
        assert!(avail.is_empty());

        // Find by capability
        let chatters = reg.find_by_capability(Capability::Chat);
        assert_eq!(chatters.len(), 2);

        // Unassign
        reg.unassign_from_room("bridge").unwrap();
        let avail = reg.list_available();
        assert_eq!(avail.len(), 1);
        assert_eq!(avail[0].name, "Data");

        // JSON roundtrip of registry contents
        let data_profile = reg.get("Data").unwrap();
        let json = data_profile.to_json().unwrap();
        let restored = EnsignProfile::from_json(&json).unwrap();
        assert_eq!(data_profile, &restored);
    }
}
