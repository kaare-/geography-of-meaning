# World History and Archaeology

> **Guiding question:** How does the past remain present in traces — and how do organisms and researchers read it?

## Status

**Planned / Partial** — voxel fields `erosion_damage`, `porosity`, `binder`, and `organic` exist in `sim-core/src/world/voxel.rs` but carry no tick-updated history logic. Environmental memory, erosion feedback, and construction persistence are **planned** ([01_world_generation.md](01_world_generation.md)). No archaeology-specific types, discovery mechanics, or historical-layer export exist in code.

> **Researcher framing:** Labels like food, shelter, wall, cache, and settlement describe **emergent patterns** readable in material traces — never creature cognition types or hardcoded world categories.

## Summary

The world does not reset.

Every action, structure, and environmental change leaves traces in the landscape.

Over long timescales, these traces accumulate into **historical layers** — buried pathways, collapsed walls, filled channels, organic deposits, altered porosity, and abandoned settlements.

Creatures do not experience history as narrative.

They experience **sensor traces** that correlate with past events.

Researchers may read the same voxel field as archaeology: layered evidence of occupation, construction, catastrophe, and ecological change.

World history is the slow memory of the physical geography.

Archaeology is how that memory becomes legible — to organisms through prediction, to researchers through analysis.

## Core Principle

> The past is present in traces.

Organisms never access historical facts directly.

They access **current sensor samples** that may carry information about prior states: compacted void fraction where many feet passed, elevated organic fraction where matter was cached, altered thermal contact where void space was enclosed, chemical gradients where food was stored.

History is not stored as events.

History is stored as **material configuration**.

Prediction is the organism's archaeology — inferring unobserved past states from present traces to improve future regulation ([08_prediction.md](08_prediction.md), [04_regulation.md](04_regulation.md)).

## Historical Layers

The landscape is stratified by time.

Each layer encodes a different era of use:

| Layer type (researcher label) | Material signature | Typical source |
|-------------------------------|-------------------|----------------|
| Surface use | Compacted porosity, organic residue | Repeated movement ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)) |
| Construction | Elevated `binder`, altered `solid_fraction` | Dig, stack, bind (planned) |
| Abandonment | Weathering, infill, vegetation | Occupation cessation ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md)) |
| Burial | Overlying deposition, reduced exposure | Erosion, collapse, sediment |
| Catastrophe | Sudden discontinuity in layer sequence | Flood, fire, conflict (planned) |

Layers overlap and erode.

Older layers may be partially destroyed, compressed, or re-exposed.

**Planned** — tick logic that writes durable signatures into `erosion_damage`, `porosity`, and related fields when organisms act and when natural processes transport material ([01_world_generation.md](01_world_generation.md)).

## Persistence Hierarchy

Not all traces survive equally.

| Durability | Trace type | Substrate | Doc |
|------------|-----------|-----------|-----|
| Ephemeral | Single footprint, brief chemical spike | Surface organic, moisture | [05_sensors.md](05_sensors.md) |
| Short | Recent pathway, shallow dig | Compacted porosity | [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) |
| Medium | Cache deposit, shelter void, wall remnant | `binder`, `organic`, void geometry | [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) |
| Long | Settlement mound, channel cut, trail network | Multi-voxel modification | [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) |
| Very long | Erosion-sculpted landform, cave system | `erosion_damage`, geology | [01_world_generation.md](01_world_generation.md), [14_time_and_scales.md](14_time_and_scales.md) |
| Cultural | Shared concepts about a place | Population memory (planned) | [16_culture.md](16_culture.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md) |

Geological and built traces outlast individual memory graphs.

Internal memory dies with the organism ([06_memory.md](06_memory.md), [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md)).

The landscape remembers longer.

## Archaeological Information

Archaeological information is any material state that **reduces uncertainty about past occupation or events**.

Examples (researcher labels):

* **Food** — localized organic fraction elevation (cache, consumption site, midden)
* **Shelter** — void geometry with altered thermal and contact traces
* **Wall** — binder-enriched barrier altering flow and movement
* **Cache** — spatially fixed organic or mineral deposit
* **Settlement** — cluster of overlapping construction, pathway, and deposit signatures

Creatures do not label these patterns.

They experience chemical, thermal, contact, and moisture traces.

If a trace improves prediction of regulatory state, it is **meaningful** — regardless of whether the creature "knows" it is historical ([04_regulation.md](04_regulation.md)).

## Historical Discovery

Organisms discover history through **exploration** — encountering traces that do not match current expectations ([21_exploration_and_discovery.md](21_exploration_and_discovery.md)).

A creature that finds elevated organic matter where its model predicted barren ground updates its memory graph.

A creature that enters a sheltered void where wind exposure was expected revises thermal predictions.

Discovery is not excavation.

It is **surprise followed by integration**.

Researchers discover history through export and visualization — reading voxel fields, tick logs, and population trajectories across [14_time_and_scales.md](14_time_and_scales.md) generational scales ([17_analysis_and_visualization.md](17_analysis_and_visualization.md)).

## Interpretation

The same trace supports multiple interpretations.

A compacted pathway may indicate:

* frequent movement (researcher label: trail)
* deliberate construction (researcher label: road)
* natural erosion channel (researcher label: gully)

Creatures resolve ambiguity through **predictive utility** — whichever interpretation improves regulation wins, until contradicted by new traces ([08_prediction.md](08_prediction.md)).

Researchers resolve ambiguity through **cross-layer correlation** — matching material signatures across depth, time, and spatial context.

Interpretation is always inference from incomplete evidence.

## Forgotten Origins

Most historical traces lose their connection to intentional action.

A pathway may persist long after anyone remembers why it exists.

A cache site may outlast the population that dug it.

A wall may stand after the conflict that motivated it is forgotten ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) — ritual and myth).

**Forgotten origins** are the normal condition.

The landscape stores **effects** without storing **reasons**.

Cultural memory ([16_culture.md](16_culture.md)) may preserve explanatory structure longer than any individual — but culture too drifts and loses provenance.

Archaeology begins where living memory ends.

## Ruins

Ruins are structures that have **lost active maintenance** but retain partial material integrity.

Researcher labels:

* collapsed wall — binder failure, load shift, erosion
* abandoned shelter — void infill, roof collapse
* silted channel — deposition exceeding excavation
* overgrown cache — organic decay and biological uptake

Ruins are high-information zones.

They combine evidence of **original construction** with evidence of **subsequent decay** — two historical layers in one location.

Creatures may still exploit ruins: a partial wall still alters wind exposure; a filled cache may retain elevated organic fraction below the surface.

Ruins are infrastructure entering the archaeological record ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)).

## Ecological Archaeology

Biological activity leaves historical traces independent of construction.

Examples (researcher labels):

* organic enrichment from repeated feeding
* soil compaction from herd paths
* altered vegetation from sustained grazing
* nutrient hotspots from death and decomposition ([23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md))

Ecological archaeology reads **life history** in material fields — `organic`, `porosity`, moisture gradients — without assuming intentional agency.

Ecology and construction interleave in the same voxel column.

A settlement mound may sit atop centuries of organic deposition.

## Cultural Archaeology

Cultural archaeology reads **predictive structure** that outlasted its authors.

Sources include:

* durable infrastructure ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md))
* communicated patterns about places ([10_communication.md](10_communication.md))
* inherited biases toward certain locations ([13_inheritance.md](13_inheritance.md))
* settlement-level concentration ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md))

A population may return to a ruin not because any individual remembers its origin, but because **traces still improve prediction** — thermal shelter, organic availability, pathway efficiency.

Culture archaeology is the study of how shared prediction aligns with material history — and where they diverge into myth.

## Layered Landscapes

A single coordinate may encode many eras.

Conceptual stack (researcher model):

```
Present surface — current occupation, weathering
  ↓
Recent abandonment — partial infill, decay
  ↓
Active-use layer — construction, pathways, caches
  ↓
Earlier settlement — buried binder, compacted void
  ↓
Pre-occupation ecology — baseline organic, porosity
  ↓
Geological substrate — terrain generation ([01_world_generation.md](01_world_generation.md))
```

Layers are not discrete files.

They are **superposed field values** — `binder`, `organic`, `erosion_damage`, `porosity`, `solid_fraction` — whose spatial patterns imply sequence.

**Planned** — archaeology mechanics that accumulate and expose layers through erosion transport, burial, and re-excavation using existing voxel fields.

## Historical Momentum

Places that have been used tend to be used again.

Historical momentum arises from:

* **Infrastructure persistence** — existing walls, channels, and caches reduce construction cost
* **Ecological enrichment** — organic and moisture signatures attract return
* **Predictive alignment** — population concepts favor known sites ([16_culture.md](16_culture.md))
* **Path dependence** — movement follows existing compaction

Momentum is not destiny.

Environmental change, catastrophe, or superior alternative sites can redirect occupation.

But the landscape biases the future toward its own past ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) — attraction, positive feedback).

## Information Accumulation

History is cumulative information storage in the landscape ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md)).

Each generation may add:

* construction layers
* organic deposits
* communicated place-knowledge
* inherited location biases

Accumulation rate depends on:

* population density and return frequency
* construction and maintenance effort
* substrate durability (`binder`, geology)
* erosion and catastrophe loss rate

When accumulation exceeds loss, the landscape becomes **information-dense** — a rich archaeological record and a rich predictive environment for future organisms.

## Historical Catastrophe

Catastrophe is rapid, large-scale **information destruction** or **layer discontinuity**.

Examples (researcher labels):

* flood — buries or erases surface layers
* drought — removes organic signatures, disperses population
* collapse — structural failure destroys construction memory
* fire — alters organic and binder fields (planned)
* conflict — accelerated destruction of infrastructure (planned)

Catastrophe does not erase all history.

It **reorders** it — burying some layers, exposing others, creating discontinuities that later interpreters must explain.

Post-catastrophe landscapes are archaeological puzzles: traces without living context.

## Historical Persistence

Persistence is the central variable of world history.

A civilization (researcher label) persists when information accumulation across infrastructure, culture, and landscape exceeds information loss across death, erosion, drift, and catastrophe ([22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md)).

Historical persistence operates on [14_time_and_scales.md](14_time_and_scales.md) scales:

| Scale | Persistence mechanism |
|-------|----------------------|
| Lifetime | Individual memory graph |
| Generational | Inheritance + landscape traces |
| Cultural | Shared concepts + stories |
| Infrastructure | Binder, pathways, void geometry |
| Geological | Erosion-sculpted landforms |

The simulation asks: what survives, where, and for how long?

## Archaeology as Prediction

For organisms, archaeology **is** prediction.

Encountering a trace, the creature infers an unobserved past state to forecast a future regulatory outcome.

* Elevated organic → predict reduced hunger cost (researcher label: food site)
* Enclosed void → predict reduced thermal stress (researcher label: shelter)
* Binder barrier → predict blocked flow (researcher label: wall)

No separate "archaeology module" is required.

Historical inference is ordinary graph traversal over experiences that include spatially persistent traces ([08_prediction.md](08_prediction.md)).

For researchers, archaeology is **retrospective prediction** — reconstructing what past organisms likely experienced from material evidence.

## World Memory

The world is the system's long-term memory ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md), [01_world_generation.md](01_world_generation.md)).

| Memory medium | Field / structure | Timescale |
|---------------|-------------------|-----------|
| Flow history | `erosion_damage`, `porosity` | Landscape |
| Construction | `binder`, `solid_fraction` | Infrastructure |
| Organic use | `organic` | Ecological |
| Occupation density | Compaction, multi-layer overlap | Settlement |
| Climate epochs | `GlobalClimate` history (planned) | Geological |

Creatures **read** world memory through sensors.

Creatures **write** world memory through actions ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [03_creatures.md](03_creatures.md)).

Natural processes **edit** world memory through erosion, deposition, and biological turnover.

World memory is distributed, lossy, and unindexed — no global history table exists.

## Relationship to adjacent docs

| Doc | Relationship |
|-----|--------------|
| [01_world_generation.md](01_world_generation.md) | Environmental memory, erosion fields, geological baseline |
| [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) | How predictive structure enters the landscape |
| [14_time_and_scales.md](14_time_and_scales.md) | Nested clocks on which layers accumulate |
| [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md) | Substrates and persistence hierarchy |
| [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) | Spatial concentration, collapse, historical layers in settlements |

Doc 22 addresses settlements as living concentrations of culture.

This doc (24) addresses the **material record** those settlements leave — and how organisms and researchers read it after the fact.

## Current implementation

| Component | Status | Location |
|-----------|--------|----------|
| `erosion_damage` field | Present, not tick-updated | `world/voxel.rs` |
| `porosity` field | Present, terrain-init only | `world/voxel.rs` |
| `binder`, `organic` fields | Present | `world/voxel.rs` |
| Environmental memory tick | Planned | `world/` |
| Construction persistence | Planned | `creatures/actions.rs` |
| Historical-layer export | Planned | `export/snapshots.rs` |
| Occupation / layer analysis | Planned | `analysis/` |

No `HistoricalLayer`, `Ruin`, or `ArchaeologicalSite` types exist.

## Planned

- Tick logic writing durable action signatures into `erosion_damage`, `porosity`, and `organic`
- Erosion-driven burial and re-exposure of construction layers
- Settlement abandonment and ruin decay coupling to [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md)
- Catastrophe events that discontinuity-layer voxel columns
- Researcher-facing archaeology maps and layer-sequence visualization in [17_analysis_and_visualization.md](17_analysis_and_visualization.md)
- Cross-tick snapshot comparison for historical reconstruction

## Open questions

- Should `erosion_damage` encode magnitude only, or also directional flow history?
- Can layer depth be inferred from porosity gradients alone, or is explicit deposition tracking needed?
- How much historical inference should emerge from creature prediction vs researcher post-hoc analysis?
- When does re-occupation of ruins erase vs preserve earlier layers?

## Core Principle

> History is material. The past survives in traces. Organisms read history through prediction; researchers read it through archaeology.

The world forgets slowly.

That slowness is the foundation of meaning across generations.
