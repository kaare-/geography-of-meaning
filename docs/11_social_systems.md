# Conflict, Coordination, and Norms

> **Guiding question:** How do predictive systems interact?

*Canonical file: `11_social_systems.md`*

## Status

**Stub / Planned** — no combat, theft, ownership, or reputation primitives in code. `Creature::signature` exists; `Move` may eventually displace others (planned). Trust emerges from memory edge confidence (planned).

## Summary

Social behavior arises when multiple predictive organisms occupy overlapping futures. They do not share goals — they share **space, traces, and regulatory consequences**. Conflict, cooperation, norms, and reputation are emergent patterns in memory and regulation, not hardcoded social roles.

## Core Principle

> Social life begins when two predictive systems have **incompatible futures**.

Organism A's predicted regulatory trajectory conflicts with B's. Resolution happens through physical interaction, avoidance, coordination signals, or learned suppression — never through a "fight" or "share" opcode.

## Physical Basis

All social dynamics ground in:

- shared voxel space (`position`)
- sensor traces (`contact_occupied`, `chemical_creature`, `sound_calls`)
- regulatory costs (energy, integrity, fatigue)
- memory edges per creature (`memory/graph.rs`)

No social layer bypasses physics or sensors.

## Pushing

**Planned** — `Move` into an occupied void cell may displace another creature if morphology/mass favors it. Displacement costs energy and integrity for both parties. No "attack" action type.

## Costs of Conflict

Conflict drains `energy`, raises `fatigue`, and may reduce `integrity`. Prolonged incompatible futures are metabolically expensive — biasing avoidance and prediction without a fear concept.

## Exclusion

**Planned** — occupying space or emitting traces that reliably precede negative regulatory outcomes for others. Researchers may label this "territory" — creatures experience contact and chemical gradients only.

## Theft

**Emergent, not a mechanic.** Taking organic fraction or deposited material another creature relied on is `ConsumeOrganic` or `Drop` on shared voxels — no ownership variable. "Theft" is a researcher label when one organism's prediction error follows another's gain.

## Defense

**Planned** — blocking movement, reinforcing contact traces, or emitting signals that precede displacement. No combat stats or armor types.

## Cooperation

**Planned** — actions that improve another's regulatory state while improving one's own (or accepting short-term cost for predicted long-term gain). Emerges from `action_leads_to` edges linking joint traces to outcomes — not altruism flags.

## Coordination

**Planned** — synchronized movement or construction ([12_construction_and_infrastructure.md](12_construction_and_infrastructure.md)) when shared predictions align. Attention signals ([10_communication.md](10_communication.md)) may bias coordination.

## Trust

Trust emerges from prediction confidence on conspecific signals — see [Communication § Trust](10_communication.md#trust). Social systems add reputation (per-`signature` subgraphs) and norm formation on top of that mechanism.

## Punishment

**Planned** — behaviors that impose regulatory cost on others who violated predicted patterns. No moral engine — only prediction error and response.

## Reputation

**Planned** — per-`signature` subgraphs in memory aggregating past regulatory outcomes following encounters. Reputation is compressed prediction about a conspecific, not a score.

## Norms

**Planned** — population-stable patterns of suppressed or encouraged actions when certain sensory-social traces co-occur. Norms are **shared predictive structure**, not rules. See [16_culture.md](16_culture.md).

## Social Stability and Breakdown

Stability when predictions align and regulatory costs stay low. Breakdown when environment shifts, communication drifts, or resource traces destabilize — increasing conflict costs and memory restructuring.

## Conflict and Meaning

Researchers may describe emergent labels: **trust**, **danger**, **ally**, **outsider**. These are post-hoc names for predictive relationships — never creature-facing concepts. Meaning remains predictive relevance to **regulation** ([04_regulation.md](04_regulation.md)).

## Core Principle

> Social order is what happens when predictive systems learn to live with each other's futures.

## Current implementation

| Component | Location | Status |
|-----------|----------|--------|
| `signature` | `creatures/creature.rs` | Per-creature ID at spawn |
| `contact_occupied`, `chemical_creature` | `sensors.rs` | Always 0.0 in skeleton |
| `Move` | `actions.rs` | No displacement |
| Edge `confidence` | `memory/edges.rs` | Trust proxy (unused socially) |
| Combat / theft / ownership | — | Not implemented |

## Planned

- Push via `Move` displacement
- Per-signature memory subgraphs
- Reputation from aggregated outcomes
- Norm formation from population-stable edges
- Social prediction in [10_communication.md](10_communication.md)

## Open questions

- Can norms emerge without explicit punishment, only via prediction error?
- Minimum population size for stable reputation?
