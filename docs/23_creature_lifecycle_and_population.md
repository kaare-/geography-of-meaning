# Creature Lifecycle and Population

> **Guiding question:** How do individuals enter, change, and leave populations while information persists?

## Status

**Planned / Partial** — `Creature::new` spawn, `age` increment each tick, and `Genome` fields exist. Reproduction, death, aging effects, population tracking, and ecological recycling are planned. See [03_creatures.md](03_creatures.md) for organism-level detail.

## Summary

The simulation follows populations rather than isolated organisms.

Each organism exists only temporarily.

Information, structures, and landscapes may survive much longer.

The lifecycle defines how organisms enter the world, interact with it, reproduce, and eventually disappear.

## Core Principle

Individuals are temporary.

Predictive structures may persist.

The simulation is therefore concerned not only with organisms but with the persistence of information across successive generations.

## Lifecycle Overview

```
Birth
  → Development
  → Exploration
  → Learning
  → Reproduction
  → Aging
  → Death
```

Each stage influences the next.

## Birth

New organisms enter the world through reproduction.

An offspring receives:

* a genome
* inherited biases
* a physical body
* a position within an existing environment

The offspring does not receive knowledge directly.

It must construct its own predictive model.

**Skeleton:** initial population via `Creature::new` and `World::find_spawn_positions` — procedural spawn, not reproduction. Genome inheritance on birth — **planned** ([13_inheritance.md](13_inheritance.md)).

## Early Development

Young organisms possess:

* limited experience
* limited concepts
* high uncertainty

They depend heavily on:

* inherited biases
* environmental structure
* nearby organisms

The world initially appears unfamiliar.

**Skeleton:** empty `recent_experience` buffer and sparse `memory_graph` at spawn. No explicit developmental stage — **planned**.

## Learning Phase

Most predictive structures emerge through experience.

The organism:

* senses
* acts
* experiences outcomes
* sleeps
* forms concepts

Learning is continuous.

See [06_memory.md](06_memory.md), [07_concepts.md](07_concepts.md), [09_sleep.md](09_sleep.md).

## Exploration Phase

As memory grows, organisms increasingly move beyond familiar environments.

Exploration generates:

* new information
* prediction errors
* new concepts

Exploration introduces novelty into the population.

See [21_exploration_and_discovery.md](21_exploration_and_discovery.md).

## Adulthood

Adult organisms possess:

* established concepts
* stable prediction systems
* developed morphology
* social relationships

Adults perform most:

* construction
* communication
* reproduction
* infrastructure maintenance

**Planned** — no explicit life-stage enum. `age` increments each tick without stage transitions.

## Reproduction

Reproduction occurs when:

* energy reserves are sufficient
* environmental conditions are favorable
* regulatory stability exists

Reproduction is expensive.

Energy invested in offspring cannot be used elsewhere.

**Planned** — genome mutation and offspring spawn. See [03_creatures.md](03_creatures.md), [13_inheritance.md](13_inheritance.md).

## Reproductive Tradeoffs

Different strategies may emerge.

Examples:

**Many offspring:**

* low investment
* high mortality

**Few offspring:**

* high investment
* lower mortality

The simulation should allow ecological conditions to shape these strategies.

**Planned** — no r/K or clutch-size parameters in skeleton.

## Aging

Aging gradually reduces performance.

Possible effects:

* reduced mobility
* reduced repair capacity
* increased maintenance cost
* declining sensory precision

Aging creates generational turnover.

**Skeleton:** `age` field increments in `sim-core/src/creatures/creature.rs`; no performance degradation yet — **planned**.

## Death

Death is inevitable.

Causes may include:

* starvation
* dehydration
* exposure
* injury
* old age

Death removes the organism.

Death does not remove information.

**Planned** — removal from simulation and organic deposition into voxel fields. Inherited and externalized information persists via [13_inheritance.md](13_inheritance.md).

## Ecological Recycling

After death:

```
body → organic matter → ecosystem
```

The organism becomes part of future ecological cycles.

**Planned** — transfer organic fraction from creature to voxel on death ([01_world_generation.md](01_world_generation.md), [02_physics_and_materials.md](02_physics_and_materials.md)).

## Population Dynamics

Population size is not fixed.

Population emerges from:

```
births − deaths
```

Population responds to:

* food availability (researcher label — organic gradients in code)
* climate
* infrastructure
* competition
* disease (future possibility)

**Planned** — no population-level aggregate tracking in skeleton.

## Carrying Capacity

Every environment possesses limits.

Limits emerge from:

* productivity
* water availability
* temperature
* infrastructure

Population growth eventually encounters constraints.

**Planned** — emergent from regulatory pressure and resource fields, not a hard cap.

## Density Effects

Increasing density changes behavior.

Examples:

* competition
* communication
* conflict
* cooperation

Density influences social complexity.

See [10_communication.md](10_communication.md), [11_social_systems.md](11_social_systems.md).

## Migration

When local regulation deteriorates:

organisms may migrate.

Migration redistributes:

* bodies
* concepts
* communication systems
* infrastructure practices

Migration moves information through the world.

See [21_exploration_and_discovery.md](21_exploration_and_discovery.md).

## Founder Effects

Small groups entering new regions may carry only part of the parent population's predictive structures.

Different populations may therefore diverge over time.

**Planned** — population subdivision and partial inheritance of communication and landscape traces.

## Population Memory

A population possesses memory beyond individual organisms.

Examples:

* communication systems
* traditions
* infrastructure
* settlement patterns

Population memory emerges through persistence.

See [13_inheritance.md](13_inheritance.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md), [16_culture.md](16_culture.md), [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md).

## Extinction

Entire populations may disappear.

Extinction removes:

* bodies
* communication systems
* traditions

However:

landscapes and infrastructure may preserve traces.

## Evolution

Evolution acts on:

* genomes
* morphology
* learning biases
* communication tendencies

Evolution does not directly optimize intelligence.

Evolution optimizes persistence.

**Skeleton:** `Genome` tunables (`metabolism_rate`, `sensor_noise_scale`, `move_speed`) — mutation on reproduction **planned**.

## Population as Information

A population is not merely a collection of organisms.

It is a collection of predictive structures distributed across:

* bodies
* memories
* communication systems
* infrastructure
* landscapes

## Current implementation

| Mechanism | Location | Status |
|-----------|----------|--------|
| Spawn at favorable voxels | `world/mod.rs` `find_spawn_positions` | Implemented |
| `Creature::new` | `creatures/creature.rs` | Implemented |
| `age` increment | `simulation/engine.rs` | Implemented |
| `Genome` defaults + `mutate_from` | `creatures/genome.rs` | `metabolism_rate` default `0.008` |
| Reproduction | `creatures/lifecycle.rs` `try_reproduce` | Implemented |
| High-energy reproduction chance | `lifecycle.rs` | `2.5%` per tick when energy > `0.6` |
| Death / removal | `lifecycle.rs`, `engine.rs` | Implemented |
| Organic recycling on death | `lifecycle.rs` `deposit_creature_organic` | Implemented |
| Birth events in tick log | `export/logs.rs` | Implemented |
| Population cap | `scheduler.rs` `max_population` | Default `30` |
| Aging effects | — | Planned |
| Population aggregates | — | Planned |

## Planned

- Death triggers beyond regulatory failure (age)
- Aging modifiers on mobility, repair, metabolism
- Ecological recycling into voxel organic fields
- Density-dependent behavior coupling
- Population-level export metrics for analysis

## Cross-references

| Topic | Doc |
|-------|-----|
| Organism model | [03_creatures.md](03_creatures.md) |
| Generational transfer | [13_inheritance.md](13_inheritance.md) |
| Exploration & migration | [21_exploration_and_discovery.md](21_exploration_and_discovery.md) |
| Simulation tick order | [20_simulation_loop.md](20_simulation_loop.md) |
| Timescales | [14_time_and_scales.md](14_time_and_scales.md) |

## Core Principle

The organism is temporary.

The population is longer-lived.

The landscape is longer-lived still.

The simulation studies how information moves between these layers through birth, learning, reproduction, and death.
