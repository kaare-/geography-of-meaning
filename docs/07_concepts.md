# Concepts

> **Guiding question:** How does abstraction emerge?

## Status

**Partial** — `ConceptNode` clustering in sleep consolidation; `activate_concepts` stub at runtime.

## Overview

Concepts compress dense regions of the memory graph into single nodes linked via `concept_compresses` edges. They reduce dimensionality while preserving predictive structure.

## Planned

- Concept hierarchies
- Links to [06_memory.md](06_memory.md), [08_prediction.md](08_prediction.md), [09_sleep.md](09_sleep.md)

## Current implementation

`sim-core/src/memory/concepts.rs` — `ConceptNode`, `ActiveConcept`, `activate_concepts()`. Sleep consolidation in `memory/graph.rs`: clusters ≥2 similar sensory nodes (cosine ≥ 0.70); dedup via `ConceptCompresses`; EMA prototype drift (`PROTOTYPE_EMA_ALPHA`); merge when concept prototypes ≥ 0.88 similar; split when member variance > 0.12. Consolidation at wake only (`creature.rs`). Concept inheritance on reproduction (up to 3, strength × 0.8). `ConceptNodeSnapshot` + `active_concepts` in `CreatureSnapshot`. Tick log: `concept_merge_count`, `concept_split_count`. Narrative: `peak_population_concept_count`.
