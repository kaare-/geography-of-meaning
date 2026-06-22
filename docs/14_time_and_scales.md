# Time and Scales

> **Guiding question:** How do phenomena at different timescales interact?

## Status

**Partial / Planned** — no dedicated time module. Cross-links to `World::time`, `GlobalClimate::season`, `Creature::age`, tick loop in `simulation/engine.rs`. Multi-timescale scheduling planned in [18_performance_architecture.md](18_performance_architecture.md).

## Summary

The simulation spans **nested temporal scales** from single events to geological epochs. Fast processes (sensing, action) nest inside slow ones (seasons, inheritance, landform change). Meaning and memory operate on different clocks than physics.

## Core Principle

> Fast processes nest inside slow ones. Meaning accumulates on slower clocks than physics.

## Temporal Hierarchy

| Scale | Duration (conceptual) | Primary locus | Skeleton |
|-------|---------------------|---------------|----------|
| Event | 1 tick | Sensor sample | `engine.rs` tick |
| Experience | 1 action cycle | `Experience` | Implemented |
| Daily | ~N ticks | Regulatory rhythm | Planned |
| Sleep | Fatigue threshold | `consolidate_sleep` | Stub |
| Seasonal | `season` 0–1 | `GlobalClimate` | Partial |
| Lifetime | `age` | Creature graph | `age` counter |
| Generational | Reproduction interval | Genome + landscape | Planned |
| Cultural | Many generations | Shared concepts | Planned |
| Infrastructure | Decades+ | Binder, pathways | Planned |
| Landscape | Centuries+ | Erosion, deposition | Planned |
| Geological | 10⁴+ ticks | Terrain, caves | Planned |

## Event Time

One simulation tick: rain may fall, water flows, creatures sense→act→remember. `World::time` increments in `tick_climate_and_water`.

## Experience Time

Single action bracket: `sensory_before` → action → `sensory_after`. Timestamp = world time at tick.

## Daily Cycles

**Planned** — diurnal temperature and light modulation ([01_world_generation.md](01_world_generation.md)).

## Sleep Time

Offline consolidation when fatigue high and sensory change low ([09_sleep.md](09_sleep.md)).

## Seasonal Time

`GlobalClimate::season` cycles; `base_temperature` and `humidity` vary sinusoidally in `climate.rs`.

## Lifetime

`Creature::age` increments each tick. Memory graph grows over lifetime; death (planned) ends graph updates.

## Generational Time

Reproduction interval links to [13_inheritance.md](13_inheritance.md).

## Cultural Time

Slower than lifetime — concepts stabilize across populations ([16_culture.md](16_culture.md)).

## Infrastructure Time

Construction and erosion modify landscape on intermediate scales ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)).

## Landscape Time

Channels, trails, organic deposits — voxel memory ([01_world_generation.md](01_world_generation.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md)).

## Geological Time

Terrain generation, cave feedback, long erosion — target of performance architecture ([18_performance_architecture.md](18_performance_architecture.md)).

## Temporal Asymmetry

Past is stored (memory, landscape); future is predicted (planned). Actions bridge present to both.

## Delayed Consequences

`delay_mean` / `delay_variance` on memory edges encode that outcomes may arrive many ticks after actions.

## Persistence

Each scale has characteristic decay: experiences capped at 64; edges weaken without reinforcement; voxel fields persist until physics changes them.

## Time and Meaning

Meaning accrues on **slower** scales — a trace matters when it repeatedly changes regulatory predictions across many events.

## Time and Culture

Culture requires temporal depth beyond individual lifetime — inheritance + communication + landscape ([13_inheritance.md](13_inheritance.md)).

## Time and Geography

**Physical time** — world ticks, erosion, seasons.  
**Cognitive time** — memory consolidation, concept drift, prediction horizons.  
They run on coupled but distinct clocks.

## Core Principle

> The world forgets differently than the mind. Geography is slow memory; experience is fast memory.

## Current implementation

| Signal | Location |
|--------|----------|
| `World::time` | `world/mod.rs` |
| `TICKS_PER_DAY` / `day_phase` | `world/mod.rs`, diurnal light in `sensors.rs` |
| `season` | `GlobalClimate` |
| `Creature::age` | `creature.rs` |
| Tick loop | `simulation/engine.rs` |
| Climate/water every tick (active chunks) | `world/mod.rs` `tick_climate_and_water` |
| Erosion placeholder every N ticks | `world/mod.rs` `tick_erosion`, `scheduler.rs` config |

## Open questions

- Fixed tick = fixed "day" or abstract time unit?
- When to run sleep relative to tick batch?
