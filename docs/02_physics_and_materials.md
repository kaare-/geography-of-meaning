# Physics and Materials

> **Guiding question:** How does matter behave?

## Status

**Partial** — material fraction fields and initial terrain assignment are implemented. Collapse, load propagation, and failure tick logic are planned.

## Summary

The physics model prioritizes **believable emergent behavior** over physical accuracy. Matter is described by compositional fractions and scalar state variables that interact over time.

## Philosophy

Simple local rules should produce rockfalls, landslides, cave-ins, and structural failure without special-case geology code. The same rules govern organism-built structures (researcher labels like walls or shelters emerge; they receive no special treatment in simulation).

## Voxel State

Each voxel tracks composition and physical state via `VoxelFields` in `sim-core/src/world/voxel.rs`:

| Concept | Field(s) |
|---------|----------|
| Material composition | `hard_mineral`, `soft_mineral`, `coarse_mineral`, `clay`, `organic`, `binder` |
| Phase / void | `solid_fraction`, `void_fraction` |
| Water | `surface_water`, `water_content`, `ice`, `snow` |
| Thermal / moisture | `temperature`, `humidity` |
| Structure | `porosity`, `permeability`, `structural_strength`, `load`, `erosion_damage` |

## Porosity

Empty space within solid matter. Erosion increases porosity. High porosity transports water and weakens structural integrity.

**Skeleton:** field initialized from clay/coarse mix at terrain gen; no erosion-driven updates yet.

## Permeability

Controls water movement rate. High permeability ≈ sand or fractured rock. Low permeability ≈ clay or compact rock.

**Skeleton:** correlated inversely with clay fraction at init. Used by `water.rs` infiltration.

## Structural Strength

Depends on material mix, water content, binder fraction, and accumulated `erosion_damage`.

**Skeleton:** field set at terrain gen; not yet updated per tick.

## Load

Downward-accumulated force from overlying material. When load exceeds local structural strength → failure.

**Skeleton:** `load` field exists; propagation and failure rules **planned**.

## Collapse

Local strength comparisons produce: rockfalls, cave-ins, avalanches, landslides. No global mesh — only neighbor comparisons.

**Planned** — see `02` future work and `world/` erosion module.

## Loose Material

Transported by water flow, organism movement, and deposition. Shares the same fraction fields as bedrock.

## Binder

Organism-produced substance increasing strength, erosion resistance, and adhesion. Enables emergent architecture.

**Skeleton:** `binder` field present; no production or deposition logic yet.

## Emergent Architecture

Structures built by creatures obey the same material rules as natural geology. No special "built" material type.

## Current implementation

- All fields in `VoxelFields` SoA
- Terrain init sets fractions, porosity, permeability, structural_strength
- Water infiltration uses permeability
- `tick_load_physics` in `world/physics.rs`: downward load propagation on erosion interval; collapse reduces `solid_fraction`, increases `void_fraction` and `erosion_damage`
- Engine applies integrity loss when creature stands in high-`erosion_damage` voxel after physics tick

## Planned

- Erosion-driven porosity and strength reduction beyond collapse
- Binder production and deposition via creature actions
- Avalanche / loose-material transport

## Open questions

- Explicit density field or derive from mineral fractions?
- Arch and roof stability as separate checks or emergent from load propagation?
