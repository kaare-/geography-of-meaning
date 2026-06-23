# World Generation

> **Guiding question:** How does a landscape emerge and evolve?

## Status

**Partial** — chunked voxel world, terrain init, climate, rainfall, and surface water flow are implemented. Erosion, caves, groundwater tables, and environmental memory are planned.

## Summary

The world is a dynamic voxel landscape. It is not a static map. The world changes continuously through climate, water movement, erosion, deposition, collapse, biological activity, and construction. The landscape is an active participant in the simulation.

## Design Goals

- Generate varied terrain
- Support long-term geological change
- Support biological adaptation
- Support environmental memory
- Support infrastructure construction

## World Representation

Chunked storage: **16 × 16 × 16** voxels per chunk. Each voxel stores material composition and environmental state as Structure-of-Arrays fields in `sim-core/src/world/voxel.rs`.

## Materials

Fractions coexist per voxel: hard mineral, soft mineral, coarse mineral, clay, organic matter, binder. Not binary solid/empty.

## Water

| Form | Field |
|------|-------|
| Surface | `surface_water` |
| Pore | `water_content` |
| Groundwater | `flow_groundwater` every 5 ticks on active chunks |
| Snow / ice | `snow`, `ice` |

Behaviors in `water.rs`: flow downhill, infiltrate, evaporate, freeze/melt stubs.

## Climate

`GlobalClimate`: season, base_temperature, humidity, rainfall_rate. Planned: sunlight, wind, diurnal cycles.

## Erosion, Caves, Organic Growth, Environmental Memory

Fields exist (`erosion_damage`, `porosity`, `permeability`, `organic`). Surface and groundwater flow in `water.rs` transport clay/organic sediment, raise `erosion_damage` along flow paths, and deposit coarse mineral/clay when inflow exceeds outflow. Groundwater erosion bumps `permeability`; high permeability + wet `water_content` slowly raises `void_fraction` (cave seed placeholder). Full cave systems and landscape-as-memory remain **planned**.

Researcher descriptions (channels, trails, caches) refer to emergent patterns — not hardcoded world types.

## Current implementation

| Module | Role |
|--------|------|
| `voxel.rs` | SoA voxel fields |
| `chunk.rs` | 16³ chunk container |
| `material.rs` | Terrain generation |
| `climate.rs` | Global and per-voxel climate |
| `water.rs` | Rain, flow, infiltration, evaporation, groundwater, sediment transport & deposition, erosion→permeability bump, cave-seed void feedback |
| `mod.rs` | `World` struct, tick loop, spawn scan |

## Planned

- Directional flow history encoding in `erosion_damage` (magnitude only today)
- Full cave system growth beyond void-seed placeholder
- Organic growth and death deposition
- Environmental memory via material history
- Active-chunk-only updates

## Open questions

- Single `water_content` vs explicit groundwater layer?
- Per-voxel sunlight or climate-derived irradiance?
- When to split `world/erosion.rs`?
