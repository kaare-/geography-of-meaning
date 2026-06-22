# Information Storage and External Memory

> **Guiding question:** Where does information live?

## Status

**Partial / Planned** — internal memory graph implemented; communication, construction, and landscape storage planned. JSON exports are **research artifacts** ([17_analysis_and_visualization.md](17_analysis_and_visualization.md)).

> **Researcher framing:** Labels like food cache, shelter, wall, river, and mountain describe **emergent storage patterns** — never creature cognition types.

## Summary

Information is whatever **influences prediction**. It is stored across multiple media: neural graphs, acoustic traces, built form, voxel fields, and population-stable concepts. No single substrate is authoritative.

## Core Principle

> Information is whatever influences prediction.

## Why Information Matters

Regulation improves when future states are anticipated. Storage preserves predictive structure across time, space, and individuals — reducing surprise cost.

## Information Storage Substrates

| Substrate | Medium | Doc |
|-----------|--------|-----|
| Internal memory | `MemoryGraph` | [06_memory.md](06_memory.md) |
| Communication | Sound / chemical traces | [10_communication.md](10_communication.md) |
| Infrastructure | Built voxel form | [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) |
| Landscape | Terrain fields | [01_world_generation.md](01_world_generation.md) |
| Culture | Shared concepts | [16_culture.md](16_culture.md) |

## Internal Memory

Per-creature directed graph — relationships, not facts. `memory/graph.rs`.

## Communication

Transient traces that modify listener predictions. No message database.

## Infrastructure

Persistent material arrangements externalizing prediction ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)). Researcher labels: shelter, cache, wall — emergent only.

## Landscape

Environmental memory: `erosion_damage`, `organic`, `porosity`, `binder`, compacted pathways ([01_world_generation.md](01_world_generation.md)).

## Culture

Slow shared compression across generations ([16_culture.md](16_culture.md), [13_inheritance.md](13_inheritance.md)).

## Multiple Storage Media

Information may exist redundantly — a pathway (landscape) and a concept (memory) may encode the same predictive shortcut.

## Persistence Hierarchy

| Durability | Storage |
|------------|---------|
| Lowest | Single sensor sample |
| Low | Recent experience buffer |
| Medium | Memory edges |
| High | Landscape / infrastructure |
| Highest | Geological + cultural (planned) |

## External Memory

Memory outside the skull: landscape + artifacts + signals. Offloads computation into the environment ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md) — this doc).

## Distributed Cognition

Population + landscape + signals form a coupled predictive system. No central store.

## Information Transfer Chain

```
Experience → Memory → (Communication | Construction) → Landscape → Offspring traces → New memory
```

No direct graph copy ([13_inheritance.md](13_inheritance.md)).

## Information Loss

Death, sleep pruning, erosion, collapse, and cultural drift destroy storage. Loss forces re-learning and innovation.

## Maintenance

Edges strengthen with use; structures need material upkeep; cultural patterns need reproduction of behavior traces.

## Information Accumulation

Over time, landscape and culture store more predictive structure — until catastrophe or bottleneck resets.

## Information and Meaning

Stored structure is meaningful when retrieval **changes regulatory predictions** ([04_regulation.md](04_regulation.md)).

## Information and Time

Different substrates operate on different clocks ([14_time_and_scales.md](14_time_and_scales.md)).

## Information and Geography

**Physical geography** — voxel fields, coordinates.  
**Cognitive geography** — concept clusters and memory subgraphs.  
Together: [Geography of Meaning](00_project_overview.md#geography-of-meaning).

## Central Question

> How does predictive structure persist outside the individual?

## Core Principle

> The world is a storage medium. Creatures read it through sensors, write it through action.

## Current implementation

| Component | Status |
|-----------|--------|
| `MemoryGraph` | Implemented |
| Landscape fields | Partial (no env-memory tick) |
| Construction storage | Planned |
| Communication storage | Planned |
| Export snapshots | Research artifact only |

## Planned

- Landscape memory tick logic
- Infrastructure as writable storage
- Memory graph export per creature
- Information genealogy visualization

## Open questions

- Measure information content in bits or predictive delta only?
- When does landscape storage outperform internal memory?
