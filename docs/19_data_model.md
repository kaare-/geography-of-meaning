# Data Model

> **Guiding question:** What runtime structures carry world, body, sensation, and memory?

## Status

**Partial** — skeleton implements core structs; many fields planned.

## Summary

This document defines the core runtime data structures of the simulation.

The goal is to keep the architecture data-oriented, inspectable, serializable, and scalable.

The simulation contains four broad categories of state:

* objective world state
* embodied creature state
* subjective sensor state
* relational memory state

## Core Principle

World state is **objective**.

Creature state is **embodied**.

Sensor state is **subjective**.

Memory state is **relational**.

Creatures never read world variables directly — only sensor traces and their own internal state. See [05_sensors.md](05_sensors.md).

## World

The world contains all physical reality.

Creatures do not access world state directly. They access it only through sensors.

**Code:** `sim-core/src/world/mod.rs` — `World`

### World Fields

| Field | Status | Code |
|-------|--------|------|
| `chunks` | Implemented | `World::chunks: HashMap<ChunkCoord, Chunk>` |
| `active_chunks` | Implemented | `World::active_chunks: HashSet<ChunkCoord>` |
| `global time` | Implemented | `World::time: u64` |
| `climate state` | Implemented | `World::climate: GlobalClimate`, `World::season` |
| `event queue` | Implemented | `World::event_queue: Vec<WorldEvent>` |
| `sound events` | Planned | — |
| `creature registry` | Planned | Creatures live on `Simulation`, not `World` |

See [01_world_generation.md](01_world_generation.md), [02_physics_and_materials.md](02_physics_and_materials.md).

## Chunk

The world is divided into chunks.

Suggested size: **16 × 16 × 16** voxels (`CHUNK_SIZE` in `sim-core/src/world/voxel.rs`).

Chunks allow:

* efficient storage
* active-region updates
* parallel processing
* future streaming

**Code:** `sim-core/src/world/chunk.rs` — `Chunk`

### Chunk Fields

| Field | Status | Code |
|-------|--------|------|
| `chunk coordinate` | Implemented | `Chunk::coord: ChunkCoord` |
| `voxel fields` | Implemented | `Chunk::fields: VoxelFields` |
| `active flag` | Planned | Activation tracked at world level via `active_chunks` |
| `dirty flag` | Planned | — |
| `local event list` | Planned | — |

## Voxel Fields

Use structure-of-arrays where possible.

**Code:** `sim-core/src/world/voxel.rs` — `VoxelFields`

Each chunk stores arrays for:

| Field | Status |
|-------|--------|
| `hard_mineral` | Implemented |
| `soft_mineral` | Implemented |
| `coarse_mineral` | Implemented |
| `clay` | Implemented |
| `organic` | Implemented |
| `binder` | Implemented |
| `solid_fraction` | Implemented |
| `void_fraction` | Implemented |
| `surface_water` | Implemented |
| `water_content` | Implemented |
| `ice` | Implemented |
| `snow` | Implemented |
| `temperature` | Implemented |
| `humidity` | Implemented |
| `porosity` | Implemented |
| `permeability` | Implemented |
| `erosion_damage` | Implemented |
| `structural_strength` | Implemented |
| `load` | Implemented |

Voxels are not objects. They are indexed positions in field arrays (`idx(x, y, z)`). Access via `VoxelView` / `VoxelViewMut`.

## Creature

A creature is a physical organism with internal regulation, sensors, memory, and action capability.

**Code:** `sim-core/src/creatures/creature.rs` — `Creature`

### Creature Fields

| Field | Status | Code |
|-------|--------|------|
| `id` | Implemented | `Creature::id: u64` |
| `position` | Implemented | `Creature::position: Vec3f` |
| `morphology` | Implemented | `Creature::morphology: Morphology` |
| `genome` | Implemented | `Creature::genome: Genome` |
| `regulatory_state` | Implemented | `Creature::regulatory: RegulatoryState` |
| `sensor_state` | Implemented | `Creature::sensor: SensorState` |
| `active_concepts` | Planned | `concept_nodes` is a placeholder list only |
| `memory_graph` | Implemented | `Creature::memory_graph: MemoryGraph` |
| `recent_experience_buffer` | Implemented | `Creature::recent_experience: Vec<Experience>` |
| `signature` | Implemented | `Creature::signature: u64` |
| `age` | Implemented | `Creature::age: u32` |
| `alive flag` | Planned | All creatures always active in skeleton |

See [03_creatures.md](03_creatures.md).

## Morphology

**Implemented** — `Morphology` in `sim-core/src/creatures/morphology.rs` defines physical tradeoffs.

| Field | Role |
|-------|------|
| `mass` | Metabolism multiplier, push strength, thermal coupling |
| `reserve_capacity` | Energy ceiling (`RegulatoryState::clamp`) |
| `heat_retention` | Thermal inertia with mass |
| `carry_capacity` | Max `carried_mass` for carry/drop actions |

Derived from genome at spawn (`Morphology::from_genome`); mutated on reproduction (`mutate_from`). `push_strength()` is computed from mass (not stored). Exported in `CreatureSnapshot`. See [03_creatures.md](03_creatures.md).

## Genome

Genome defines inherited tendencies. It does not contain concepts or knowledge.

**Code:** `sim-core/src/creatures/genome.rs` — `Genome`

| Field (spec) | Status | Code |
|--------------|--------|------|
| sensor sensitivity | Planned | — |
| sensor noise | Implemented | `sensor_noise_scale` |
| movement efficiency | Implemented | `move_speed` |
| metabolism | Implemented | `metabolism_rate` |
| mass bias | Implemented | `mass_bias` |
| heat retention | Implemented | `heat_retention` |
| carry bias | Implemented | `carry_bias` |
| reserve bias | Implemented | `reserve_bias` |
| learning rate | Planned | — |
| memory capacity | Planned | — |
| sleep tendency | Planned | — |
| vocal ability | Planned | — |
| binder production | Planned | — |

See [13_inheritance.md](13_inheritance.md).

## Regulatory State

The organism regulates internal variables. Prediction concerns future regulatory state.

**Code:** `sim-core/src/creatures/regulation.rs` — `RegulatoryState`

| Field | Status |
|-------|--------|
| `energy` | Implemented |
| `hydration` | Implemented |
| `temperature_stress` | Implemented |
| `fatigue` | Implemented |
| `integrity` | Implemented |
| `carried_mass` | Implemented |

See [04_regulation.md](04_regulation.md).

## Sensor State

Sensor state is the creature's subjective interface to the world.

**Code:** `sim-core/src/creatures/sensors.rs` — `SensorState`

Sensor values are noisy traces. They are not direct world variables.

| Channel | Status |
|---------|--------|
| `light` | Implemented |
| `thermal` | Implemented |
| `chemical_organic` | Implemented |
| `chemical_wet_mineral` | Implemented |
| `chemical_decay` | Implemented |
| `chemical_binder` | Implemented |
| `chemical_creature` | Implemented (always 0.0 in skeleton) |
| `sound_ambient` | Implemented |
| `sound_calls` | Implemented (0.0 in skeleton) |
| `contact_hard` | Implemented |
| `contact_soft` | Implemented |
| `contact_occupied` | Implemented (0.0 in skeleton) |
| `internal_energy` | Implemented |
| `internal_hydration` | Implemented |
| `internal_temperature_stress` | Implemented |
| `internal_fatigue` | Planned | Not yet a dedicated channel; fatigue exists in `RegulatoryState` |
| `internal_integrity` | Planned | Not yet a dedicated channel; integrity exists in `RegulatoryState` |

Skeleton exposes **15** channels via `SensorState::as_vector()`. Full spec targets **17** internal+external channels. See [05_sensors.md](05_sensors.md).

## Experience

An experience records a transition. Experiences are temporary until consolidated.

**Code:** `sim-core/src/creatures/creature.rs` — `Experience`

| Field | Status | Code |
|-------|--------|------|
| `timestamp` | Implemented | `Experience::timestamp` |
| `sensory_before` | Implemented | `Experience::sensory_before` |
| `internal_state_before` | Implemented | `Experience::state_before: RegulatoryState` |
| `action` | Implemented | `Experience::action` |
| `sensory_after` | Implemented | `Experience::sensory_after` |
| `internal_state_after` | Implemented | `Experience::state_after` |
| `outcome` | Implemented | `Experience::outcome` (energy delta placeholder) |
| `nearby_sender_signature` | Planned | — |
| `heard_sounds` | Planned | — |

Recent buffer capped at 64 (`MAX_RECENT_EXPERIENCE`). See [06_memory.md](06_memory.md).

## Memory Graph

Memory is a directed graph. It stores relationships, not facts.

**Code:** `sim-core/src/memory/graph.rs` — `MemoryGraph { nodes, edges }`

Direction is essential for prediction.

### Memory Nodes

**Code:** `sim-core/src/memory/nodes.rs` — `NodeKind`

| Type (spec) | Status | Code |
|-------------|--------|------|
| `sensory_pattern` | Implemented | `NodeKind::SensoryPattern(SensorState)` |
| `action` | Implemented | `NodeKind::Action(Action)` |
| `outcome` | Implemented | `NodeKind::Outcome(f32)` |
| `sound` | Stub | `NodeKind::Sound(f32)` |
| `concept` | Stub | `NodeKind::Concept` |

### Memory Edges

**Code:** `sim-core/src/memory/edges.rs` — `EdgeType`, `MemoryEdge`

| Type (spec) | Status |
|-------------|--------|
| `co_occurs` | Implemented |
| `precedes` | Implemented |
| `action_leads_to` | Implemented |
| `sound_activates` | Planned |
| `concept_compresses` | Planned |

### Edge Fields

| Field | Status |
|-------|--------|
| `source_id` | Implemented |
| `target_id` | Implemented |
| `edge_type` | Implemented |
| `strength` | Implemented |
| `confidence` | Implemented |
| `observations` | Implemented |
| `delay_mean` | Implemented |
| `delay_variance` | Implemented |

## Concept Node

A concept is a compressed predictive region of memory space.

**Code:** `sim-core/src/memory/concepts.rs` — `ConceptNode` (placeholder)

| Field | Status |
|-------|--------|
| `id` | Implemented |
| `prototype_vector` | Planned |
| `member_nodes` | Planned |
| `associated_actions` | Planned |
| `associated_sounds` | Planned |
| `expected_outcomes` | Planned |
| `strength` | Planned |
| `confidence` | Planned |
| `age` | Planned |

Concept labels are researcher-facing only. The creature only uses concept IDs and activation strengths. See [07_concepts.md](07_concepts.md).

## Active Concepts

**Planned** — active concepts are concepts currently triggered by sensor input or spreading activation.

Fields:

* `concept_id`
* `activation`
* `source`
* `decay_rate`

Possible sources: sensor, memory, sound, sleep.

No `ActiveConcept` struct in skeleton. See [07_concepts.md](07_concepts.md), [08_prediction.md](08_prediction.md).

## Sound Event

**Planned** — sound events are temporary world events.

Fields:

* `position`
* `emitter_id`
* `sender_signature`
* `pattern`
* `amplitude`
* `frequency_profile`
* `rhythm`
* `duration`
* `age`

Sound is interpreted through memory, not attached at emission. See [10_communication.md](10_communication.md).

## Simulation Event

Generic event type for delayed processing.

**Code:** `sim-core/src/world/event.rs` — `WorldEvent`

| Example | Status |
|---------|--------|
| rainfall | Implemented (`WorldEvent::Rain`) |
| collapse | Planned |
| sound | Planned |
| birth | Planned |
| death | Planned |
| sleep_start / sleep_end | Planned |

## Serialization

Important data should be exportable.

**Code:** `sim-core/src/export/snapshots.rs`, `sim-core/src/export/logs.rs`

| Export target | Status |
|---------------|--------|
| world snapshots | Implemented (`WorldSnapshot`, `ChunkSnapshot`) |
| creature states | Implemented (`CreatureSnapshot`) |
| sensor logs | Partial (sensor embedded in tick log) |
| memory graphs | Planned (counts only in snapshot) |
| concept histories | Planned |
| sound histories | Planned |
| lineage histories | Planned |

See [17_analysis_and_visualization.md](17_analysis_and_visualization.md).

## Researcher Labels

The simulation should support generated diagnostic labels.

Example:

```text
C-04821 | thermal high / wind low / hard contact | action:rest | outcome:reduced cold stress
```

Labels are analysis overlays. They are not creature cognition. Forbidden as creature-facing types: `food`, `shelter`, `wall`, `river`, `cave`, `mountain`, etc.

## Current implementation

| Component | Location |
|-----------|----------|
| `World`, `Chunk`, `VoxelFields` | `sim-core/src/world/` |
| `Creature`, `Experience` | `sim-core/src/creatures/creature.rs` |
| `RegulatoryState` | `sim-core/src/creatures/regulation.rs` |
| `Genome` | `sim-core/src/creatures/genome.rs` |
| `SensorState` | `sim-core/src/creatures/sensors.rs` |
| `MemoryGraph`, nodes, edges | `sim-core/src/memory/` |
| `ConceptNode` placeholder | `sim-core/src/memory/concepts.rs` |
| `WorldEvent` | `sim-core/src/world/event.rs` |
| Export types | `sim-core/src/export/` |

## Planned

- `ActiveConcept` activation layer
- `SoundEvent` type and world sound queue
- Full `SimulationEvent` enum (birth, death, sleep, collapse)
- Creature registry on world or simulation coordinator
- Chunk dirty flags and local event lists
- `internal_fatigue` and `internal_integrity` sensor channels
- Full memory graph and concept JSON export
- Experience fields: `nearby_sender_signature`, `heard_sounds`
- `alive` flag and death state on `Creature`

## Core Principle

The data model must preserve the distinction between **reality**, **sensation**, **memory**, **prediction**, and **interpretation**.
