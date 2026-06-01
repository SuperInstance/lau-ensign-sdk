# lau-ensign-sdk

**THE ensign development SDK** — how to define, configure, validate, and deploy an ensign (room DJ). Provides a fluent builder, preset profiles, model configuration, capability declarations, and a registry for managing multiple ensigns and their room assignments.

Built on top of [`lau-ensign`](https://github.com/SuperInstance/lau-ensign), this SDK handles the "deployment" side: constructing profiles, wiring up model configs, and managing the fleet.

**68 tests** · `serde` + `serde_json` + `thiserror` · MIT licensed.

---

## Table of Contents

1. [What This Does](#what-this-does)
2. [Key Idea](#key-idea)
3. [Install](#install)
4. [Quick Start](#quick-start)
5. [API Reference](#api-reference)
6. [How It Works](#how-it-works)
7. [The Math](#the-math)
8. [License](#license)

---

## What This Does

| Feature | Detail |
|---|---|
| **EnsignProfile** | Full definition: name, model config, gravity range, system prompt, capabilities, onboarding phrases |
| **EnsignBuilder** | Fluent builder with validation on `.build()` |
| **EnsignPreset** | 7 pre-built profiles inspired by the Enterprise crew (LaForge, Data, Worf, Crusher, Troi, Riker, Picard) |
| **ModelConfig** | Provider, model name, temperature, top_p, max_tokens, penalties — with presets for 9 providers |
| **Capability enum** | Chat, Code, Reason, Image, Audio, Security, Medical, Diplomacy, Strategy, Engineering |
| **OnboardingPhase** | 6-phase onboarding protocol with descriptions and success criteria |
| **EnsignRegistry** | Register/unregister ensigns, assign to rooms, find by capability, list available |
| **Validation** | All profiles validate required fields, gravity range, provider/model existence |
| **JSON round-trips** | Every profile serialises to JSON and restores perfectly |

---

## Key Idea

An ensign is more than a model config — it's a **complete deployment specification**. This SDK captures:

1. **What model** powers it (provider + model + sampling params)
2. **What it can do** (capabilities: chat, code, security, etc.)
3. **How it behaves** (system prompt + gravity range)
4. **How it onboards** (phrases for each phase of the call → stand-down lifecycle)
5. **Where it's deployed** (room assignments via the registry)

The builder pattern ensures you can't create an invalid ensign: `.build()` returns `Result<EnsignProfile, EnsignError>` with specific error variants for every missing or invalid field.

---

## Install

```toml
[dependencies]
lau-ensign-sdk = "0.1"
```

### Dependencies

| Crate | Why |
|---|---|
| `serde` + `serde_json` | Serialisation |
| `thiserror` | Ergonomic error types |

---

## Quick Start

### Build from scratch

```rust
use lau_ensign_sdk::*;

let profile = EnsignBuilder::new("Data")
    .with_provider("deepinfra")
    .with_model("deepseek-v4-flash")
    .with_gravity_range(-1.0, 0.3)
    .with_system_prompt("You are Data, the android operations officer...")
    .with_capability(Capability::Chat)
    .with_capability(Capability::Code)
    .with_max_tokens(2048)
    .with_onboarding_phrase("I am operational.")
    .build()
    .unwrap();
```

### Build from preset

```rust
let profile = EnsignPreset::Data.to_profile();
// Already has provider, model, prompt, capabilities, gravity range, and onboarding phrases
```

### Override a preset

```rust
let profile = EnsignBuilder::new("Data-v2")
    .with_preset(EnsignPreset::Data)          // start from preset
    .with_model("deepseek-v4-pro")             // override model
    .with_gravity_range(-0.5, 0.5)             // tighten gravity
    .with_system_prompt("Custom prompt")       // override prompt
    .build()
    .unwrap();
```

### Register and assign

```rust
let mut registry = EnsignRegistry::new();

registry.register(EnsignPreset::Data.to_profile()).unwrap();
registry.register(EnsignPreset::Worf.to_profile()).unwrap();

registry.assign_to_room("Data", "bridge").unwrap();
registry.assign_to_room("Worf", "security").unwrap();

// Who's available?
let avail = registry.list_available();
assert!(avail.is_empty());

// Find by capability
let chatters = registry.find_by_capability(Capability::Chat);
assert_eq!(chatters.len(), 2);
```

### Serialise

```rust
let json = profile.to_json().unwrap();
let restored = EnsignProfile::from_json(&json).unwrap();
assert_eq!(profile, restored);
```

---

## API Reference

### `EnsignProfile`

The complete definition of an ensign.

| Field | Type | Required |
|---|---|---|
| `name` | `String` | ✅ |
| `model_config` | `ModelConfig` | ✅ (provider + model_name) |
| `gravity_range` | `(f64, f64)` | Default: (-1.0, 1.0) |
| `system_prompt` | `String` | ✅ |
| `capabilities` | `Vec<Capability>` | ✅ (non-empty) |
| `onboarding_phrases` | `Vec<String>` | Optional |

| Method | Returns |
|---|---|
| `new(name)` | Blank profile |
| `validate()` | `Result<(), EnsignError>` |
| `to_json()` | `Result<String, EnsignError>` |
| `from_json(json)` | `Result<Self, EnsignError>` |
| `with_model_config(mc)` | Builder chain |
| `with_gravity_range(min, max)` | Builder chain |
| `with_system_prompt(prompt)` | Builder chain |
| `with_capability(cap)` | Builder chain |
| `with_onboarding_phrase(phrase)` | Builder chain |

### `EnsignBuilder`

Fluent builder that produces a validated `EnsignProfile`.

| Method | Description |
|---|---|
| `new(name)` | Start building |
| `with_provider(provider)` | Set model provider |
| `with_model(model_name)` | Set model name |
| `with_gravity_range(min, max)` | Set gravity bounds |
| `with_system_prompt(prompt)` | Set system prompt |
| `with_capability(cap)` | Add a capability |
| `with_max_tokens(n)` | Override max tokens |
| `with_onboarding_phrase(phrase)` | Add an onboarding phrase |
| `with_preset(preset)` | Seed from a preset (can override after) |
| `build()` | → `Result<EnsignProfile, EnsignError>` |

### `ModelConfig`

| Field | Default |
|---|---|
| `provider` | `""` (must set) |
| `model_name` | `""` (must set) |
| `temperature` | 0.7 |
| `top_p` | 0.9 |
| `max_tokens` | 1024 |
| `frequency_penalty` | 0.0 |
| `presence_penalty` | 0.0 |

**Presets:**

| Method | Provider | Model | Temp | Max Tokens |
|---|---|---|---|---|
| `deepseek_flash()` | deepinfra | deepseek-v4-flash | 0.8 | 4096 |
| `deepseek_pro()` | deepinfra | deepseek-v4-pro | 0.6 | 8192 |
| `seed_mini()` | openrouter | seed-mini | 0.7 | 2048 |
| `seed_pro()` | openrouter | seed-pro | 0.5 | 4096 |
| `qwen_36()` | fireworks | qwen-36 | 0.7 | 4096 |
| `qwen_235()` | fireworks | qwen-235 | 0.6 | 8192 |
| `nemotron()` | nvidia | nemotron-4 | 0.75 | 4096 |
| `gemma_4()` | google | gemma-4 | 0.8 | 2048 |
| `glm_51()` | zhipu | glm-51 | 0.7 | 4096 |

All presets support chaining: `ModelConfig::deepseek_flash().with_temperature(0.5)`.

### `EnsignPreset`

7 Enterprise-crew-inspired presets:

| Preset | Capabilities | Gravity | Model | Temp |
|---|---|---|---|---|
| **LaForge** | Engineering, Code, Chat | (-0.5, 0.8) | deepseek-v4-flash | 0.8 |
| **Data** | Chat, Code, Reason | (-1.0, 0.3) | deepseek-v4-pro | 0.2 |
| **Worf** | Security, Chat | (-0.2, 0.5) | qwen-36 | 0.3 |
| **Crusher** | Medical, Chat, Diplomacy | (-0.8, 0.9) | seed-pro | 0.7 |
| **Troi** | Diplomacy, Chat | (-0.9, 0.7) | glm-51 | 0.8 |
| **Riker** | Strategy, Chat, Diplomacy | (-0.4, 0.6) | qwen-235 | 0.6 |
| **Picard** | Strategy, Diplomacy, Chat | (-0.7, 0.4) | nemotron-4 | 0.4 |

Each preset provides: `.name()`, `.system_prompt()`, `.capabilities()`, `.gravity_range()`, `.model_config()`, `.onboarding_phrases()`, `.to_profile()`.

### `Capability`

```rust
pub enum Capability {
    Chat, Code, Reason, Image, Audio,
    Security, Medical, Diplomacy, Strategy, Engineering,
}
```

### `OnboardingPhase`

```rust
pub enum OnboardingPhase {
    Call, Orientation, Story, FineTune, TickReady, StandDown,
}
```

Each variant has `.description()` and `.success_criteria()`.

### `EnsignRegistry`

| Method | Returns | Description |
|---|---|---|
| `new()` | `Self` | Empty registry |
| `register(profile)` | `Result<(), EnsignError>` | Validate + insert |
| `unregister(name)` | `Result<EnsignProfile, EnsignError>` | Remove + clear assignments |
| `get(name)` | `Option<&EnsignProfile>` | Lookup by name |
| `find_by_capability(cap)` | `Vec<&EnsignProfile>` | All ensigns with that capability |
| `list_available()` | `Vec<&EnsignProfile>` | Not currently assigned to a room |
| `assign_to_room(name, room_id)` | `Result<(), EnsignError>` | Assign ensign to room |
| `unassign_from_room(room_id)` | `Result<String, EnsignError>` | Returns ensign name |
| `get_room_assignment(room_id)` | `Option<&EnsignProfile>` | Who's on this room? |
| `list_all()` | `Vec<&EnsignProfile>` | All registered |
| `count()` | `usize` | Total registered |

### `EnsignError`

```rust
pub enum EnsignError {
    InvalidGravityRange { min: String, max: String },
    MissingField(String),
    SerializationError(String),
    DeserializationError(String),
    NotFound(String),
    AlreadyRegistered(String),
    InvalidCapability(String),
    AssignmentFailed(String),
}
```

Implements `std::error::Error` via `thiserror`. Converts from `serde_json::Error`.

---

## How It Works

### Builder Pattern

`EnsignBuilder` collects optional fields. On `.build()`:

1. Assembles a `ModelConfig` from builder overrides (provider, model, max_tokens) merged onto default.
2. Constructs the `EnsignProfile` from all fields.
3. Calls `profile.validate()`.
4. Returns `Ok(profile)` or `Err(EnsignError)`.

### Preset Seeding

`with_preset(preset)` copies all fields from the preset's `to_profile()` into the builder's optional slots. Any subsequent `with_*` call overrides the preset value. This lets you start from a known-good configuration and tweak it.

### Validation Rules

A valid profile requires:
1. `name` non-empty
2. `model_config.provider` non-empty
3. `model_config.model_name` non-empty
4. `gravity_range.0 ≤ gravity_range.1`
5. `system_prompt` non-empty
6. `capabilities` non-empty

### Registry Assignment Model

The registry maintains two `HashMap`s:
- `ensigns: HashMap<String, EnsignProfile>` — all profiles by name
- `assignments: HashMap<String, String>` — room_id → ensign_name

When an ensign is unregistered, its room assignment is automatically cleared. Multiple rooms can share an ensign (each room gets a separate entry), but `list_available()` filters to ensigns not assigned to **any** room.

### JSON Contract

`EnsignProfile::to_json()` and `from_json()` use serde's default JSON serialisation. The JSON structure mirrors the Rust struct exactly, making it human-readable and editable. A round-trip through JSON preserves every field.

---

## The Math

### Gravity Range as a Control Surface

The gravity range `(min, max)` defines the operational envelope for the ensign's "room gravity" parameter. This is a **normalised control signal** in the range `[-1.0, 1.0]` by default:

```
gravity ∈ [min, max]

min = −1.0  →  fully passive / hands-off
max = +1.0  →  fully active / high intervention
```

Different presets choose different envelopes:
- **Data** (−1.0, 0.3): errs toward passive, only intervenes conservatively
- **Worf** (−0.2, 0.5): narrow range, always somewhat vigilant
- **Crusher** (−0.8, 0.9): wide range, highly adaptable

### Temperature as Stochastic Control

The model temperature parameter controls the randomness of the ensign's outputs:

```
T = 0.2  →  nearly deterministic (Data)
T = 0.8  →  creative/varied (LaForge, Troi)
```

Lower temperature = more predictable, higher = more creative. The presets map personality to temperature: Data is precise (0.2), Troi is empathic and varied (0.8).

### Capability Coverage

Each preset's capabilities form a set. The registry's `find_by_capability` performs a set membership test:

```
match(ensign) = capability ∈ ensign.capabilities
```

For fleet planning, you can check coverage:

```
required = {Chat, Code}
available = registry.find_by_capability(Chat) ∩ registry.find_by_capability(Code)
```

### Onboarding Phase Model

The 6 phases form a strict ordering:

```
Call → Orientation → Story → FineTune → TickReady → StandDown
```

Each phase has a `success_criteria` string defining what must be true to advance. This is a **finite state machine** where transitions are gated by validation.

---

## License

MIT
