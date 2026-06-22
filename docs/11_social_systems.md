# Conflict, Coordination, and Norms

> **Guiding question:** How do predictive systems interact?

*Canonical file: `11_social_systems.md`*

## Status

**Partial** — `Action::Push` displaces occupants when `push_strength` (move_speed + carried_mass) wins; engine blocks overlap and resolves conflicts. No combat, theft, or ownership primitives. Trust emerges from memory edge confidence (planned).

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

**Implemented** — `Action::Push(direction)` attempts entry into an occupied void cell. If pusher `push_strength` exceeds target, target is displaced to an adjacent free voxel and pusher occupies the cell. Both pay energy and fatigue. `Move` fails when the target cell is creature-occupied. `resolve_position_overlaps` runs each tick. Push events optionally appear in `tick_log.jsonl`. No attack action type.

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
| `contact_occupied`, `chemical_creature` | `sensors.rs` | From adjacent creature positions |
| `push_strength` | `creatures/spatial.rs` | `move_speed + carried_mass * 0.6` |
| `Action::Push` | `creatures/actions.rs`, `spatial.rs` | Displacement when strength wins |
| `Move` occupancy check | `spatial.rs` | Blocks entry into occupied voxels |
| Overlap resolution | `spatial.rs` `resolve_position_overlaps` | End-of-tick separation |
| Push events in tick log | `export/logs.rs` | Optional `push_events` array |
| Edge `confidence` | `memory/edges.rs` | Trust proxy via sound→outcome paths and follow bias |
| `trusted_signature_count` | `export/snapshots.rs` | Per-creature count of signatures with positive sound memory |
| Combat / theft / ownership | — | Not implemented |

## Planned

- Per-signature memory subgraphs
- Reputation from aggregated outcomes
- Norm formation from population-stable edges
- Social prediction in [10_communication.md](10_communication.md)

## Open questions

- Can norms emerge without explicit punishment, only via prediction error?
- Minimum population size for stable reputation?
