# Creatures

> **Guiding question:** What is an organism?

## Status

**Partial** — `Creature` struct, regulatory state, genome, three actions, sensor integration, experience buffer, and memory graph wiring are implemented. Full lifecycle, reproduction, death, extended action space, and cognition cost are planned.

## Summary

An organism is a **predictive regulatory system**, not a goal-driven agent. It does not pursue objectives like "find food" or "reach shelter." It maintains internal variables within viable ranges through actions whose consequences are learned over time via sensor traces and memory.

Regulation comes first. Prediction and meaning emerge from the history of regulation — not from pre-programmed goals.

## Core Principle

Creatures experience **traces** ([05_sensors.md](05_sensors.md)), store **relationships** ([06_memory.md](06_memory.md)), and eventually compress **concepts** ([07_concepts.md](07_concepts.md)). They never receive world variables or semantic object labels directly.

What researchers may later call food, shelter, rivers, or caves appears only as patterns across sensor channels and memory — never as named types in creature code.

## Organism Model

Each creature combines physical presence, sensing, memory, and regulation:

```
┌─────────────────────────────────────────┐
│  Body (position, genome, signature)     │
│  Sensors (15-channel traces)            │
│  Regulatory state (internal variables)  │
│  Memory graph (relational, per-creature)│
│  Concept nodes (placeholder)            │
│  Prediction (planned → 08_prediction)     │
└─────────────────────────────────────────┘
```

Implemented in `sim-core/src/creatures/creature.rs` as `Creature`:

| Field | Module | Role |
|-------|--------|------|
| `id`, `position`, `signature`, `age` | `creature.rs` | Identity and physical presence |
| `genome` | `genome.rs` | Heritable tunables |
| `regulatory` | `regulation.rs` | Internal homeostasis |
| `sensor` | `sensors.rs` | Current sensor traces |
| `recent_experience` | `creature.rs` | Rolling buffer (max 64) |
| `memory_graph` | `memory/graph.rs` | Directed relational memory |
| `concept_nodes` | `memory/concepts.rs` | Placeholder for compression |

## Physical Presence

Creatures occupy a single voxel position (`Vec3f`). Movement is discrete: one step per `Move` action into a cell with sufficient `void_fraction`. Physical interaction with the world modifies voxel fields (e.g. organic transfer on `ConsumeOrganic`) without semantic classification.

Spawn logic in `World::find_spawn_positions` places creatures in warm, wet, organic-rich void cells — procedural criteria, not labeled habitats.

## Regulatory Variables

`RegulatoryState` in `sim-core/src/creatures/regulation.rs`:

| Variable | Range | Role |
|----------|-------|------|
| `energy` | 0–1 | Metabolic fuel; drains passively each tick |
| `hydration` | 0–1 | Water balance |
| `temperature_stress` | 0–1 | Thermal discomfort |
| `integrity` | 0–1 | Structural / damage state |
| `fatigue` | 0–1 | Activity exhaustion |
| `carried_mass` | ≥ 0 | Material mass being transported |

`tick_passive_drain(metabolism)` runs each tick. `apply_action_cost(energy, fatigue)` runs after actions. All values clamped to valid ranges.

Low energy and high fatigue bias action selection toward rest and organic consumption — regulatory pressure, not goal pursuit.

## Actions

Full envisioned action space:

| Action | Skeleton | Description |
|--------|----------|-------------|
| Move | **Yes** | Step to adjacent void-rich voxel |
| Rest | **Yes** | Reduce fatigue, slight energy recovery |
| Consume organic | **Yes** | Transfer organic fraction from adjacent voxel → energy |
| Drink | Planned | Absorb pore/surface water → hydration |
| Carry | Planned | Pick up material → `carried_mass` |
| Drop | Planned | Deposit carried material into voxel |
| Dig | Planned | Remove solid fraction from voxel |
| Place | Planned | Add carried material to voxel |
| Emit sound | Planned | Produce acoustic signal ([10_communication.md](10_communication.md)) |
| Follow | Planned | Move toward sensory gradient |
| Push | Planned | Displace loose material or organism |

Skeleton `Action` enum in `sim-core/src/creatures/actions.rs`:

```rust
Action::Move(Vec3i)
Action::ConsumeOrganic
Action::Rest
```

Selection: `choose_action()` — weighted random modulated by regulatory state. Not prediction-guided (planned in [08_prediction.md](08_prediction.md)).

Researchers may describe `ConsumeOrganic` as "eating" in analysis. The code uses no food concept.

## Morphology

### Summary

Morphology is **ecological commitment made physical** — a body size and shape strategy for regulating uncertainty under local conditions. Mass, surface area, and sensor reach bias which traces dominate experience and which regulatory tradeoffs matter. Morphology is not cosmetic; it is prediction embodied.

### Core Principle

> The body is a strategy for regulating uncertainty.

Larger bodies buffer fluctuation; smaller bodies sample more of the world per unit mass. Neither is optimal globally — each commits to a different error profile across energy, thermal, and contact channels.

### Body Mass

**Planned:** explicit `body_mass` field in `Genome` / `Creature`. Skeleton uses implicit defaults via metabolism and action costs only; `carried_mass` in `RegulatoryState` is separate from structural body mass.

| Scale | Regulatory implication |
|-------|------------------------|
| Higher mass | Slower energy turnover, higher baseline metabolism (planned) |
| Lower mass | Faster drain, finer gradient sampling (planned) |

### Large Morphologies

**Advantages (planned coupling):**

- Thermal inertia — slower temperature_stress swings
- Contact dominance — higher `contact_hard` / `contact_soft` in conflicts
- Carrying headroom — more `carried_mass` before movement penalty

**Disadvantages:**

- Higher baseline energy demand
- Coarser sensor integration (fewer samples per displacement)
- Slower movement, higher fatigue per step

### Small Morphologies

**Advantages:**

- Lower metabolism per tick (planned mass scaling)
- Finer chemical/thermal gradients in neighborhood
- Faster movement through void-rich voxels

**Disadvantages:**

- Rapid thermal equilibration — environmental volatility hits regulation harder
- Vulnerable in contact-dominated interactions
- Limited carry capacity

### Thermal Consequences

Body mass modulates coupling between `thermal` sensor traces and `temperature_stress`. Large forms lag the environment; small forms track it closely. No creature perceives "cold" or "heat" as categories — only stress scalars and traces.

### Conflict Consequences

When organisms occupy adjacent voxels, contact channels and mass (planned) determine displacement and integrity costs. No "fight" or "enemy" concepts — only regulatory outcomes from contact events.

### Carrying Capacity

`carried_mass` in `regulation.rs` exists; carry/drop actions are **planned** ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)). Effective capacity will scale with body mass and genome.

### Ecological Niches

Roles like **explorer**, **builder**, **forager**, or **stationary accumulator** are **emergent researcher descriptions** of morphology + behavior clusters — not predefined creature types or cognition labels. A small, mobile, low-mass lineage may repeatedly visit high-organic traces; a large, slow lineage may modify local solids. The simulation assigns no niche enum.

What researchers might call a "food-rich" patch is only elevated `chemical_organic` — not a forager role in code.

### Core Principle

> Morphology commits the organism to a prediction strategy before memory forms.

Sensor geometry, mass, and metabolism shape which experiences accumulate and which concepts (eventually) compress — without hardcoded ecological roles.

## Cognition Cost

**Planned.** Memory retrieval, spreading activation, and prediction traversal will consume energy/fatigue — making cognition metabolically expensive and biasing organisms toward efficient compressed concepts.

## Lifecycle

**Planned** — birth → growth → reproduction → death. See also: [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md) (population dynamics, migration, extinction, evolution).

| Phase | Status |
|-------|--------|
| Birth (spawn) | Implemented via `Creature::new` + world scan |
| Aging | `age` increments each tick |
| Growth | Planned |
| Reproduction | Planned |
| Death | Planned |

## Reproduction

**Planned.** Genome mutation (`genome.rs` tunables: `metabolism_rate`, `sensor_noise_scale`, `move_speed`) will vary across offspring. Inherited memory structures link to [13_inheritance.md](13_inheritance.md) and [16_culture.md](16_culture.md).

## Death

**Planned.** Triggered by energy depletion, integrity failure, or environmental catastrophe. Death may deposit organic matter into the voxel landscape ([01_world_generation.md](01_world_generation.md)).

## Morphology

### Summary

Morphology is **ecological commitment** — a body size and form that embodies a strategy for regulating uncertainty in a particular environment. Mass, surface area, and carrying capacity shape which sensor traces matter and which regulatory futures are viable.

### Core Principle

> The body is a strategy for regulating uncertainty.

Morphology is not cosmetic. It determines metabolism, thermal coupling, conflict outcomes, and construction capacity — all without predefined roles like "builder" or "forager."

### Body Mass

**Planned** — explicit `body_mass` field in `Genome` or `Creature`. Skeleton uses implicit defaults via metabolism and action costs only.

| Scale | Regulatory implication |
|-------|------------------------|
| Low mass | Fast metabolism drain per unit; low inertia |
| High mass | Slower movement; higher energy reserves possible |

### Large Morphologies

**Advantages (planned):** greater `carried_mass` capacity, push dominance, thermal inertia in cold traces, structural interaction with collapse.

**Disadvantages (planned):** higher baseline metabolism, slower movement, larger sensor contact profile, harder to find void-rich paths.

### Small Morphologies

**Advantages (planned):** lower metabolism, access to narrow voids, faster fatigue recovery relative to mass.

**Disadvantages (planned):** push vulnerability, rapid thermal stress from environment, limited carry capacity.

### Thermal Consequences

Surface-area-to-mass ratio couples creatures to `temperature` and `temperature_stress` ([04_regulation.md](04_regulation.md)). Large bodies buffer thermal swings; small bodies track local climate closely.

### Conflict Consequences

Mass modulates push outcomes and integrity damage ([11_social_systems.md](11_social_systems.md)). No combat types — only physical displacement economics.

### Carrying Capacity

`carried_mass` in `RegulatoryState` — skeleton field without pickup logic. Future: capacity scales with morphology and affects movement cost.

### Ecological Niches

Researchers may describe emergent roles — **explorers**, **builders**, **aggregators** — from morphology × memory × environment. These are **not** predefined classes or AI goals. Niches appear when regulatory + morphological strategies stabilize in a landscape.

### Core Principle

> Form predicts which futures are cheap. Evolution (planned) selects forms that compress regulatory surprise.

**Current implementation:** `carried_mass` in `regulation.rs`; genome tunables in `genome.rs`. Explicit `body_mass` — **planned**.

## Cross-references

| Topic | Doc |
|-------|-----|
| Lifecycle & population | [23_creature_lifecycle_and_population.md](23_creature_lifecycle_and_population.md) |
| Sensor traces | [05_sensors.md](05_sensors.md) |
| Experience & memory | [06_memory.md](06_memory.md) |
| Concept compression | [07_concepts.md](07_concepts.md) |
| Prediction-guided action | [08_prediction.md](08_prediction.md) |
| Construction / carrying | [12_construction_and_infrastructure.md](12_construction_and_infrastructure.md) |
| World spawn environment | [01_world_generation.md](01_world_generation.md) |

## Current implementation

| Module | Role |
|--------|------|
| `creature.rs` | `Creature`, `Experience`, experience buffer |
| `genome.rs` | `Genome` defaults |
| `regulation.rs` | Passive drain, action costs, clamping |
| `actions.rs` | `Action` enum, `choose_action`, `apply_action` |
| `sensors.rs` | `read_sensors` (see 04) |

Tick integration: `sim-core/src/simulation/engine.rs` — sense → regulate → act → sense → experience → memory.

## Planned

- Full lifecycle (growth, reproduction, death)
- Extended actions (drink, carry, drop, dig, place, sound, follow, push)
- Cognition cost for memory/prediction
- Integrity damage from environment and collapse
- Prediction-guided action selection
- Genome mutation on reproduction

## Open questions

- When does integrity interact with collapse physics ([02_physics_and_materials.md](02_physics_and_materials.md))?
- How should `carried_mass` affect movement cost and sensor traces?
- Minimum viable regulatory set for emergent concepts?
