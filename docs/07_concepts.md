# Concepts

> **Guiding question:** How does abstraction emerge?

## Status

**Partial** — `ConceptNode` clustering in sleep consolidation; `activate_concepts` stub at runtime.

## Overview

Concepts compress dense regions of the memory graph into single nodes linked via `concept_compresses` edges. They reduce dimensionality while preserving predictive structure.

## Planned

- Memory clustering by sensory similarity
- Compression into `Concept` nodes
- Hierarchies and concept drift
- Concept inheritance across reproduction
- Links to [06_memory.md](06_memory.md), [08_prediction.md](08_prediction.md), [09_sleep.md](09_sleep.md)

## Current implementation

`sim-core/src/memory/concepts.rs` — `ConceptNode` (prototype sensor vector, member node ids, strength), `ActiveConcept`, `activate_concepts()`. Sleep consolidation clusters ≥2 similar sensory nodes (cosine ≥ 0.70) into concepts; loose sensory nodes merge into existing clusters during `consolidate_sleep`. Consolidation runs at sleep onset and wake (`creature.rs`). Creature `concept_nodes` and `active_concepts` updated in `creature.rs` / `engine.rs`. `concepts_formed` per tick in `TickLogEntry`. `population_concept_count` in `narrative_summary.json`. Exported in `CreatureSnapshot`.
