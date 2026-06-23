# Prediction

> **Guiding question:** How does a creature construct a future?

## Status

**Partial** — prediction engine wired into action choice with delay-weighted chains and uncertainty-driven exploration.

## Planned

- Deeper multi-hop prediction chains with explicit delay variance
- Imagination-driven offline prediction (see [09_sleep.md](09_sleep.md))
- Export prediction inspection UI

## Current implementation

| Component | Location |
|-----------|----------|
| `predict_regulatory_delta` | `memory/graph.rs` — sensory match + spread activation weights |
| `predict_action_outcomes` | `memory/graph.rs` — per-action deltas with `delay_mean` weighting; sound-activated paths |
| `prediction_uncertainty` | `memory/graph.rs` — low-confidence paths raise exploration rate |
| `spread_activation` | `memory/graph.rs` — 2-hop along `concept_compresses` / `precedes` / `CoOccurs` |
| Tick order | `engine.rs` — concepts → spread → predict → action (refresh before `choose_action`) |
| Action bias in `choose_action` | `creatures/actions.rs` — uncertainty-adjusted exploration + prediction weight |
| `action_predictions` export | `export/memory_dump.rs` |
| `Experience.outcome` | `creature.rs` — scalar regulatory delta |
