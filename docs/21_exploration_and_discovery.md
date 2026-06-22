# Exploration and Discovery

> **Guiding question:** How does information about the wider world enter predictive models?

## Status

**Planned / Partial** — mostly conceptual. Skeleton provides random/weighted action selection, `Experience.outcome` as prediction-error placeholder, genome movement bias, sensor gradients, and memory similarity retrieval. No explicit curiosity variable.

## Summary

The simulation does not begin with a populated world.

It begins with a small number of organisms occupying a favorable environment.

The challenge is understanding how information about the wider world is discovered.

Exploration is the process through which organisms encounter information not already present within their predictive models.

## Core Principle

An organism cannot discover a new future without leaving a known one.

Exploration is therefore a tradeoff between:

**certainty**

and

**possibility**.

## The Problem

Local optimization is often insufficient.

An organism may occupy:

* a warm valley
* abundant organic gradients (researcher label: food)
* reliable moisture traces (researcher label: water)

The immediate gradients all point toward remaining there.

Yet a more favorable environment may exist elsewhere.

Exploration therefore requires mechanisms that overcome local optimization.

## Exploration Pressure

Exploration emerges from:

* crowding
* scarcity
* competition
* environmental change
* prediction uncertainty

These pressures increase the likelihood of movement into unfamiliar regions.

**Skeleton:** `choose_action()` in `sim-core/src/creatures/actions.rs` biases toward `ConsumeOrganic` and `Rest` when energy is low and toward `Rest` when fatigue is high — a minimal regulatory pressure, not full exploration dynamics.

## Information Horizon

Every organism possesses an information horizon.

Beyond this horizon:

prediction confidence decreases.

Unknown regions become increasingly uncertain.

The information horizon expands through experience.

**Skeleton:** `MemoryGraph::find_similar_sensory()` returns matches only for previously visited sensor patterns. Unfamiliar neighborhoods produce novel `SensoryPattern` nodes — implicit horizon boundary at the edge of lived experience.

## Curiosity

Curiosity should **not** be implemented directly.

Instead it emerges from:

prediction uncertainty + potential regulatory improvement

An organism may investigate because uncertainty itself possesses predictive value.

**Skeleton:** no `curiosity` field or dedicated exploration drive. Random move weight in `choose_action()` provides incidental exploration only.

## Imagination

Imagination allows exploration before movement.

Example (researcher labels):

```text
small elevation change → descent → moisture gradient
```

many similar experiences become

```text
elevation change → possible descent → possible moisture
```

The future is extrapolated from previous patterns.

The prediction may be wrong.

Exploration tests the prediction.

**Planned** — graph traversal and offline activation without sensing. See [08_prediction.md](08_prediction.md), [09_sleep.md](09_sleep.md).

## Discovery

Discovery occurs when:

experience differs from expectation.

Prediction error generates new information.

Discovery is therefore a consequence of failed prediction.

**Skeleton:** `Experience::outcome` is currently `energy_after - energy_before` — a scalar prediction-error placeholder. Large absolute values signal unexpected regulatory change. See [04_regulation.md](04_regulation.md), [06_memory.md](06_memory.md).

## Scouts vs Settlers

### Scouts

Some morphologies may naturally favor exploration.

Examples:

* small bodies
* low reserves
* high mobility

These organisms effectively predict: **movement is valuable**.

### Settlers

Other morphologies may favor stability.

Examples:

* large bodies
* large reserves
* infrastructure investment

These organisms effectively predict: **place is valuable**.

Exploration and settlement become complementary strategies.

**Planned** — `Morphology` struct with mass, reserve capacity, movement efficiency. See [03_creatures.md](03_creatures.md), [19_data_model.md](19_data_model.md). Genome `move_speed` is the only movement-related inherited trait in skeleton.

## Environmental Information

The world contains information before it is discovered.

Examples (researcher labels only):

* hidden void spaces (cave)
* groundwater systems
* fertile valleys
* migration routes

Information exists independently of organisms.

Meaning does not.

Meaning emerges when information enters prediction.

**Skeleton:** `World::find_spawn_positions()` scores organic, water, temperature, and void fraction — favorable patches exist in the voxel field before any creature acts. Creatures sense only traces via `read_sensors()`.

## Communication of Discovery

Discovery becomes more powerful when shared.

Examples (researcher labels):

* moisture source discovered
* organic gradient discovered
* sheltered void discovered

Communication extends discovery beyond the discoverer.

**Planned** — sound events and signature-linked memory. See [10_communication.md](10_communication.md).

## Infrastructure and Discovery

Exploration often creates infrastructure.

Examples:

* trails
* bridges
* camps
* markers

Discovery leaves traces.

Future organisms encounter these traces.

**Planned** — landscape modification and external memory. See [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).

## Collective Discovery

Groups may discover environments more effectively than individuals.

Information becomes distributed across many organisms.

Collective memory exceeds individual memory.

**Planned** — population-scale analysis and overlapping memory graphs. See [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md).

## Current implementation

| Mechanism | Location | Role |
|-----------|----------|------|
| Weighted random action selection | `creatures/actions.rs` | Incidental movement |
| `read_sensors()` neighborhood sampling | `creatures/sensors.rs` | Local gradient traces |
| `Experience.outcome` | `creatures/creature.rs` | Prediction error scalar |
| `MemoryGraph::find_similar_sensory` | `memory/graph.rs` | Familiar vs novel patterns |
| `Genome::move_speed` | `creatures/genome.rs` | Movement inheritance (unused in selection) |
| Spawn position scoring | `world/mod.rs` | Pre-existing favorable patches |

## Planned

- Prediction-driven exploration (uncertainty-weighted action selection)
- Imagination / graph extrapolation before movement
- Morphology-based scout vs settler tradeoffs
- Communication of discovered patterns
- Infrastructure traces that alter future sensor readings
- Collective discovery metrics in analysis export

## Core Principle

Exploration is the process through which information enters the predictive system.

Without exploration there can be memory, but no novelty.
