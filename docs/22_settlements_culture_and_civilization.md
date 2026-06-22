# Settlements, Culture, and Civilization

> **Guiding question:** How do repeated occupation and shared prediction concentrate into places and persistence?

## Status

**Planned / Stub** — no settlement or civilization primitives in code. Conceptual design for long-timescale spatial concentration of culture. Distinct from [16_culture.md](16_culture.md): doc 16 addresses how shared predictive patterns stabilize in populations; this doc addresses settlements as spatial concentration of culture and the emergence of civilization.

## Summary

Settlements emerge when organisms repeatedly invest effort into the same location.

Culture emerges when predictive structures survive individuals.

Civilization, if it emerges at all, is the accumulation of predictive structures across many generations.

The simulation does not begin with settlements.

Settlements emerge through interactions between ecology, memory, communication, infrastructure, and inheritance.

## Core Principle

A settlement is a **concentration of information**.

The structures within a settlement contain knowledge about (researcher labels):

* water
* food
* weather
* movement
* storage
* conflict

Settlements are memory made spatial.

Creatures never receive settlement, city, or civilization as cognition — only sensor traces, regulatory consequences, and relational memory. See [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).

## Settlement Formation

A settlement emerges when organisms repeatedly return to a location.

Common causes (researcher labels):

* reliable moisture gradients
* organic abundance
* sheltered void space
* existing infrastructure
* communication hubs

Repeated use increases local investment.

**Planned** — no occupancy tracking or return-frequency metric in skeleton. Emergence depends on [03_creatures.md](03_creatures.md) action history, [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) landscape modification, and [14_time_and_scales.md](14_time_and_scales.md) long-run dynamics.

## Attraction

A settlement becomes attractive because of existing information.

Examples (researcher labels):

* paths
* shelters
* caches
* channels

The settlement itself modifies future prediction — sensor traces at a frequented site differ from traces at a virgin site.

## Positive Feedback

Infrastructure encourages occupation.

Occupation creates more infrastructure.

Infrastructure then attracts more occupation.

Settlements therefore exhibit self-reinforcing behavior.

**Planned** — binder deposits, void modification, and repeated `ConsumeOrganic`/`Move`/`Rest` cycles altering local voxel fields. See [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md).

## Centers and Peripheries

Not all locations are equal.

Some locations become:

* communication centers
* storage centers
* construction centers

Others remain peripheral.

Spatial hierarchy emerges naturally from differential return rates and infrastructure density — not from programmed city tiers.

## Shared Prediction

Settlements allow predictive structures to become partially shared.

Examples:

* common migration routes
* common danger signals
* common storage locations

The group develops overlapping models of the world.

**Planned** — overlapping memory graphs and communicated sound patterns. See [10_communication.md](10_communication.md), [16_culture.md](16_culture.md).

## Social Memory

Information may become distributed across many organisms.

No single organism possesses all information.

The settlement collectively remembers.

**Skeleton:** each `Creature` owns an isolated `MemoryGraph`. No population-level memory store exists yet.

## Knowledge Accumulation

Knowledge accumulates when information persists longer than individual lifetimes.

Mechanisms include:

* communication ([10_communication.md](10_communication.md))
* infrastructure ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md))
* inheritance ([13_inheritance.md](13_inheritance.md))
* landscape modification ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md))

## Tradition

Traditions are predictive structures that persist through repeated use.

Examples (researcher labels):

* seasonal migration
* storage practices
* construction methods

Traditions are inherited predictions — not hardcoded behaviors.

## Ritual

Some behaviors may persist despite incomplete understanding.

A behavior may continue because it has historically improved regulation.

The underlying reason may be forgotten.

Rituals emerge when behavior survives explanation.

## Stories

Stories are portable predictive structures.

Stories allow information to move without direct experience.

Stories may contain:

* environmental information
* social information
* historical information

Stories need not be accurate. They only need to influence prediction.

**Planned** — temporal sound→activation sequences. See [10_communication.md](10_communication.md).

## Myth

Myths emerge when predictive structures persist after their original context disappears.

Examples (researcher labels):

* a warning survives the danger
* a route survives the destination
* a practice survives the condition that produced it

Myths are historical predictions — researcher-facing labels for high-confidence, low-falsifiability concept clusters. See [16_culture.md](16_culture.md).

## Cultural Drift

Cultures change continuously.

Concepts:

* merge
* split
* disappear
* emerge

Culture remains dynamic across [14_time_and_scales.md](14_time_and_scales.md) generational timescales.

## Collective Intelligence

A settlement may solve problems that exceed individual capabilities.

Prediction becomes distributed.

The group becomes an ecological cognitive system.

**Planned** — analysis of population-level predictive overlap and coordinated action. No collective agent type in code.

## Infrastructure and Culture

Infrastructure stabilizes culture.

Examples (researcher labels):

* paths stabilize movement
* walls stabilize boundaries
* caches stabilize storage

Infrastructure acts as cultural memory embedded in the voxel landscape. See [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md), [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).

## Settlement Collapse

Settlements are not permanent.

Collapse may result from:

* environmental change
* resource depletion
* conflict
* infrastructure failure

Collapse redistributes information.

Some information survives.

Some disappears.

## Historical Layers

Old settlements leave traces.

Examples (researcher labels):

* collapsed walls
* abandoned trails
* filled-in channels
* buried caches

History becomes embedded in the landscape — readable by researchers and future organisms through sensor traces, not through labeled history nodes.

## Civilization

Civilizations emerge when information accumulation exceeds information loss.

Key ingredients:

* inheritance ([13_inheritance.md](13_inheritance.md))
* communication ([10_communication.md](10_communication.md))
* infrastructure ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md))
* maintenance (ongoing regulatory and construction effort)

Civilization is not population size.

Civilization is **persistence**.

## Relationship to doc 16 (Culture)

| Doc | Focus |
|-----|-------|
| [16_culture.md](16_culture.md) | How shared predictive patterns stabilize across a population |
| This doc (22) | How repeated occupation concentrates those patterns in place and scales to civilization |

Doc 16 is population-stable predictive structure. Doc 22 is spatial concentration, infrastructure feedback, and multi-generational accumulation.

## Current implementation

None beyond prerequisites that may later support settlement emergence:

| Prerequisite | Status | Location |
|--------------|--------|----------|
| Individual memory graphs | Partial | `memory/graph.rs` |
| Voxel landscape modification | Partial | `creatures/actions.rs` (organic consumption) |
| Genome inheritance | Partial | `creatures/genome.rs` |
| Spawn clustering | Partial | `world/mod.rs` |
| Export for spatial analysis | Partial | `export/snapshots.rs` |

No `Settlement`, `Civilization`, or population-memory types exist.

## Planned

- Occupancy and return-frequency tracking for settlement detection
- Infrastructure types (paths, caches, shelters) as voxel/binder patterns
- Population-level culture metrics in analysis export
- Communication-mediated shared prediction at hubs
- Collapse and historical-layer simulation
- Researcher-facing settlement/civilization labels in analysis only

## Core Principle

Culture is predictive structure that survives individuals.

Settlements are places where that survival becomes concentrated.

Civilizations are landscapes shaped by accumulated prediction.
