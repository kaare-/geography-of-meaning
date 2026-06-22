# Regulation

> **Guiding question:** What gives experience value?

## Status

**Partial** — `RegulatoryState` with six variables, passive metabolism drain, and action costs are implemented. Full environmental coupling (temperature stress from climate, hydration from water traces) and prediction of future regulation are planned.

## Summary

Regulation is the **foundation of value** in the simulation. A creature does not care about food, shelter, communication, or geography directly — it cares about maintaining internal variables within viable ranges. Everything else (sensors, memory, concepts, prediction) exists in service of **future regulatory state**.

## Core Principle

> Prediction predicts future **regulation**, not the world.

The world matters only insofar as it changes energy, hydration, temperature stress, integrity, and fatigue. Memory is significant when it improves forecasts of those variables.

## Regulatory Variables

Implemented in `sim-core/src/creatures/regulation.rs` as `RegulatoryState`:

| Variable | Range | Consumption / replenishment | Influences |
|----------|-------|------------------------------|------------|
| **Energy** | 0–1 | Passive drain via `tick_passive_drain(metabolism)`; replenished by `ConsumeOrganic` action | Action availability, survival |
| **Hydration** | 0–1 | Passive drain (half metabolism rate) | Planned: water_content / surface_water traces |
| **Temperature stress** | 0–1 | Partial: sensor-derived thermal deviation + regulatory field | Planned: climate, shelter traces (researcher label) |
| **Fatigue** | 0–1 | Increases with metabolism and action costs; reduced by `Rest` | Rest action weighting in `choose_action` |
| **Integrity** | 0–1 | Default 1.0; damage not yet applied | Planned: collapse, collision |
| **Carried mass** | ≥0 | Increases with carry actions (planned) | Planned: movement cost |

Methods: `tick_passive_drain`, `apply_action_cost`, `clamp`.

## Prediction

**Planned** ([08_prediction.md](08_prediction.md)) — traverse memory graph to estimate future `RegulatoryState` before choosing actions. Outcome of prediction is regulatory delta, not world state.

## Meaning

Meaning is **predictive relevance to regulation** ([00_project_overview.md](00_project_overview.md)). A sensory pattern is meaningful when activating its memory subgraph changes expected future energy, hydration, or other regulatory variables.

`Experience.outcome` in skeleton uses delta **energy** as scalar prediction-error placeholder.

## Core Principle

> Regulation gives memory its significance. Without regulatory consequence, experience would not persist as weighted relationships.

## Current implementation

| Component | Location |
|-----------|----------|
| `RegulatoryState` | `creatures/regulation.rs` |
| Passive drain each tick | `simulation/engine.rs` |
| Action costs | `creatures/actions.rs` |
| Internal sensor channels | `creatures/sensors.rs` (`internal_energy`, `internal_hydration`, `internal_temperature_stress`) |
| Integrity damage from temperature stress | `regulation.rs` `apply_environmental_stress` |
| Fatigue integrity loss | `regulation.rs` |
| Passive hydration from wet mineral sensor | `regulation.rs` `apply_passive_hydration` |
| Hydration on consume/rest (sensor-based) | `creatures/actions.rs` |
| Death when energy or integrity hits zero | `lifecycle.rs` |

## Open questions

- Single combined "distress" scalar vs independent regulatory dimensions for prediction?
- Should carried_mass cap scale with genome?
