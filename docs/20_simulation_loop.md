# Simulation Loop

> **Guiding question:** How do world, body, memory, and prediction advance each tick?

## Status

**Partial** — skeleton implements a minimal world→creature→memory tick; most cognitive and lifecycle stages are planned.

## Summary

The simulation loop defines how the world, organisms, sensors, memory, and prediction update over time.

The loop is not only a physics update. It is the continuous negotiation between objective world state and subjective predictive models.

## Core Principle

The world changes.

Sensors sample traces.

Memory activates.

Predictions form.

Actions modify the world.

The modified world changes future experience.

## High-Level Loop

Each tick (target order):

| Step | Status |
|------|--------|
| Advance time | Implemented |
| Update climate and weather | Implemented |
| Update active chunks | Partial (creature presence + water activity) |
| Update water, temperature, and chemical fields | Partial (water + temperature; chemicals static) |
| Process sound events | Planned |
| Update creature sensors | Implemented |
| Activate concepts | Planned |
| Spread activation | Planned |
| Predict outcomes | Planned |
| Select actions | Implemented |
| Apply actions | Implemented |
| Store experiences | Implemented |
| Check sleep, reproduction, and death | Partial (death on regulatory failure) |
| Export logs if needed | Partial (in-memory tick log; file export at run end) |

**Code:** `sim-core/src/simulation/engine.rs` — `Simulation::tick()`

### Skeleton tick order (actual)

```text
1. Roll rainfall → queue WorldEvent::Rain (humidity-modulated rate)
2. world.process_events()
3. world.tick_climate_and_water()   // advances world.time, day_phase
4. For each creature:
     a. Save sensory_before, state_before
     b. regulatory.tick_passive_drain()
     c. choose_action()
     d. apply_action()
     e. read_sensors()
     f. regulatory.clamp(); age += 1
     g. If energy <= 0 or integrity <= 0: deposit organic, record death, remove
     h. Else: Build Experience; record_experience(); push_experience()
5. world.refresh_active_chunks() from creature positions + water activity
6. Append TickLogEntry (deaths, day_phase) to tick_logs
```

Post-run export in `main.rs` via `export_all()` — not per-tick.

See [19_data_model.md](19_data_model.md) for struct definitions.

## Time Advancement

The simulation maintains global time.

Time tracks:

| Scale | Status | Code |
|-------|--------|------|
| tick | Implemented | `World::time` increments in `tick_climate_and_water()` |
| day phase | Partial | `World::day_phase` (`time % TICKS_PER_DAY`) |
| season | Partial | `World::season`, `GlobalClimate::season` |
| year | Planned | — |
| generation count | Planned | — |

Not all systems update every tick. See [14_time_and_scales.md](14_time_and_scales.md).

## Climate Update

Climate controls:

| Driver | Status |
|--------|--------|
| sunlight | Planned |
| temperature | Implemented (`GlobalClimate::base_temperature`, seasonal offset) |
| rainfall | Implemented (`rainfall_rate`, `WorldEvent::Rain`) |
| snow | Partial (fields exist; limited tick logic) |
| wind | Planned |
| humidity | Implemented (`GlobalClimate::humidity`) |

Climate may activate chunks by changing local conditions — planned. Current skeleton updates all `active_chunks` every tick.

**Code:** `sim-core/src/world/climate.rs`, `World::tick_climate_and_water()`

## Active Chunk Update

Only active chunks should eventually update.

A chunk may become active because of:

* water movement
* creature presence
* construction
* erosion
* collapse
* temperature transition
* organic growth

Inactive chunks remain dormant.

**Current:** `active_chunks` is populated at terrain generation and expanded each tick when creatures occupy a chunk or water activity exceeds threshold. All active chunks receive climate/water updates. See [18_performance_architecture.md](18_performance_architecture.md).

## Field Updates

Fields update at different rates.

| Rate | Fields | Status |
|------|--------|--------|
| Fast | sound events, creature positions, contact | Partial (positions + contact via actions/sensors) |
| Medium | surface water, temperature, chemical traces | Partial (water + temperature per tick) |
| Slow | erosion, deposition, groundwater, organic growth, decay | Planned |
| Very slow | landscape change, cave formation, cultural drift | Planned |

## Sound Events

**Planned** — sound events are processed as temporary signals.

Sources include: calls, movement, digging, water flow, collapse, conflict.

Creatures receive sound through sensors (`sound_ambient`, `sound_calls`). No meaning is attached at this stage. See [10_communication.md](10_communication.md).

## Creature Sensor Update

Each creature samples the world through sensors.

Sensors produce subjective readings. The creature does not receive:

* material labels
* object identity
* coordinates
* exact resource values

Only sensor traces.

**Code:** `read_sensors()` in `sim-core/src/creatures/sensors.rs` — 3×3×3 neighborhood sampling with Gaussian noise.

See [05_sensors.md](05_sensors.md).

## Active Concept Update

**Planned** — sensor readings activate matching memory and concept regions.

Example (researcher label only): cool air + hard contact + echoing sound may activate a cave-like concept cluster. The creature does not know the word *cave*.

See [07_concepts.md](07_concepts.md).

## Spreading Activation

**Planned** — activation spreads through the memory graph.

Example: cave-like concept → storage concept → winter concept → energy prediction.

This is the basis for thought-like processes. See [08_prediction.md](08_prediction.md), [06_memory.md](06_memory.md).

## Prediction

**Planned** — predictions are generated by graph traversal.

Predictions estimate future **regulatory** state. The creature is not predicting the world in general — it is predicting what the world may do to its regulation.

See [04_regulation.md](04_regulation.md), [08_prediction.md](08_prediction.md).

## Action Selection

Actions are selected according to expected regulatory consequences (target). Skeleton uses weighted random selection biased by low energy and high fatigue.

**Implemented actions:** `Move`, `ConsumeOrganic`, `Rest` in `sim-core/src/creatures/actions.rs`.

**Planned primitive actions:** drink, carry, drop, dig, place material, bind material, emit sound, follow, push.

No high-level actions exist. See [03_creatures.md](03_creatures.md).

## Apply Actions

Actions modify creature state, world state, event queue, and memory buffer.

| Action | Effect (skeleton) |
|--------|-------------------|
| Move | Position change if `void_fraction > 0.4`; energy/fatigue cost |
| ConsumeOrganic | Reduces local organic; increases energy |
| Rest | Reduces fatigue; slight energy gain |

**Code:** `apply_action()` in `sim-core/src/creatures/actions.rs`

Planned: digging, sound emission, material placement, pushing — each with regulatory and world consequences.

## Experience Storage

After action, the creature stores an experience.

Experience includes: sensory before, internal state before, action, sensory after, internal state after, outcome.

Experiences are later consolidated during sleep (planned).

**Code:** `Experience` in `creature.rs`; `MemoryGraph::record_experience()` in `graph.rs`.

`outcome` is currently `energy_after - energy_before` — a prediction-error placeholder. See [06_memory.md](06_memory.md), [04_regulation.md](04_regulation.md).

## Sleep Check

**Planned** — sleep may begin depending on fatigue, light, temperature, safety, social context.

During sleep: action reduced, sensory input reduced, memory consolidation (`consolidate_sleep()` stub), concepts may form or change.

See [09_sleep.md](09_sleep.md).

## Reproduction Check

**Planned** — reproduction when energy sufficient, regulatory state stable, environmental conditions permit.

Offspring inherit genome, biases, environment, social context. Direct memory copying should be avoided.

See [13_inheritance.md](13_inheritance.md).

## Death Check

**Implemented** — death through regulatory failure (`energy <= 0`, `integrity <= 0`). Organic matter deposited to voxel on death. See `creatures/lifecycle.rs`.

## Export

The simulation should periodically export:

| Target | Status |
|--------|--------|
| world snapshots | Implemented (end of run) |
| creature positions | Implemented |
| regulatory states | Implemented |
| concept activations | Planned |
| memory changes | Partial (memory_node_count per creature in snapshots) |
| sound events | Planned |
| construction events | Planned |

**Code:** `export_all()` → `world_final.json`, `tick_log.jsonl`. See [17_analysis_and_visualization.md](17_analysis_and_visualization.md).

## Multi-Rate Scheduling

Different systems update at different frequencies.

| Interval | Systems | Status |
|----------|---------|--------|
| Every tick | sound, creature sensing, action selection | Partial |
| Every few ticks | surface water, temperature, chemical gradients | Partial (all per tick today) |
| Every day | sleep consolidation | Planned |
| Every season | climate shift, growth cycle | Partial (season drifts every tick) |
| Long interval | erosion, cave formation, cultural analysis | Planned |

**Code:** `sim-core/src/simulation/scheduler.rs` holds `SimulationConfig` only; no multi-rate scheduler yet. See [14_time_and_scales.md](14_time_and_scales.md).

## Performance Principle

Do not simulate everything everywhere all the time.

Use:

* active chunks
* event queues
* multi-rate updates
* sparse creature processing
* periodic exports

See [18_performance_architecture.md](18_performance_architecture.md).

## Current implementation

| Component | Location |
|-----------|----------|
| `Simulation::tick`, `Simulation::run` | `sim-core/src/simulation/engine.rs` |
| Death + organic recycling | `sim-core/src/creatures/lifecycle.rs` |
| Reproduction + birth log | `lifecycle.rs`, `export/logs.rs` |
| Sleep cycle | `creature.rs`, `engine.rs` |
| Prediction bias | `memory/graph.rs`, `creatures/actions.rs` |
| Tick order | `engine.rs` — sleep/imagination → active concepts → predict → action |
| `SimulationConfig` (multi-rate) | `sim-core/src/simulation/scheduler.rs` |
| World climate/water/events/active chunks | `sim-core/src/world/mod.rs` |
| Creature loop | `engine.rs` + `creatures/` |
| Tick log (deaths, births, day_phase, sleep) | `sim-core/src/export/logs.rs` |
| CLI + export | `sim-core/src/main.rs`, `export/mod.rs` |

## Planned

- `SimulationEvent` queue for delayed effects

## Core Principle

The simulation loop is the place where physical reality and predictive models continually correct one another.
