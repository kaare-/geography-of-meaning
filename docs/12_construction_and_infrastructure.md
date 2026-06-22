# Construction and Infrastructure

> **Guiding question:** How does memory move into the landscape?

## Status

**Stub** — voxel `binder` field and creature `carried_mass` exist; dig, carry, stack, place, and bind actions are **planned**. No construction tick logic in skeleton.

> **Researcher framing:** All examples below (shelter, cache, wall, bridge, settlement, trail) describe **emergent patterns** and **externalized prediction** — not hardcoded world types, voxel categories, or creature cognition concepts.

## Summary

Creatures carry memory internally ([06_memory.md](06_memory.md)). Construction **externalizes** predictive structure into the physical landscape: material arrangements that persist beyond individual lifespan and bias future sensor traces. Internal memory and landscape memory form a **distributed cognitive system**.

## Core Principle

> Memory is not only in the organism. The landscape stores predictive structure.

A pile of bound mineral, a channel cut through clay, or a compacted path are physical states that make certain futures more probable. Creatures never perceive "shelter" or "wall" — they perceive contact, thermal, and chemical traces that correlate with regulatory stability.

## Why Infrastructure Exists

Regulation has a spatial component. Surviving cold, retaining organic matter, diverting water, or blocking collapse can be achieved by **changing local material configuration**. Infrastructure is regulation extended into geology — using the same physics rules as natural formation ([02_physics_and_materials.md](02_physics_and_materials.md)).

## Information Storage

The world already stores information implicitly: channels remember flow (`erosion_damage`, `porosity`), trails remember movement (compacted fractions), deposits remember transport. Construction accelerates and directs this **landscape memory** ([01_world_generation.md](01_world_generation.md)).

## Externalized Prediction

Researcher descriptions of emergent patterns:

| Emergent pattern (researcher label) | Externalized prediction |
|-------------------------------------|-------------------------|
| Shelter | Reduced thermal stress, contact stability |
| Cache | Elevated organic fraction at fixed location |
| Wall | Blocked flow / movement, altered contact |
| Channel | Directed water, lowered adjacent saturation |
| Trail | Reduced movement cost, chemical residue |

None of these labels exist in simulation code.

## Construction

Planned action families:

| Action | Effect |
|--------|--------|
| Dig | Remove solid fraction, increase void |
| Carry | Transfer material to `carried_mass` |
| Drop / place | Deposit carried mass at voxel |
| Stack | Increase local solid fraction vertically |
| Bind | Add `binder`, increase `structural_strength` |

All modifications obey standard material physics — no "built" material type.

## Costs

Construction consumes **energy**, increases **fatigue**, and may reduce **integrity** (collapse risk during digging). Costs scale with `carried_mass` and material hardness.

## Maintenance

Structures erode, load-shift, and fail per [02_physics_and_materials.md](02_physics_and_materials.md). Maintenance is ongoing material addition — another form of externalized prediction refresh.

## Pathways

Repeated movement compacts void fraction and leaves organic/chemical residue. Pathways reduce movement cost and create predictable sensor traces — trails as **movement memory**.

## Water Infrastructure

Channels and dams alter `surface_water` flow. Researcher label: "irrigation" or "mill race" — emergent only.

## Storage Infrastructure

Localized elevation of `organic` or other fractions via place/stack actions. Researcher label: "cache" — creatures experience chemical gradients, not storage intent.

## Defensive Infrastructure

Barriers that alter contact and movement traces. Researcher label: "wall" — no combat or territory types in cognition.

## Settlement Formation

**Planned** — spatial clustering of construction, pathways, and storage when multiple creatures' externalized predictions overlap and reinforce.

## Infrastructure and Communication

Built structures change sound propagation and create landmarks for attention signals ([10_communication.md](10_communication.md)).

## Infrastructure and Culture

Durable structures survive individuals; later generations inherit **sensor traces** linked to ancestral construction — substrate for shared concepts ([16_culture.md](16_culture.md)).

## Landscape Memory

The voxel field is a slow memory medium: `binder`, `erosion_damage`, `organic`, `solid_fraction` history encode past labor and events.

## Cognitive Consequences

Externalized prediction offloads computation from individual graphs into the environment. A creature benefits from others' construction without explicit cooperation concepts.

## Distributed Intelligence

Population + landscape forms a coupled system: internal graphs, shared traces, and persistent material state co-evolve.

## Long-Term Evolution

Over long timescales, infrastructure feeds back into geology (erosion, load, collapse), biology (organic concentration), and culture (inherited trace patterns).

## Core Principle

> Construction is memory made durable. The landscape predicts so organisms do not have to remember alone.

## Current implementation

| Field / struct | Location | Status |
|----------------|----------|--------|
| `binder` | `world/voxel.rs` | Field present |
| `carried_mass` | `creatures/regulation.rs` | Field present |
| Dig / carry / place / bind | `creatures/actions.rs` | Not implemented |
| Load / collapse interaction | `world/` | Planned |

## Planned

- Full construction action set in `actions.rs`
- Binder production (organism metabolite → voxel `binder`)
- Pathway compaction from repeated `Move`
- Export infrastructure maps in [17_analysis_and_visualization.md](17_analysis_and_visualization.md)

## Open questions

- Single `carried_mass` scalar vs typed material fractions?
- Should binding require sustained presence (maintenance ticks)?
