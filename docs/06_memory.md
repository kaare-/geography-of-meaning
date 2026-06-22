# Memory

> **Guiding question:** How does experience persist?

## Status

**Partial** — experience recording, directed graph nodes/edges, and similarity retrieval are implemented. Spreading activation, prediction traversal, sleep consolidation, imagination, and full concept compression are planned.

## Summary

Memory is not a recording of the world. It is a **graph of relationships** between sensory patterns, actions, outcomes, sounds, and (eventually) compressed concepts. Each creature maintains its own memory graph, built incrementally from lived experience.

The landscape and the creature's internal graph are separate memory systems. What a researcher might later label as a mountain, river, cave, or shelter exists only as **patterns in sensory traces and their predictive relationships** — never as named concept nodes in the skeleton.

## Core Principle

Creatures do not store the world. They store **relationships**:

- what sensory patterns co-occurred
- what preceded what
- what actions led to what outcomes
- (planned) what sounds activated what patterns
- (planned) what concept nodes compress which clusters

Memory is relational, directed, and weighted by strength, confidence, and observation count.

## Memory Architecture

```
SensoryPattern ──precedes──▶ Action ──action_leads_to──▶ Outcome
       │                                              │
       └────────────────co_occurs─────────────────────┘
```

Implemented in `sim-core/src/memory/graph.rs` as `MemoryGraph { nodes, edges }`.

## Why Direction Matters

Undirected associations cannot support prediction. If A and B merely co-occur, there is no temporal structure. Directed edges encode **time's arrow**:

- `precedes` — temporal ordering of sensory states
- `action_leads_to` — agency linking action to outcome
- `action_leads_to` with delay statistics — future prediction chains

Prediction (planned, see [08_prediction.md](08_prediction.md)) will traverse directed edges forward in time.

## Memory Nodes

Defined in `sim-core/src/memory/nodes.rs` as `NodeKind`:

| Kind | Payload | Role |
|------|---------|------|
| `SensoryPattern` | `SensorState` (15 channels) | Snapshot of sensor traces at a moment |
| `Action` | `Action` enum | move / consume_organic / rest |
| `Outcome` | `f32` | Scalar result (prediction error placeholder) |
| `Sound` | `f32` | Sound event intensity (stub) |
| `Concept` | — | Placeholder for compressed clusters ([07_concepts.md](07_concepts.md)) |

## Edge Types

Defined in `sim-core/src/memory/edges.rs` as `EdgeType` with fields `strength`, `confidence`, `observations`, `delay_mean`, `delay_variance`:

| Edge | Meaning |
|------|---------|
| `CoOccurs` | Two patterns observed together |
| `Precedes` | Source temporally before target |
| `ActionLeadsTo` | Action node linked to outcome |
| `SoundActivates` | Sound triggers sensory pattern (planned) |
| `ConceptCompresses` | Concept node summarizes a cluster (planned) |

## Experience Formation

Each tick, after action and sensor update, the engine builds an `Experience` in `sim-core/src/creatures/creature.rs`:

```text
sensory_before  → sensor state before action
state_before    → regulatory state before action
action          → chosen action
sensory_after   → sensor state after action
state_after     → regulatory state after action
outcome         → delta energy (prediction error placeholder)
timestamp       → world time
```

Recent experiences are capped at 64 (`MAX_RECENT_EXPERIENCE`). Each experience is also passed to `MemoryGraph::record_experience`.

## Retrieval

`MemoryGraph::find_similar_sensory(pattern, threshold)` performs **cosine similarity** over the 15-channel sensor vector. Matching patterns reuse existing sensory nodes rather than duplicating identical traces.

## Active Concepts

**Planned** ([07_concepts.md](07_concepts.md)) — concept nodes that activate subsets of the memory graph during prediction and action selection.

## Spreading Activation

**Planned** ([08_prediction.md](08_prediction.md)) — when a sensory pattern is recognized, activation propagates along directed edges weighted by strength and confidence.

## Prediction

**Planned** ([08_prediction.md](08_prediction.md)) — traverse `precedes` and `action_leads_to` edges to estimate likely future sensory and regulatory states before choosing an action.

## Compression

**Planned** ([07_concepts.md](07_concepts.md)) — merge densely connected sensory clusters into `Concept` nodes via `concept_compresses` edges, reducing graph size while preserving predictive structure.

## Concept Formation

`sim-core/src/memory/concepts.rs` holds a `ConceptNode` placeholder. No compression runs in the skeleton. Emergent clusters (what researchers might call mountain, river, cave, food, or shelter) must arise from graph structure — not from pre-assigned labels.

## Sleep

**Planned / stub** ([09_sleep.md](09_sleep.md)) — `MemoryGraph::consolidate_sleep()` is a no-op. Future: strengthen frequent edges, weaken unused ones, create concept nodes offline when sensors are quiet.

## Imagination

**Planned** — activate memory subgraphs without external sensory input, simulating possible futures for prediction comparison.

## Prediction Error

`Experience.outcome` is currently `energy_after - energy_before`. This scalar is the skeleton's stand-in for **prediction error**: the difference between expected and actual internal state change. Meaning (predictive relevance) accrues to patterns that reliably reduce this error.

## Individuality

Each creature owns a separate `MemoryGraph` and `signature`. Two creatures in the same place develop different graphs because their action histories and noise-realized sensor traces diverge.

## Meaning

Something becomes meaningful when it **changes expectations about future states**. In the skeleton, this is implicit: edges strengthen when experiences repeat; high-confidence `action_leads_to` links encode "when I did this in this context, this happened." Full meaning-as-prediction requires prediction traversal (planned).

## Current implementation

| Component | Location |
|-----------|----------|
| `Experience` struct | `sim-core/src/creatures/creature.rs` |
| `MemoryGraph`, `record_experience`, `find_similar_sensory` | `sim-core/src/memory/graph.rs` |
| `record_heard_sound`, `node_summary`, concept clustering in `consolidate_sleep` | `sim-core/src/memory/graph.rs` |
| `NodeKind`, `MemoryNode` | `sim-core/src/memory/nodes.rs` |
| `EdgeType`, `MemoryEdge` | `sim-core/src/memory/edges.rs` |
| `ConceptNode`, `ActiveConcept`, `activate_concepts` | `sim-core/src/memory/concepts.rs` |
| Tick integration | `sim-core/src/simulation/engine.rs` |

## Planned

- Spreading activation and prediction traversal
- Imagination / offline activation
- Full compression hierarchies and concept drift
- Memory graph JSON export per creature

## Open questions

- Should outcome nodes aggregate multiple regulatory deltas (energy, hydration, integrity)?
- What similarity threshold best balances reuse vs discrimination?
- When should recent_experience flush into long-term graph-only storage?
