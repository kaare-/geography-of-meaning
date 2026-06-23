# World Initialization and Eden

> **Guiding question:** How does the simulation begin before culture exists?

## Status

**Partial** — `Simulation::new` generates terrain, scans for favorable spawn voxels, and places a configurable initial population. Morphology variation across individuals and multiple independent starting regions are **planned**.

## Summary

The simulation begins before culture, before infrastructure, and before communication systems.

A small population of organisms is introduced into a favorable environment.

This environment serves as the initial ecological cradle from which future exploration, migration, and culture emerge.

The purpose of Eden is not permanence.

Its purpose is to provide a stable starting point from which complexity can develop.

**Eden** is a researcher term for the starting cradle — it does not appear in creature code.

## Core Principle

The world should not begin as a finished world.

The world should begin as a world full of possibilities.

Most information should be discovered rather than assigned.

## Initial World State

The world begins with:

* terrain
* climate
* water systems
* vegetation (organic matter in voxels)
* geological structure

No settlements exist.

No infrastructure exists.

No traditions exist.

No culture exists.

Only ecological potential exists.

**Skeleton:** `World::generate_terrain(size_chunks, seed)` in `sim-core/src/world/mod.rs` builds a chunked voxel field via `generate_height_map` and `fill_terrain` ([01_world_generation.md](01_world_generation.md)). Climate defaults to `GlobalClimate::default()`; `time` starts at 0.

## The First Population

A small number of organisms are introduced.

Suggested scale:

**10–100 individuals**

The exact number is less important than diversity.

Initial individuals should vary slightly in:

* morphology
* sensor sensitivity
* metabolism
* learning tendencies

Variation provides material for future evolution.

**Skeleton:** CLI `--creatures` (default 5) sets `SimulationConfig::creature_count`. Each creature receives a random `signature` and varied `Genome` / `Morphology` at spawn (`engine.rs::random_spawn_genome`) — morphology and sensitivity variation across individuals is **partial** ([03_creatures.md](03_creatures.md), [13_inheritance.md](13_inheritance.md)).

If fewer spawn positions are found than requested, remaining creatures are placed at the first spawn position (fallback at `(8, 8, 4)` if none found).

## Eden

The starting environment should be favorable.

Characteristics (researcher labels):

* moderate temperature
* reliable water
* food abundance (organic gradients)
* low environmental stress

The goal is not challenge.

The goal is survival and expansion.

**Skeleton:** `World::find_spawn_positions(count)` scans all voxels and scores candidates where:

```
void_fraction > 0.3
organic > 0.05
temperature > 18.0
water_content + surface_water > 0.1
```

Score: `organic × water × (temperature / 25.0)`. Top-scoring positions are returned. After selection, `World::enrich_spawn_site` boosts organic (+0.12) and water_content (+0.05) in a 3×3×3 neighborhood so the starting cradle has reliable food. Shallow soil organic in `fill_terrain` is slightly richer near the surface. This is procedural habitat selection — not a named Eden region in code.

## Why Eden Exists

A completely hostile environment may produce immediate extinction.

A completely perfect environment may produce stagnation.

Eden provides:

**stability**

without

**perfection**.

The spawn scan biases toward survivable voxels while leaving most of the world unexplored and unoptimized.

## Hidden World

Most of the world remains unknown.

Examples:

* distant valleys
* cave systems
* underground water
* mountain ranges
* seasonal environments

These become future discoveries.

**Skeleton:** terrain is fully generated at init (`--world-size` sets chunk count per axis), but creatures start at a small number of high-score voxels. No fog-of-war or chunk activation limits — the world exists objectively; creatures only know what they sense ([21_exploration_and_discovery.md](21_exploration_and_discovery.md)).

## Information Asymmetry

The world contains information before organisms discover it.

Examples (researcher labels):

* water sources
* fertile soils
* migration corridors

The information exists physically.

Its meaning emerges later.

Creatures never receive world facts — only sensor traces after `read_sensors()` at spawn and each tick.

## Exploration Pressure

As populations grow:

* competition increases
* resources become localized
* prediction uncertainty increases

These pressures encourage exploration.

**Skeleton:** no population growth or crowding yet — reproduction is **planned** ([23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md)). Regulatory pressure via low energy/fatigue in `choose_action()` provides minimal movement incentive ([21_exploration_and_discovery.md](21_exploration_and_discovery.md)).

## First Discoveries

Typical early discoveries may include (researcher labels):

* nearby water sources
* sheltered locations
* food-rich regions
* favorable migration routes

These discoveries become the foundation of culture.

**Skeleton:** discoveries are implicit — new `SensoryPattern` nodes in `memory_graph` when creatures act in unfamiliar voxels. No labeled discovery events.

## First Infrastructure

Initially:

all regulation is internal.

Later:

organisms begin modifying the environment.

Examples (researcher labels):

* paths
* caches
* shelters

External memory begins to emerge.

See [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).

## First Communication

Initially communication may be minimal.

Over time:

repeated interactions produce shared predictive structures.

Signals gradually acquire meaning.

See [10_communication.md](10_communication.md).

## Population Expansion

Successful populations spread outward.

Expansion creates:

* regional variation
* isolation
* adaptation

History begins when populations diverge.

See [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md), [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md).

## Multiple Edens

Future versions may support:

* multiple starting populations
* different climates
* different continents

This allows independent cultural trajectories.

**Planned** — current `find_spawn_positions` returns the top N scored voxels globally; no regional Eden partitioning or separate spawn basins.

## Deep Time

The simulation should support very long runtimes.

The initial population may eventually disappear.

Its influence may persist through:

* descendants
* infrastructure
* landscape modifications
* cultural traces

The world becomes historical.

See [14_time_and_scales.md](14_time_and_scales.md), [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md). CLI `--ticks` controls run length; long-run archaeology is **planned**.

## Current implementation

| Mechanism | Location | Status |
|-----------|----------|--------|
| `Simulation::new` | `simulation/engine.rs` | Implemented |
| `World::generate_terrain` | `world/mod.rs` | Implemented |
| `World::find_spawn_positions` | `world/mod.rs` | Implemented |
| Warm/wet/organic spawn scan | `world/mod.rs` | Implemented |
| `World::enrich_spawn_site` (Eden organic boost) | `world/mod.rs` | Implemented |
| `Creature::new` + initial `read_sensors` | `creatures/creature.rs`, `engine.rs` | Implemented |
| CLI `--creatures`, `--world-size`, `--seed` | `main.rs` | Implemented |
| Morphology variation at spawn | — | Planned |
| Multiple Edens | — | Planned |
| Reproduction / population growth | `lifecycle.rs`, `engine.rs` | Implemented |

## Initialization flow

```
CLI args → SimulationConfig
  → World::generate_terrain(world_chunks, seed)
  → World::find_spawn_positions(creature_count)
  → for each spawn: World::enrich_spawn_site(pos)
  → for each position: Creature::new(id, pos, signature)
  → read_sensors(creature, world, rng)
  → Simulation { world, creatures, ... }
```

See [20_simulation_loop.md](20_simulation_loop.md).

## Cross-references

| Topic | Doc |
|-------|-----|
| Terrain generation | [01_world_generation.md](01_world_generation.md) |
| Organism model | [03_creatures.md](03_creatures.md) |
| Simulation tick | [20_simulation_loop.md](20_simulation_loop.md) |
| Exploration | [21_exploration_and_discovery.md](21_exploration_and_discovery.md) |
| Settlements | [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) |
| Lifecycle | [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md) |
| World history | [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md) |

## Core Principle

The simulation begins with organisms.

The project investigates everything that emerges afterward.
