# Inheritance

> **Guiding question:** How does predictive structure survive across generations?

## Status

**Planned / Stub** — genome fields inherit at reproduction (not yet implemented). No direct memory-graph copy. Landscape and social inheritance planned.

## Summary

Predictive structure outlives individuals through **multiple substrates**: genetic biases, cognitive tendencies, social learning, environmental modification, and landscape memory. No single channel carries "knowledge" intact.

## Core Principle

> Inheritance operates through **multiple substrates** — never a single memory dump.

## Why Inheritance Exists

Individual memory graphs die with the organism. Long-timescale regulation requires persistence across birth and death — via whatever channels reduce regulatory surprise for descendants.

## Forms of Inheritance

| Form | Substrate | Timescale | Skeleton |
|------|-----------|-----------|----------|
| **Genetic** | `Genome` tunables | Generations | `metabolism_rate`, `sensor_noise_scale`, `move_speed` |
| **Cognitive** | Biased attention / learning rates | Lifetime | Planned |
| **Social** | Modified predictions via communication | Days–generations | Planned ([10_communication.md](10_communication.md)) |
| **Environmental** | Construction, trails, caches | Generations+ | Planned ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)) |
| **Landscape** | Voxel fields (erosion, organic, binder) | Geological | Partial fields only |

## Information Persistence

| Timescale | Examples |
|-----------|----------|
| Event | Single experience |
| Experience | Recent buffer (64 ticks) |
| Lifetime | Memory graph |
| Generational | Genome + landscape traces |
| Cultural | Shared compressed concepts |
| Geological | Landform, erosion patterns |

## Direct Memory Transfer

**Avoid.** Copying one creature's `MemoryGraph` to offspring would bypass individual prediction learning and collapse individuality. Preferred: offspring **reconstruct** similar structure through shared traces and biases.

## Preferred Model

1. Genetic biases shape sensor noise, metabolism, morphology (planned)
2. Social signals bias attention ([10_communication.md](10_communication.md))
3. Landscape stores externalized prediction ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md))
4. Sleep consolidates personal graph ([09_sleep.md](09_sleep.md))

## Tradition

**Planned** — stable cross-generation patterns in sound→outcome and trace→construction without explicit teaching.

## Cultural Accumulation

**Planned** — see [16_culture.md](16_culture.md). Layered compression of population-stable predictions.

## Lineages

`signature` distinguishes individuals; genome defines lineage. Lineages diverge when reproductive isolation (spatial or social) separates predictive environments.

## Inheritance and Meaning

Meaning inherited only if compressed structure **still reduces regulatory prediction error** in new contexts — otherwise edges weaken (concept drift).

## Evolution of Concepts

Concepts that survive inheritance are those that compress well and predict reliably across environmental shift ([07_concepts.md](07_concepts.md)).

## Information Bottlenecks

Death, sleep pruning, erosion, and population collapse destroy information. Bottlenecks force re-compression and innovation.

## Innovation

Mutation (genome), prediction error (memory), and environmental shock create novel edges — source of new predictive structure.

## Core Principle

> What survives is what still predicts — across bodies, minds, landscapes, and time.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `Genome` + `mutate_from` | `creatures/genome.rs` | Three tunables with mutation |
| `signature` | `creatures/creature.rs` | Parent hash + RNG variation on birth |
| Reproduction | `creatures/lifecycle.rs` | No memory graph copy |
| Memory copy | — | Explicitly avoided |

## Planned

- Reproduction with genome mutation
- Cognitive bias inheritance
- Social learning without graph copy
- Landscape legibility for offspring

## Open questions

- Minimum viable genome dimensionality?
- Can landscape alone sustain "culture" without communication?
