# Geography of Meaning

## Status

**Substantive** — primary onboarding document.

## Summary

Geography of Meaning is a simulation-art project investigating how meaning emerges from the interaction between prediction, memory, communication, environmental modification, inheritance, and geography.

The project does not begin with language. It begins with simple organisms attempting to regulate internal conditions in a dynamic world composed of water, weather, erosion, materials, and time.

The central hypothesis is that meaning emerges whenever an organism develops structures that improve prediction of future states. Language, concepts, stories, norms, traditions, and culture are understood as later developments built upon this foundation.

## Fundamental Principle

Organisms do not experience the world directly — they experience **traces**.  
**Meaning = predictive relevance** — something matters when it changes expectations about future **regulatory** state.

## Core Architecture

Extended predictive loop (implemented stages in **bold**):

```
World → Sensors → Experiences → Memory → Concepts → Active Concepts
  → Spreading Activation → Predictions → Actions → World
```

| Stage | Doc | Code | Status |
|-------|-----|------|--------|
| World | [01](01_world_generation.md) | `world/` | Partial |
| Regulation | [04](04_regulation.md) | `regulation.rs` | Partial |
| Sensors | [05](05_sensors.md) | `sensors.rs` | Partial |
| Experiences | [06](06_memory.md) | `creature.rs` | Partial |
| Memory | [06](06_memory.md) | `memory/graph.rs` | Partial |
| Concepts | [07](07_concepts.md) | `concepts.rs` | Stub |
| Prediction | [08](08_prediction.md) | — | Stub |
| Actions | [03](03_creatures.md) | `actions.rs` | Partial |
| Sleep | [09](09_sleep.md) | `memory/graph.rs` (planned) | Partial |
| Communication | [10](10_communication.md) | — | Partial |
| Social systems | [11](11_social_systems.md) | — | Stub |
| Construction | [12](12_construction_and_infrastructure.md) | — | Stub |
| Inheritance | [13](13_inheritance.md) | `genome.rs` (planned) | Stub |
| Time & scales | [14](14_time_and_scales.md) | `simulation/engine.rs` | Partial |
| Info storage | [15](15_information_storage_and_external_memory.md) | — | Partial |
| Culture | [16](16_culture.md) | — | Stub |
| Analysis | [17](17_analysis_and_visualization.md) | `export/` | Partial |
| Performance | [18](18_performance_architecture.md) | `simulation/` | Partial |
| Data model | [19](19_data_model.md) | `world/`, `creatures/`, `memory/` | Partial |
| Simulation loop | [20](20_simulation_loop.md) | `simulation/engine.rs` | Partial |
| Exploration | [21](21_exploration_and_discovery.md) | `actions.rs`, `memory/graph.rs` | Planned / Partial |
| Settlements | [22](22_settlements_culture_and_civilization.md) | — | Planned / Stub |
| Lifecycle & population | [23](23_creature_lifecycle_and_population.md) | `creature.rs`, `genome.rs`, `engine.rs` | Planned / Partial |
| World history & archaeology | [24](24_world_history_and_archaeology.md) | `voxel.rs` (`erosion_damage`, `porosity`) | Planned / Partial |
| World initialization & Eden | [25](25_world_initialization_and_eden.md) | `engine.rs`, `world/mod.rs` | Partial |
| Research tools & observability | [26](26_research_tools_and_observability.md) | `export/` | Partial |
| Narrative extraction | [27](27_narrative_extraction_and_interpretation.md) | — | Planned |
| Language & signal evolution | [28](28_language_and_signal_evolution.md) | `sensors.rs`, `memory/` (planned) | Planned |

## World (summary)

Dynamic chunked voxel landscape — climate, water, materials, erosion (planned). Landscape functions as **external memory** ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md)).

## Sensors (summary)

Fifteen noisy channels; no object perception ([05_sensors.md](05_sensors.md)).

## Memory (summary)

Directed graph of relationships, not world facts ([06_memory.md](06_memory.md)).

## Concepts (summary)

Compressed predictive regions for efficiency ([07_concepts.md](07_concepts.md)) — planned.

## Prediction (summary)

Graph traversal and activation ([08_prediction.md](08_prediction.md)) — planned.

## Imagination (summary)

Activation without current sensing ([09_sleep.md](09_sleep.md)) — planned.

## Communication (summary)

Modifies another's prediction ([10_communication.md](10_communication.md)) — stub.

## Language (summary)

Signals evolve into shared predictive structure across populations ([28_language_and_signal_evolution.md](28_language_and_signal_evolution.md)) — planned. No hardcoded vocabulary in creature code.

## Geography of Meaning

Two coupled landscapes:

| Landscape | Medium | Timescale |
|-----------|--------|-----------|
| **Physical geography** | Voxels, water, climate, erosion | Fast → geological |
| **Cognitive geography** | Memory graphs, concepts, norms | Experience → cultural |

Researchers may label emergent physical patterns (river, cave, mountain, shelter, food cache). Creatures never receive these as cognition — only sensor traces and regulatory consequences.

## Definition of Meaning

**Meaning is predictive relevance.** Examples (smell, wall, sound, story, concept) illustrate emergent structures at the philosophical level — not creature-facing types.

## Research Questions

- How do concepts emerge from experience?
- How do organisms construct models of reality?
- How do communication and culture emerge?
- How does information become embedded in landscapes and infrastructures?
- How do predictive structures survive across generations?

## Long-Term Goal

Create a simulation in which language, concepts, stories, traditions, and culture emerge from interactions between organisms and a changing landscape rather than being explicitly programmed.

## Tech stack

- **Rust** — `sim-core` simulation engine
- **Python** — `analysis/` visualization and research tools
- **JSON** — snapshot and tick log exports

## Getting started

```bash
cargo run -- --ticks 100 --seed 42 --world-size 2 --creatures 5 --output exports
python analysis/scripts/load_snapshot.py exports/snapshots/world_final.json
```

See [docs/README.md](README.md) for the full documentation series.

## Design constraints

No hardcoded cognition concepts (`food`, `shelter`, `wall`, `river`, etc.) in creature code. Researcher labels are permitted in analysis and documentation only.
