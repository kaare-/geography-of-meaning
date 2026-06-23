# Sensors

> **Guiding question:** How does reality become traces?

## Status

**Partial** — all 15 sensor channels implemented with neighborhood sampling and Gaussian noise. Active concepts and spreading activation are planned ([07_concepts.md](07_concepts.md), [08_prediction.md](08_prediction.md)).

## Summary

Creatures are almost blind, termite-like organisms. They do not perceive objects — they perceive **noisy traces** of local physical state. Mountains, rivers, caves, shelters, and organic patches are never transmitted as named categories; they appear only as patterns across sensor channels (researcher labels applied during analysis).

## Core Principle

**Creatures never access world variables directly.** All world contact passes through `read_sensors()` in `sim-core/src/creatures/sensors.rs`, which samples the 3×3×3 neighborhood and returns a `SensorState`.

## Design Philosophy

Sensors produce **gradients and intensities**, not classifications. A creature might learn that a particular chemical-thermal-contact pattern precedes energy gain — without ever possessing a "food" concept.

## Sensor Categories

| Category | Channels |
|----------|----------|
| Light | `light` |
| Thermal | `thermal` |
| Chemical | `chemical_organic`, `chemical_wet_mineral`, `chemical_decay`, `chemical_binder`, `chemical_creature` |
| Sound | `sound_ambient`, `sound_calls` |
| Contact | `contact_hard`, `contact_soft`, `contact_occupied` |
| Internal | `internal_energy`, `internal_temperature_stress`, `internal_hydration` |

Note: `integrity` and `fatigue` are regulatory variables not yet exposed as dedicated internal sensor channels in the skeleton (partial).

## Light

Derived from local `void_fraction` and solid occlusion in neighborhood. High values = more open sky / less surrounding solid.

## Chemical

Finite-difference style aggregation over neighborhood:

- **organic** — organic fraction
- **wet_mineral** — clay + soft mineral
- **decay** — organic × humidity proxy
- **binder** — binder fraction
- **creature** — other organisms (0.0 in skeleton)

## Sound

- **ambient** — low-level noise floor
- **calls** — conspecific signals (0.0 in skeleton; see [10_communication.md](10_communication.md))

## Thermal

Gradient of temperature across neighborhood relative to center voxel.

## Contact

- **hard** — max hard mineral × solid fraction in neighborhood
- **soft** — max soft mineral × solid fraction
- **occupied** — other creatures present (0.0 in skeleton)

## Internal Sensors

Direct mapping from `RegulatoryState`:

- `internal_energy` ← energy
- `internal_hydration` ← hydration
- `internal_temperature_stress` ← regulatory stress + local temperature deviation

## Sensor Noise

Gaussian noise scaled by `genome.sensor_noise_scale`. Box-Muller transform in `gaussian_noise()`.

## Sensor Range

Hierarchy (skeleton implements local neighborhood only):

1. **Contact** — immediate adjacency (implemented)
2. **Chemical / thermal** — short-range gradients (3×3×3)
3. **Light** — line-of-sight aggregate (simplified)
4. **Sound** — medium range (planned)

## Sensor Signatures

Each creature has a `signature` (u64) for future identification of conspecific sound/chemical traces.

## Active Concepts

**Planned** ([07_concepts.md](07_concepts.md)) — compressed memory nodes that modulate which sensor dimensions are attended.

## Spreading Activation

**Planned** ([08_prediction.md](08_prediction.md)) — memory activation biases sensor interpretation toward expected patterns.

## Sensor Grounding

All sensor values are grounded in simulated physics state, never in semantic labels. The mapping from voxel fields to channels is explicit and local.

## Key Principle

> Reality becomes traces. Traces become experience. Experience becomes memory. Meaning emerges when memory improves prediction.

## Current implementation

`SensorState` — 15 `f32` channels, `as_vector()` for similarity, `read_sensors(creature, world, rng)`.

## Planned

- Creature chemical and contact_occupied from nearby creatures
- Sound propagation from actions and calls
- Integrity/fatigue internal channels
- Active concept modulation
- Longer-range chemical and sound gradients

## Open questions

- Per-channel noise correlation vs independent noise?
- Should light account for diurnal climate curve?

---

## Environmental Sound and Action Sound

> **Addendum** — acoustic traces as sensor evidence; incidental action sounds planned.

### Core Principle

> Sound is **evidence**, not message.

Creatures never receive decoded meanings like "food nearby" or "creature approaching." They receive **acoustic traces** — scalar intensities and rhythms on `sound_ambient` and `sound_calls` — that enter experience like any other sensor channel. What a researcher later labels as feeding noise, construction, or a conspecific call is an emergent interpretation of trace→outcome pairings in memory, not a built-in category in cognition.

### Action-Generated Sound

Physical actions disturb the environment and produce sound as a side effect of work, not as communication:

| Action class | Acoustic character (researcher label) |
|--------------|--------------------------------------|
| Movement | Footstep rhythm, scrape on contact material |
| Dig | Impulsive strikes, debris scatter |
| Carry / drop | Load shift, impact on placement |
| Push | Collision, displacement thud |
| Eat | Chewing / grinding on organic contact |
| Place / binder | Construction taps, adhesion work |
| Rest | Low-amplitude settling, breathing proxy |
| Vocalization (`EmitSound`) | Intentional call — see [10_communication.md](10_communication.md) |

**Implemented:** each resolving action emits low-amplitude `SoundEvent`s into `World::active_sounds`; sensors aggregate them into `sound_ambient` (environmental + incidental) and `sound_calls` (conspecific intentional emissions). No action opcode carries a semantic sound label.

### Material Coupling

Local voxel fields shape how the same action sounds — without exposing material **names** to creatures:

| Material trace (voxel field) | Acoustic signature (researcher label) |
|------------------------------|---------------------------------------|
| Hard mineral | Sharp, high-frequency impulse |
| Soft mineral / clay | Muffled thud |
| Wet / high water content | Splash, dampened resonance |
| Organic | Soft crunch, decay rustle |
| Void / low solid fraction | Hollow echo, less attenuation |

**Implemented:** sensor read derives acoustic timbre from neighborhood fractions (`hard_mineral`, `soft_mineral`, `clay`, `organic`, `void_fraction`, moisture) as continuous scalars — creatures see only the resulting trace vector, never `"granite"` or `"mud"`.

### Morphology Coupling

Body parameters modulate incidental sound production:

| Morphology field | Sound effect |
|------------------|--------------|
| `mass` | Higher impact amplitude on movement and drop |
| `move_speed` (genome) | Faster cadence, sharper rhythm |
| `carried_mass` | Heavier footfalls, louder placement |

**Implemented:** `Morphology` (`creatures/morphology.rs`) — `mass`, `carry_capacity`, `heat_retention`, `reserve_capacity` derived from genome at spawn and inherited on reproduction. Incidental sound emission scales amplitude and rhythm via `SoundEmitterContext`.

### Acoustic Ecology

The soundscape mixes natural and organism-generated traces:

| Source | Channel bias | Status |
|--------|--------------|--------|
| Water flow, rain | `sound_ambient` | Implemented (`emit_environmental_sound`) |
| Collapse / erosion | `sound_ambient` | Implemented (collapse in `physics.rs` → ambient) |
| Movement, digging, feeding | `sound_ambient` | Implemented (incidental `SoundEvent`s) |
| Construction (place, binder) | `sound_ambient` | Partial (place emits; binder planned) |
| Intentional calls | `sound_calls` | Partial (`EmitSound` → `SoundEvent`) |

Acoustic ecology is **competitive attention** — loud natural events can mask or contextualize social signals. Listeners learn which trace combinations precede regulatory outcomes, not which "source type" spoke.

### Learning Through Sound

Repeated pairings build predictive structure:

- movement rhythm + rising `chemical_organic` → energy gain (researcher label: *food discovery via follower*)
- digging sound + exposed organic trace → foraging opportunity
- construction rhythm + shelter-like thermal/light pattern → aggregated rest benefit

These are **memory edges** (`SoundActivates`, `precedes`, `action_leads_to`) — never predefined labels in creature code. Meaning emerges when sound traces change expectation about future regulation ([00_project_overview.md](00_project_overview.md)).

### Current implementation (sound addendum)

| Component | Location | Status |
|-----------|----------|--------|
| `sound_ambient`, `sound_calls` | `creatures/sensors.rs` | Incidental → ambient; intentional → calls |
| `EmitSound` → `SoundEvent` | `creatures/actions.rs`, `world/sound.rs` | Intentional vocalization (`intentional: true`) |
| `emit_incidental_sound` | `world/sound.rs` | Move, dig, carry, drop, push, consume, place |
| `intentional` flag | `world/sound.rs`, `export/logs.rs` | Distinguishes calls from action noise |
| `sample_material_acoustics` | `world/sound.rs` | Voxel hard/soft/wet/organic/void → timbre |
| Morphology sound coupling | `world/sound.rs` | Mass, speed, carried_mass scale amplitude/rhythm |
| Age-scaled signatures | `world/sound.rs` | `signature_with_age_band`, `age_adjusted_vocal_profile` |
| `Morphology` | `creatures/morphology.rs` | Implemented; coupled via emitter context |
| `emit_environmental_sound` | `world/sound.rs`, `world/mod.rs`, `engine.rs` | Rain, water-flow, collapse — `intentional: false`, low amplitude |
| `apply_ambient_processing_cost` | `regulation.rs`, `engine.rs` | Minimal energy drain when `sound_ambient` > 0.25 |

### Cross-references (sound addendum)

| Topic | Doc |
|-------|-----|
| Intentional vs incidental signals | [10_communication.md](10_communication.md) § Communication and Incidental Signals |
| Signal evolution & families | [28_language_and_signal_evolution.md](28_language_and_signal_evolution.md) |
| Social following | [11_social_systems.md](11_social_systems.md) |
