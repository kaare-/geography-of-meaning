# Research Tools and Observability

> **Guiding question:** How do researchers inspect what emerged?

## Status

**Partial** — JSON snapshot export, per-tick JSONL logs, and a Python loader/summary script are implemented. Dedicated viewers, GraphML export, historical timelines, narrative extraction, and real-time inspection are **planned**. See [17_analysis_and_visualization.md](17_analysis_and_visualization.md), [19_data_model.md](19_data_model.md).

## Summary

The simulation is also a research instrument.

Without observability, emergent behavior cannot be understood.

The goal of this document is to define tools that allow researchers to inspect:

* organisms
* concepts
* memories
* communication
* infrastructure
* culture
* history

## Core Principle

Every important process should be observable.

Not necessarily in real time.

But retrospectively.

## Creature Inspector

Selecting a creature should reveal:

* age
* morphology
* regulation
* active concepts
* current predictions
* recent experiences

The goal is understanding subjective state.

**Skeleton:** `CreatureSnapshot` in `export/snapshots.rs` exports `id`, `position`, regulatory scalars (`energy`, `hydration`, `temperature_stress`, `integrity`, `fatigue`), `age`, full `sensor` state, and `memory_nodes` / `memory_edges` counts. Morphology, active concepts, predictions, and experience history are **planned**.

## Sensor Inspector

Displays current sensory traces.

Examples:

* chemical gradients
* thermal gradients
* sound activity
* contact state

Allows comparison between world state and perceived state.

**Skeleton:** `CreatureSnapshot.sensor` serializes the 15-channel `SensorState` from the final tick. Per-tick sensor history is available in `tick_log.jsonl` via repeated `CreatureSnapshot` entries. No side-by-side world-voxel vs sensor comparison UI — **planned**.

## Memory Graph Viewer

Displays:

* nodes
* edges
* confidence
* activation

Allows inspection of:

* concept formation
* prediction chains
* communication effects

**Skeleton:** exports report `memory_nodes` and `memory_edges` counts only. Full graph serialization (GraphML or nested JSON) is **planned** ([06_memory.md](06_memory.md)).

## Concept Viewer

Displays:

* concept age
* constituent memories
* activation frequency
* expected outcomes

Research labels may be generated automatically.

Example:

```
Warm / Hard / Low Wind Cluster
```

rather than:

```
Concept #2481
```

**Planned** — `concept_nodes` exist on `Creature` but are not exported ([07_concepts.md](07_concepts.md)).

## Prediction Viewer

Shows:

* current active predictions
* expected outcomes
* confidence

This allows researchers to inspect decision-making.

**Planned** — prediction system is stub ([08_prediction.md](08_prediction.md)). Tick logs capture regulatory deltas as implicit outcomes.

## Communication Viewer

Displays:

* sound events
* sender
* receiver
* resulting activations

Allows analysis of communication evolution.

**Planned** — no sound events or communication channel in exports ([10_communication.md](10_communication.md)).

## Settlement Viewer

Displays:

* population
* infrastructure
* age
* activity

Allows study of settlement formation.

**Planned** — no settlement entities ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md)). Researcher label **settlement** applies only in analysis.

## Infrastructure Viewer

Displays:

* paths
* walls
* shelters
* caches
* channels

Infrastructure should be visible as external memory.

**Planned** — world snapshots export 2D voxel slices (`organic`, `surface_water`, `temperature`, `solid_fraction`) per chunk; no infrastructure overlay ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md)). Researcher labels (shelter, wall, cache) apply only in analysis.

## Migration Viewer

Tracks:

* movement
* expansion
* population shifts

Supports historical analysis.

**Skeleton:** `CreatureSnapshot.position` at each tick in `tick_log.jsonl` allows post-hoc trajectory reconstruction in Python. No dedicated migration map or viewer — **planned**.

## Lineage Viewer

Displays:

* parent-child relationships
* population branching
* evolutionary divergence

Supports inheritance research.

**Planned** — no parent links or reproduction in runtime ([13_inheritance.md](13_inheritance.md), [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md)).

## Historical Timeline

The simulation should record major events.

Examples (researcher labels):

* settlement founded
* settlement abandoned
* population collapse
* migration wave
* communication divergence

The world develops a historical record.

**Planned** — see [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md). Current exports capture implicit history via voxel and creature state snapshots, not event streams.

## Concept Genealogy

Tracks:

* concept formation
* concept merging
* concept splitting

Allows study of cognitive evolution.

**Planned** ([07_concepts.md](07_concepts.md)).

## Cultural Viewer

Displays:

* traditions
* recurring behaviors
* communication clusters
* infrastructure practices

Culture becomes inspectable.

**Planned** ([16_culture.md](16_culture.md)).

## World History Viewer

Allows exploration of different historical periods.

The researcher may move backward and forward through time.

The simulation becomes archaeological.

**Planned** — requires multi-checkpoint exports or full tick replay. Single end-state snapshot and JSONL log are the current substrate ([24_world_history_and_archaeology.md](24_world_history_and_archaeology.md)).

## Narrative Extraction

The system should support automatic extraction of:

* individual biographies
* settlement histories
* migration stories
* cultural histories

These become valuable artistic outputs.

**Planned** — may build on tick logs, memory graph exports, and historical event timelines.

## Research Exports

Export formats:

| Format | Status |
|--------|--------|
| JSON (world snapshot) | Implemented — `exports/snapshots/world_final.json` |
| JSONL (tick log) | Implemented — `exports/logs/tick_log.jsonl` |
| CSV | Planned |
| GraphML | Planned |
| Images | Partial — `load_snapshot.py --plot` |
| Video | Planned |
| Timelines | Planned |

The simulation should support both scientific and artistic analysis.

### Current export pipeline

```
Simulation::run()
  → export_all(sim, output_dir)
    → write_snapshot → WorldSnapshot::from_simulation
    → write_tick_log → TickLogEntry per tick
```

Implemented in `sim-core/src/export/mod.rs`, `snapshots.rs`, `logs.rs`.

### Python tools

```bash
python analysis/scripts/load_snapshot.py exports/snapshots/world_final.json
python analysis/scripts/load_snapshot.py --plot organic
```

`load_snapshot.py` summarizes time, season, chunk count, creature count, and optionally plots a 2D field slice.

## Current implementation

| Tool | Location | Status |
|------|----------|--------|
| `WorldSnapshot` / `CreatureSnapshot` | `export/snapshots.rs` | Memory node counts by type, concept count, active concepts |
| `TickLogEntry` | `export/logs.rs` | Births, deaths, `sound_event_count`, optional sound slice |
| `export_all()` | `export/mod.rs` | Implemented |
| `load_snapshot.py` | `analysis/scripts/` | Snapshot + tick-log population/births/deaths |
| Creature inspector UI | — | Planned |
| Memory graph viewer | — | Planned |
| All specialized viewers | — | Planned |
| GraphML / CSV / timeline exports | — | Planned |
| Real-time inspection | — | Planned |
| Narrative extraction | — | Planned |

## Planned

- Per-creature memory graph JSON / GraphML export
- Concept and prediction inspection endpoints
- Communication and sound event logging
- Settlement and infrastructure overlays on world slices
- Lineage and genealogy tracking post-reproduction
- Historical event timeline in exports
- Multi-checkpoint snapshots for time-travel archaeology
- Jupyter notebooks in `analysis/notebooks/`
- Narrative extraction pipeline for biographies and migration stories — see [27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md)

## Cross-references

| Topic | Doc |
|-------|-----|
| Analysis overview | [17_analysis_and_visualization.md](17_analysis_and_visualization.md) |
| Serialization & DTOs | [19_data_model.md](19_data_model.md) |
| World history | [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md) |
| Narrative extraction | [27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md) |
| Information storage | [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md) |
| Memory | [06_memory.md](06_memory.md) |

## Core Principle

Emergence is only useful if it can be observed.

The simulation should not merely generate complexity.

It should allow complexity to be understood.
