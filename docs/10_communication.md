# Communication

> **Guiding question:** How does one model alter another?

## Status

**Stub / Partial** — sensor channels `sound_ambient` and `sound_calls`, `Sound` memory node kind, `sound_activates` edge type, and per-creature `signature` exist. Sound production, sender identity resolution, and listener prediction updates are **planned**.

## Summary

Communication is not information transmission. One predictive system **modifies another's prediction** — altering which memory subgraph activates and what future regulatory states are expected. There are no predefined message meanings, words, or danger/food/shelter categories in creature cognition.

## Core Principle

> Communication modifies prediction, not information.

A listener does not receive facts about the world. It receives **sensor traces** (sound patterns) that activate memory structures, which in turn shift expectations about future internal state. What a researcher might later call "warning," "invitation," or "food location" is an emergent label on a predictive relationship — never a built-in message type.

## Origins

Social prediction precedes symbols. Before language, organisms already benefit from anticipating conspecific behavior: where others move, what traces they leave, what regulatory outcomes follow proximity. Communication grows from this **social prediction** layer.

## Stage 1 — Social Prediction

Observe another organism's sensor-correlated behavior and build `precedes` / `action_leads_to` edges linking their trace patterns to outcomes. No emission required — passive coupling.

**Skeleton:** `chemical_creature` and `contact_occupied` channels are 0.0; social prediction not yet wired.

## Stage 2 — Attention Signals

Low-cost emissions that bias a listener's attention toward a region or pattern — still without semantic content. Rhythmic or intense sound draws processing resources.

**Planned:** `sound_calls` driven by action emission.

## Stage 3 — Predictive Signals

Signals reliably precede outcomes the sender has already learned. The listener compresses repeated sound→outcome pairs into predictive edges. Meaning emerges when a sound **changes expectation** about future regulation.

## Sound System

Sounds are physical events with **frequency, duration, rhythm, and intensity** — not labels.

| Property | Role |
|----------|------|
| Frequency | Timbre / source discrimination |
| Duration | Urgency or persistence |
| Rhythm | Pattern recognition, memorability |
| Intensity | Range and salience |

No predefined meanings attached to any parameter combination.

## Sound Nodes

`NodeKind::Sound(f32)` in `sim-core/src/memory/nodes.rs` stores sound-event intensity. Linked to sensory patterns via `EdgeType::SoundActivates` in `edges.rs`.

**Skeleton:** nodes created only when experiences include sound; production not implemented.

## Sender Identity

Listeners must distinguish **who** produced a sound to maintain separate predictive models per conspecific. Implemented via `Creature::signature` (u64) — future mapping from sound trace to signature.

## Signatures

Each creature carries a unique `signature` at spawn. Future: signature encoded in sound production parameters so listeners build per-sender memory subgraphs.

## Trust

Trust is **not** a separate variable. It emerges from **prediction confidence**: when sender S's signals repeatedly reduce listener L's prediction error about regulation, edges strengthen and L "trusts" S's influence. Low confidence after repeated error weakens trust.

## Deception

Deception is indistinguishable from prediction error at the mechanism level: a signal predicts an outcome that does not occur. `Experience.outcome` (delta energy / regulatory change) records the mismatch; edges weaken or restructure.

## Communication and Memory

Sound events enter the graph as nodes; `sound_activates` edges link them to sensory and outcome patterns. Over time, compressed clusters ([07_concepts.md](07_concepts.md)) may represent recurring call types — still without fixed semantics.

## Communication and Imagination

During sleep or quiet periods ([09_sleep.md](09_sleep.md)), spreading activation may replay sound→outcome chains without live input — proto-narrative rehearsal.

## Storytelling

**Planned** — sequenced sound activation that guides listeners through a predicted trajectory (regulatory and sensory), externalizing one creature's memory subgraph into another's activation. Not language; temporal pattern transfer.

## Knowledge Transfer

Indirect: modified predictions reshape action choices, which reshape experiences, which reshape memory. No "teaching" opcode.

## Cultural Drift

Populations diverge when sound→outcome associations differ by lineage and geography. See [16_culture.md](16_culture.md).

## Language

Language is **emergent**, not assumed. Symbolic mapping arises only if compressed concepts stabilize across generations and communication — a late-phase research target, not a skeleton feature. Full treatment: signal space, drift, dialects, proto-syntax, and language emergence ([28_language_and_signal_evolution.md](28_language_and_signal_evolution.md)).

## Communication and Meaning

A sound becomes meaningful when it **changes expectations about future regulatory state**. Same definition as project-wide meaning ([00_project_overview.md](00_project_overview.md)).

## Core Principle

> One model alters another by shifting its predictions. Traces in, prediction change out — never propositions.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `sound_ambient`, `sound_calls` | `creatures/sensors.rs` | Read from world `active_sounds` |
| `SoundEvent`, `World::active_sounds` | `world/sound.rs`, `world/mod.rs` | Implemented |
| `Action::EmitSound` | `creatures/actions.rs` | Uses `genome.vocal_profile`; energy cost; biased when energy high |
| `Action::Follow` | `creatures/actions.rs`, `spatial.rs` | Biased by `chemical_creature` / `sound_calls`; engine resolves direction toward strongest neighbor gradient |
| `NodeKind::Sound` (`intensity`, `signature`), `record_heard_sound` | `memory/graph.rs` | Sound nodes with emitter signature on hear |
| `predict_action_outcomes` sound paths | `memory/graph.rs` | `SoundActivates` chains weighted by edge confidence and per-signature outcome boost |
| `trusted_follow_boost`, `trusted_signature_count` | `memory/graph.rs`, `export/snapshots.rs` | Follow bias when calls salient + positive sound→outcome for signature |
| `dominant_heard_signature` | `creatures/sensors.rs` | Resolves strongest non-self emitter per tick |
| `EdgeType::SoundActivates` | `memory/edges.rs` | Used on heard experiences |
| `signature` | `creatures/creature.rs` | Assigned at spawn |
| `genome.vocal_profile` | `creatures/genome.rs` | Pitch, duration, amplitude, rhythm; mutated on reproduction |
| `SoundEvent::signal_family_id` | `world/sound.rs` | Hash of vocal profile for family tracking |
| Tick log sound export | `export/logs.rs` | `sound_event_count` + optional slice with `signal_family_id` |
| Listener signature match | `sensors.rs` `dominant_heard_signature` | Implemented |
| Trust as confidence | `memory/graph.rs` | Per-signature outcome boost in prediction; `trusted_signature_count` in snapshot |

## Planned

- Propagation through world (attenuation, occlusion)
- Per-signature memory subgraphs
- Integration with prediction engine ([08_prediction.md](08_prediction.md))

## Cross-references

| Topic | Doc |
|-------|-----|
| Language & signal evolution | [28_language_and_signal_evolution.md](28_language_and_signal_evolution.md) |
| Memory | [06_memory.md](06_memory.md) |
| Concepts | [07_concepts.md](07_concepts.md) |
| Sleep & rehearsal | [09_sleep.md](09_sleep.md) |
| Culture | [16_culture.md](16_culture.md) |

## Open questions

- Should signatures modulate sound parameters or live in a separate channel?
- Maximum audible range vs chemical trace range?
