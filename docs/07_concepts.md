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

`sim-core/src/memory/concepts.rs` — `ConceptNode` (prototype sensor vector, member node ids, strength), `ActiveConcept`, `activate_concepts()`. Sleep consolidation clusters ≥3 similar sensory nodes (cosine > 0.85) into concepts; creature `concept_nodes` and `active_concepts` updated in `creature.rs` / `engine.rs`. Exported in `CreatureSnapshot`.
