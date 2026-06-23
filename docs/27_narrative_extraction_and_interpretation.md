# Narrative Extraction and Interpretation

> **Guiding question:** How do meaningful stories emerge from simulation data?

## Status

**Partial** — `export/narrative.rs` scans the tick log at end of run and writes `exports/logs/narrative_summary.json` (first birth/death/dig, concept-formation spikes, birth bursts). Full significance scoring, biographies, and query APIs remain **planned**. Depends on observability exports ([26_research_tools_and_observability.md](26_research_tools_and_observability.md)), historical event recording ([24_world_history_and_archaeology.md](24_world_history_and_archaeology.md)), and memory/concept inspection ([06_memory.md](06_memory.md), [07_concepts.md](07_concepts.md)).

## Summary

The simulation produces vast quantities of events, memories, structures, and histories.

Most of this information cannot be directly observed.

Narrative extraction provides methods for identifying meaningful trajectories within the simulation.

The goal is not to impose stories upon the world.

The goal is to reveal stories already emerging from interactions between organisms, environments, and predictive systems.

## Core Principle

A narrative is a sequence of events that remains meaningful across time.

Meaningful events are events that significantly alter future prediction.

Narratives are therefore extracted from changes in predictive structure.

## Why Narrative Extraction Exists

The simulation may contain:

* millions of actions
* thousands of organisms
* centuries of history

Direct observation becomes impossible.

Narrative extraction identifies significant patterns.

**Skeleton:** current exports provide end-state snapshots and per-tick regulatory summaries — insufficient for narrative reconstruction without planned event streams and memory graph exports ([26_research_tools_and_observability.md](26_research_tools_and_observability.md), [17_analysis_and_visualization.md](17_analysis_and_visualization.md)).

## Event Significance

Not all events are equally important.

An event becomes significant when it causes large downstream effects.

Examples (researcher labels):

* discovery of water
* first shelter construction
* first successful migration
* settlement foundation
* settlement collapse
* communication divergence

Significance emerges through consequences.

**Planned:** significance scoring from regulatory deltas, memory graph changes, and landscape modifications — not pre-labeled event types.

## Narrative Seeds

Narratives often begin with small events.

Examples:

* single exploration
* single discovery
* single conflict
* single communication signal

Small events may produce large historical effects.

## Individual Narratives

A biography may be extracted from an individual organism.

Example structure:

```
Birth → Learning → Exploration → Reproduction → Death → Legacy
```

The organism becomes a narrative thread.

**Planned:** requires lineage tracking, experience history export, and death events ([23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md)).

## Discovery Narratives

Examples (researcher labels):

* first cave discovered
* first mountain crossed
* first groundwater source found

These narratives describe the expansion of predictive knowledge.

See [21_exploration_and_discovery.md](21_exploration_and_discovery.md).

## Infrastructure Narratives

Examples (researcher labels):

* path formation
* well construction
* wall construction
* settlement growth

These narratives describe externalized memory.

See [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).

## Communication Narratives

Examples:

* new signal appears
* signal spreads
* signal diverges
* signal disappears

These narratives describe cultural evolution.

See [10_communication.md](10_communication.md), [16_culture.md](16_culture.md).

## Settlement Narratives

Examples (researcher labels):

* foundation
* growth
* stability
* decline
* abandonment

Settlements become historical actors.

See [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md).

## Concept Narratives

Concepts possess histories.

A concept may:

* emerge
* expand
* split
* merge
* disappear

Concepts therefore have genealogies.

See [07_concepts.md](07_concepts.md).

## Cultural Narratives

Examples:

* migration traditions
* construction traditions
* warning traditions

Stories may persist for generations.

The simulation can reconstruct these trajectories.

See [16_culture.md](16_culture.md).

## Narrative Scale

Narratives exist at many scales.

Examples:

| Scale | Example focus |
|-------|---------------|
| Individual | Biography, learning arc |
| Population | Migration wave, divergence |
| Settlement | Foundation to abandonment |
| Culture | Signal spread, tradition persistence |
| Landscape | Channel formation, soil enrichment |
| World | Deep-time ecological and geological change |

The same event may appear differently at different scales.

## Historical Compression

Narrative extraction functions similarly to concept formation.

The simulation compresses:

```
many events → larger historical structures
```

History becomes a form of memory.

See [06_memory.md](06_memory.md), [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md).

## Archaeological Narratives

Not all narratives remain complete.

Some must be reconstructed from traces.

Examples (researcher labels):

* abandoned settlement
* buried cache
* collapsed channel

The researcher performs archaeology.

See [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md).

## Competing Narratives

Different organisms may experience the same event differently.

Narratives are not necessarily objective.

The simulation should preserve multiple perspectives.

**Planned:** per-creature memory graph exports allow parallel narrative threads from the same world events.

## Narrative Uncertainty

Historical reconstruction should allow uncertainty.

Not all causes are known.

Not all events are observed.

Narratives may remain incomplete.

Extracted narratives should expose confidence and gaps — not present false completeness.

## Research Applications

Narrative extraction supports:

* cognitive analysis
* communication analysis
* cultural analysis
* evolutionary analysis

## Artistic Applications

Narrative extraction supports:

* exhibitions
* installations
* books
* timelines
* printed outputs
* generated texts

The simulation becomes a storytelling instrument.

## Example Queries

Tell me about the first organism that crossed the mountain.

Tell me how the concept associated with shelter emerged.

Tell me why Settlement 14 collapsed.

Show the history of this cave.

Show the origin of this migration route.

Show how this signal changed over time.

These questions become answerable through narrative extraction.

Researcher labels (shelter, settlement, cave, mountain, food, water) appear only in queries and analysis — never in creature code.

## Narrative and Meaning

Meaning emerges through prediction.

Narratives emerge through changes in prediction across time.

Narrative extraction therefore becomes the study of meaning in motion.

See [00_project_overview.md](00_project_overview.md) — **meaning = predictive relevance**.

## Current implementation

| Mechanism | Location | Status |
|-----------|----------|--------|
| Narrative extraction pipeline | `export/narrative.rs` | Partial — `narrative_summary.json` from tick log |
| Event significance scoring | — | Planned |
| Biography / genealogy export | — | Planned |
| Query interface | — | Planned |
| Tick logs (raw substrate) | `export/logs.rs` | Partial |
| World snapshots (end state) | `export/snapshots.rs` | Partial |

## Planned

- Historical event stream with significance metadata
- Per-creature experience and memory graph export for biography extraction
- Multi-scale narrative compression (individual → world)
- Archaeological reconstruction from landscape traces
- Competing-perspective narrative bundles
- Uncertainty annotations on reconstructed narratives
- Query API or analysis scripts for example queries above
- Artistic output formats (timelines, generated text, exhibition data)

## Cross-references

| Topic | Doc |
|-------|-----|
| Research tools & exports | [26_research_tools_and_observability.md](26_research_tools_and_observability.md) |
| Analysis overview | [17_analysis_and_visualization.md](17_analysis_and_visualization.md) |
| World history | [24_world_history_and_archaeology.md](24_world_history_and_archaeology.md) |
| Memory | [06_memory.md](06_memory.md) |
| Concepts | [07_concepts.md](07_concepts.md) |
| Settlements | [22_settlements_culture_and_civilization.md](22_settlements_culture_and_civilization.md) |
| Exploration | [21_exploration_and_discovery.md](21_exploration_and_discovery.md) |

## Core Principle

The simulation does not generate stories.

The simulation generates events.

Stories emerge when events are connected through memory, prediction, consequence, and time.
