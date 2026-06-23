# Inheritance

> **Guiding question:** How does predictive structure survive across generations?

## Status

**Planned / Stub** — genome fields inherit at reproduction (not yet implemented). No direct memory-graph copy. Landscape and social inheritance planned.

## Summary

Predictive structure outlives individuals through **multiple substrates**: genetic biases, cognitive tendencies, social learning, environmental modification, and landscape memory. No single channel carries "knowledge" intact.

## Core Principle

> Inheritance operates through **multiple substrates** — never a single memory dump.

## Why Inheritance Exists

Individual memory graphs die with the organism. Long-timescale regulation requires persistence across birth and death — via whatever channels reduce regulatory surprise for descendants.

## Forms of Inheritance

| Form | Substrate | Timescale | Skeleton |
|------|-----------|-----------|----------|
| **Genetic** | `Genome` tunables | Generations | `metabolism_rate`, `sensor_noise_scale`, `move_speed` |
| **Cognitive** | Biased attention / learning rates | Lifetime | Planned |
| **Social** | Modified predictions via communication | Days–generations | Planned ([10_communication.md](10_communication.md)) |
| **Environmental** | Construction, trails, caches | Generations+ | Planned ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)) |
| **Landscape** | Voxel fields (erosion, organic, binder) | Geological | Partial fields only |

## Information Persistence

| Timescale | Examples |
|-----------|----------|
| Event | Single experience |
| Experience | Recent buffer (64 ticks) |
| Lifetime | Memory graph |
| Generational | Genome + landscape traces |
| Cultural | Shared compressed concepts |
| Geological | Landform, erosion patterns |

## Direct Memory Transfer

**Avoid.** Copying one creature's `MemoryGraph` to offspring would bypass individual prediction learning and collapse individuality. Preferred: offspring **reconstruct** similar structure through shared traces and biases.

## Preferred Model

1. Genetic biases shape sensor noise, metabolism, morphology (planned)
2. Social signals bias attention ([10_communication.md](10_communication.md))
3. Landscape stores externalized prediction ([15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md))
4. Sleep consolidates personal graph ([09_sleep.md](09_sleep.md))

## Tradition

**Planned** — stable cross-generation patterns in sound→outcome and trace→construction without explicit teaching.

## Cultural Accumulation

**Planned** — see [16_culture.md](16_culture.md). Layered compression of population-stable predictions.

## Lineages

`signature` distinguishes individuals; genome defines lineage. Lineages diverge when reproductive isolation (spatial or social) separates predictive environments.

## Inheritance and Meaning

Meaning inherited only if compressed structure **still reduces regulatory prediction error** in new contexts — otherwise edges weaken (concept drift).

## Evolution of Concepts

Concepts that survive inheritance are those that compress well and predict reliably across environmental shift ([07_concepts.md](07_concepts.md)).

## Information Bottlenecks

Death, sleep pruning, erosion, and population collapse destroy information. Bottlenecks force re-compression and innovation.

## Innovation

Mutation (genome), prediction error (memory), and environmental shock create novel edges — source of new predictive structure.

## Core Principle

> What survives is what still predicts — across bodies, minds, landscapes, and time.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `Genome` + `mutate_from` | `creatures/genome.rs` | Tunables including `developmental_bias` |
| `signature` | `creatures/creature.rs` | Parent hash + RNG variation on birth |
| Reproduction | `creatures/lifecycle.rs` | No memory graph copy |
| Memory copy | — | Explicitly avoided |

## Planned

- Reproduction with genome mutation
- Cognitive bias inheritance
- Social learning without graph copy
- Landscape legibility for offspring

## Open questions

- Minimum viable genome dimensionality?
- Can landscape alone sustain "culture" without communication?

---

## Developmental Signals and Social Inheritance

> **Addendum** — age-structured traces, weak biases, and proximity-based transmission without memory copy.

### Core Principle

> Offspring do **not** inherit memory graphs. They inherit **weak biases** and receive **developmental signals** from nearby organisms through the same sensor traces as adults.

Predictive structure crosses generations through genome mutation, modified landscapes, and socially biased experience — never through copying another creature's `MemoryGraph`.

### Developmental Signatures

Age modulates observable traces — what researchers label juvenile, adult, or elder **phenotypes**:

| Cue channel | Juvenile (researcher label) | Adult | Elder (researcher label) |
|-------------|----------------------------|-------|--------------------------|
| Vocal | Higher pitch, shorter duration | Baseline `vocal_profile` | Lower pitch, slower rhythm |
| Chemical | Weaker creature trace | Full `chemical_creature` | Accumulated metabolic signature |
| Movement | Irregular, low-amplitude sound | Steady cadence | Slower, heavier footfalls |
| Body size | Low mass morphology | Peak `mass` / carry | High mass, reduced speed |

**Implemented:** `Creature::age` increments each tick. Age-dependent scaling of `vocal_profile` emission (`age_adjusted_vocal_profile`), signature age bands (`signature_with_age_band`), and incidental movement sound amplitude via morphology coupling.

### Developmental Biases

Genome carries **weak inherited biases** toward attending juvenile-like trace patterns — not predefined caregiving behaviors:

- slightly elevated weight on high-pitch `sound_calls` in action scoring via `developmental_follow_boost` (memory-mediated, not role-based)
- no `caregiver` role enum — caregiving emerges when following/proximity repeatedly precedes positive regulatory outcomes for both parties

Biases shape **which experiences are likely**, not which actions are mandated.

### Social Inheritance

Knowledge transfers through **interaction geometry**, not graph duplication:

| Mechanism | Substrate | Status |
|-----------|-----------|--------|
| Following | `Action::Follow` toward salient creature/sound traces | Implemented |
| Proximity organic transfer | `Action::TransferOrganic` — adjacent low-energy neighbor | Implemented |
| Shared foraging | Proximity → overlapping sensor experiences | Emergent |
| Call imitation | Listener memory edges from heard `SoundEvent`s | Partial |
| Trust-weighted follow | `trusted_follow_boost` per `signature` | Implemented |

A juvenile that follows an adult into high-organic traces may gain energy without inheriting the adult's memory — it **reconstructs** similar edges from its own experiences.

### Dynamic Family Formation

No `family` struct, parent pointer, or kinship table exists in code — by design.

Researchers may later describe **biological parent**, **unrelated adult mentor**, or **cooperative group** clusters from:

- spatial proximity after birth (`lifecycle.rs` reproduction)
- shared `signature` lineage (hash-derived, not explicit pedigree)
- positive `trusted_follow_boost` history

Family is an **emergent label** on repeated beneficial interaction, not a simulation primitive.

### Elder Effects

Older organisms may accumulate stronger prediction confidence for local trace→outcome chains. Younger organisms following elders benefit when elder trajectories improve listener regulation — researchers call this *knowledge* or *experience*.

**Emergent, not role-based:** no `elder` type in creature code. Effects arise from longer `age` → more memory edges → higher edge confidence → stronger `trusted_follow_boost` influence on followers' `Action::Follow` scoring.

### Current implementation (developmental addendum)

| Component | Location | Status |
|-----------|----------|--------|
| `age` | `creatures/creature.rs`, `export/snapshots.rs` | Increments each tick; exported in snapshot |
| Age-scaled `vocal_profile` | `world/sound.rs` | `age_adjusted_vocal_profile` on `EmitSound` |
| `signature_with_age_band` | `world/sound.rs` | Subtle age-band mixing into emitter signature |
| `Genome::developmental_bias` | `genome.rs` | Mutated on reproduction; weak juvenile-signature bias |
| `developmental_follow_boost` | `memory/graph.rs` | Memory-mediated follow weight (not caregiving opcode) |
| `Genome::mutate_from` + reproduction | `genome.rs`, `lifecycle.rs` | Weak bias inheritance (partial) |
| `signature` | `creature.rs` | Per-individual; no family struct |
| `Action::Follow` | `actions.rs`, `spatial.rs` | Social proximity inheritance |
| `Action::TransferOrganic` | `actions.rs`, `spatial.rs`, `engine.rs` | Adjacent transfer when actor `carried_mass` ≥ 0.12 and neighbor energy ≤ 0.45; selection boost when nearby creature trace + carried load; `transfer_count` in tick log |
| `trusted_follow_boost` | `memory/graph.rs` | Social prediction link |
| Memory graph copy on birth | — | Explicitly avoided |

### Cross-references (developmental addendum)

| Topic | Doc |
|-------|-----|
| Incidental & intentional signals | [10_communication.md](10_communication.md) § Communication and Incidental Signals |
| Following, push, conflict | [11_social_systems.md](11_social_systems.md) |
| Reproduction & aging | [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md) |
| Acoustic & chemical traces | [05_sensors.md](05_sensors.md) § Environmental Sound and Action Sound |
