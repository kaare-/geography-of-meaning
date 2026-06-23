# Sleep

> **Guiding question:** What happens when sensors are quiet?

## Status

**Partial** — `MemoryGraph::consolidate_sleep()` is a no-op stub in `memory/graph.rs`. `Rest` action and fatigue in `RegulatoryState` partially influence recovery; full sleep cycle not implemented.

## Summary

Waking life **gathers** experience; sleep **organizes** it. When sensory input drops and fatigue rises, the organism enters an offline period where memory graphs are pruned, strengthened, compressed, and rehearsed — without new world data.

## Core Principle

> Waking gathers experience. Sleep organizes it.

## Why Sleep Exists

Continuous experience would overwhelm an unbounded graph. Sleep provides:

- capacity management (pruning weak edges)
- pattern reinforcement (strengthening reliable predictions)
- abstraction (concept formation via compression)
- rehearsal (spreading activation without sensors)

## Sleep Trigger

**Planned** — composite of:

- high `fatigue` ([04_regulation.md](04_regulation.md))
- low sensory change rate (quiet environment)
- optional energy threshold

**Skeleton:** `Rest` action reduces fatigue slightly; no dedicated sleep state.

## Reduced Sensory Input

During sleep, external sensor channels attenuate toward internal noise floor. Internal sensors remain active. Prevents new experiences from overwriting consolidation.

## Memory Consolidation

Offline pass over `MemoryGraph`: merge redundant sensory nodes, update edge statistics, queue concept candidates.

**Skeleton:** `consolidate_sleep()` empty in `graph.rs`.

## Edge Strengthening

Edges with high `observations` and low prediction error gain `strength` and `confidence`.

## Edge Weakening

Edges with poor predictive value decay — preventing outdated associations from dominating.

## Concept Formation

Dense clusters of sensory nodes become `Concept` nodes linked via `concept_compresses` edges ([07_concepts.md](07_concepts.md)).

## Compression

Reduce graph size while preserving predictive structure — essential for long-lived organisms.

## Concept Drift

Concepts shift as new experiences differ from compressed prototype — tracked via edge confidence decay.

## Concept Merging

Similar concepts (high cosine similarity on activation profiles) merge into one node.

## Concept Splitting

Bimodal outcome distributions split a concept into two when prediction error rises.

## Spreading Activation

**Planned** ([08_prediction.md](08_prediction.md)) — during sleep, activate subgraphs without external input to test predictive coherence.

## Imagination

Activation of memory chains **without current sensing** — simulating possible futures for comparison. Proto-planning offline.

## Dreaming

**Planned** — unconstrained spreading activation producing sensor-like internal patterns; may strengthen unusual associations (creative error).

## Prediction Improvement

Sleep tunes `delay_mean` / `delay_variance` on edges and prunes misleading `action_leads_to` links — lowering future `Experience.outcome` magnitude.

## Knowledge Accumulation

Consolidated concepts persist across wake cycles; cumulative predictive structure without explicit "knowledge" variable.

## Social Consequences

Shared sleep environments may synchronize activation patterns — link to [10_communication.md](10_communication.md), [11_social_systems.md](11_social_systems.md), [16_culture.md](16_culture.md).

## Inheritance

**Planned** — compressed concept subgraphs partially copied at reproduction; sleep-quality affects fidelity of inherited structure.

## Computational Interpretation

Sleep implements:

| Process | Algorithmic analog |
|---------|-------------------|
| Consolidation | Batch graph update |
| Compression | Clustering / autoencoder |
| Edge strengthen/weaken | Hebbian / anti-Hebbian |
| Pruning | Sparsification |
| Dreaming | Generative replay |

## Core Principle

> Sleep turns traces into structure. Structure turns into prediction.

Researcher labels (food, caves, shelter, water sources) describe emergent clusters that may consolidate during sleep — never pre-named in creature memory.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `consolidate_sleep()` | `memory/graph.rs` | Edge strengthen/weaken; concept cluster/merge/split |
| `imagination_replay()` | `memory/graph.rs` | Offline spread activation during sleep ticks |
| `try_early_wake()` | `creature.rs` | Exit sleep when fatigue < 0.35 |
| `dream_noise` config | `scheduler.rs` | Optional edge noise during imagination |
| `imagination_events` | `export/logs.rs` | Per-tick sleep replay count |
| `fatigue`, `Rest` | `regulation.rs`, `actions.rs` | Partial recovery + wet-environment hydration |
| Sleep state / trigger | `creature.rs`, `engine.rs` | Fatigue > 0.65 + light < 0.45 |
| Sensory attenuation | `sensors.rs` `read_sensors_with_noise` | 0.25× noise while sleeping |
| Sleep in snapshot | `export/snapshots.rs` | `sleeping`, `sleep_ticks_remaining` |

## Planned

- Richer imagination replay (multi-seed chains)
- Integration with [08_prediction.md](08_prediction.md) for offline prediction scoring

## Open questions

- Fixed sleep duration vs fatigue-driven exit?
- Should dreaming intentionally add noise to edges?
