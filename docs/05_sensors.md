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
