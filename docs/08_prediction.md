# Prediction

> **Guiding question:** How does a creature construct a future?

## Status

**Stub** — no prediction engine in skeleton.

## Overview

Prediction traverses directed memory edges to estimate future sensory and regulatory states before action selection. Spreading activation, confidence weighting, and uncertainty are core mechanisms.

## Planned

- Active concept activation
- Spreading activation along `precedes` / `action_leads_to` edges
- Prediction chains with delay statistics
- Imagination (offline activation)
- Integration with action choice in `creatures/actions.rs`

## Current implementation

| Component | Location |
|-----------|----------|
| `predict_regulatory_delta` | `memory/graph.rs` |
| `predict_action_outcomes` | `memory/graph.rs` |
| Action bias in `choose_action` | `creatures/actions.rs` (15% exploration retained) |
| `Experience.outcome` | `creature.rs` — scalar regulatory delta |
