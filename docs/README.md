# Documentation Index

Reading order for the Geography of Meaning design series.

| Doc | Title | Status | Guiding question |
|-----|-------|--------|------------------|
| [00](00_project_overview.md) | Project Overview | Substantive | — |
| [01](01_world_generation.md) | World Generation | Partial | How does a landscape emerge and evolve? |
| [02](02_physics_and_materials.md) | Physics and Materials | Partial | How does matter behave? |
| [03](03_creatures.md) | Creatures | Partial | What is an organism? |
| [04](04_regulation.md) | Regulation | Partial | What gives experience value? |
| [05](05_sensors.md) | Sensors | Partial | How does reality become traces? |
| [06](06_memory.md) | Memory | Partial | How does experience persist? |
| [07](07_concepts.md) | Concepts | Stub | How does abstraction emerge? |
| [08](08_prediction.md) | Prediction | Stub | How does a creature construct a future? |
| [09](09_sleep.md) | Sleep | Partial | What happens when sensors are quiet? |
| [10](10_communication.md) | Communication | Partial | How does one model alter another? |
| [11](11_social_systems.md) | Conflict, Coordination, and Norms | Stub | How do predictive systems interact? |
| [12](12_construction_and_infrastructure.md) | Construction | Stub | How does memory move into the landscape? |
| [13](13_inheritance.md) | Inheritance | Stub | How does predictive structure survive across generations? |
| [14](14_time_and_scales.md) | Time and Scales | Partial | How do phenomena at different timescales interact? |
| [15](15_information_storage_and_external_memory.md) | Information Storage | Partial | Where does information live? |
| [16](16_culture.md) | Culture | Stub | How do shared predictive patterns stabilize in populations? |
| [17](17_analysis_and_visualization.md) | Analysis | Partial | How do we understand what emerged? |
| [18](18_performance_architecture.md) | Performance | Partial | How do we simulate millions of years efficiently? |
| [19](19_data_model.md) | Data Model | Partial | What runtime structures carry state? |
| [20](20_simulation_loop.md) | Simulation Loop | Partial | How does one tick advance the world? |
| [21](21_exploration_and_discovery.md) | Exploration | Planned / Partial | How does novelty enter prediction? |
| [22](22_settlements_culture_and_civilization.md) | Settlements & Civilization | Planned / Stub | How does culture concentrate in place? |
| [23](23_creature_lifecycle_and_population.md) | Lifecycle & Population | Planned / Partial | How does information persist across generations? |
| [24](24_world_history_and_archaeology.md) | World History & Archaeology | Planned / Partial | How does the past remain present in traces? |
| [25](25_world_initialization_and_eden.md) | World Initialization & Eden | Partial | How does the simulation begin before culture exists? |
| [26](26_research_tools_and_observability.md) | Research Tools & Observability | Partial | How do researchers inspect what emerged? |
| [27](27_narrative_extraction_and_interpretation.md) | Narrative Extraction | Planned | How do meaningful stories emerge from simulation data? |
| [28](28_language_and_signal_evolution.md) | Language & Signal Evolution | Planned | How do signals become shared predictive structure? |

**Reading order (19–28):** Implementers should read [19](19_data_model.md) after [00](00_project_overview.md)–[03](03_creatures.md), or alongside [04](04_regulation.md)–[08](08_prediction.md) as a technical reference. Read [20](20_simulation_loop.md) after [19](19_data_model.md) or alongside [14](14_time_and_scales.md) and [18](18_performance_architecture.md). Docs [21](21_exploration_and_discovery.md)–[22](22_settlements_culture_and_civilization.md) are long-horizon design — read after [06](06_memory.md)–[13](13_inheritance.md). Read [23](23_creature_lifecycle_and_population.md) after [03](03_creatures.md) and [13](13_inheritance.md); it extends the organism model with population-scale dynamics. Read [24](24_world_history_and_archaeology.md) after [01](01_world_generation.md), [12](12_construction_and_infrastructure.md), [15](15_information_storage_and_external_memory.md), and [22](22_settlements_culture_and_civilization.md) — material history and archaeological reading of the landscape. Read [25](25_world_initialization_and_eden.md) before [20](20_simulation_loop.md) for startup context. Read [26](26_research_tools_and_observability.md) and [27](27_narrative_extraction_and_interpretation.md) after [17](17_analysis_and_visualization.md) and [24](24_world_history_and_archaeology.md). Read [28](28_language_and_signal_evolution.md) after [10](10_communication.md), [06](06_memory.md), [07](07_concepts.md), and [16](16_culture.md) — long-horizon design for signal evolution, dialects, and emergent language.

## Changes from v0.1 plan (00–14)

| Original | Current | Notes |
|----------|---------|-------|
| 07_prediction only | 04 regulation + 08 prediction | Regulation inserted as value foundation after creatures |
| 12_culture | 13 inheritance, 16 culture | Inheritance split out; culture shifted and reframed |
| — | 14_time_and_scales | New cross-cutting doc |
| — | 15_information_storage | New synthesis / substrate index |
| 13_analysis | 17_analysis | Renumbered |
| 14_performance | 18_performance | Renumbered |

## Code mapping

| Doc | Module |
|-----|--------|
| 01, 02 | `sim-core/src/world/` |
| 03, 04 | `sim-core/src/creatures/` |
| 05 | `sim-core/src/creatures/sensors.rs` |
| 06 | `sim-core/src/memory/`, `creatures/creature.rs` |
| 07 | `sim-core/src/memory/concepts.rs` |
| 08, 09 | `sim-core/src/memory/graph.rs` (planned) |
| 17 | `sim-core/src/export/`, `analysis/scripts/` |
| 18, 20 | `sim-core/src/simulation/` |
| 19 | `sim-core/src/world/`, `creatures/`, `memory/`, `export/` |
| 21 | `creatures/actions.rs`, `memory/graph.rs` (partial) |
| 22 | — (planned) |
| 23 | `creatures/creature.rs`, `genome.rs`, `simulation/engine.rs` (partial) |
| 24 | `world/voxel.rs` (`erosion_damage`, `porosity`; planned) |
| 25 | `simulation/engine.rs`, `world/mod.rs` (`generate_terrain`, `find_spawn_positions`) |
| 26 | `sim-core/src/export/`, `analysis/scripts/` |
| 27 | — (planned; builds on 26 exports) |
| 28 | `creatures/sensors.rs`, `memory/nodes.rs`, `memory/edges.rs`, `creatures/creature.rs` (planned production/mutation) |

## Architecture pipeline

```
World → Sensors → Experiences → Memory → Concepts → Active Concepts
  → Spreading Activation → Predictions → Actions → World
```

See [00_project_overview.md](00_project_overview.md).
