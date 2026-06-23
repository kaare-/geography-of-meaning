# Analysis and Visualization

> **Guiding question:** How do we understand what emerged?

## Status

**Partial** — JSON snapshot export, tick logs, and Python loader script.

## Exports

| Output | Path | Contents |
|--------|------|----------|
| World snapshot | `exports/snapshots/world_final.json` | Time, season, 2D slice per chunk, creature states |
| Tick log | `exports/logs/tick_log.jsonl` | Per-tick creature regulatory + sensor summary |

Implemented in `sim-core/src/export/`.

## Snapshot schema

- `WorldSnapshot`: time, season, chunk_size, chunks[], creatures[]
- `ChunkSnapshot`: coord, slice_z, 2D grids (organic, surface_water, temperature, solid_fraction)
- `CreatureSnapshot`: position, regulatory scalars, sensor state, memory node/edge counts

## Python tools

```bash
python analysis/scripts/load_snapshot.py exports/snapshots/world_final.json
python analysis/scripts/load_snapshot.py --plot organic
```

## Planned

- Memory graph export per creature
- Concept inspection and genealogy
- Migration and infrastructure maps
- Narrative extraction and interpretation ([27_narrative_extraction_and_interpretation.md](27_narrative_extraction_and_interpretation.md))

## Current implementation

| File | Role |
|------|------|
| `export/snapshots.rs` | Serde DTOs |
| `export/logs.rs` | JSONL tick entries |
| `export/mod.rs` | `export_all()` |
| `analysis/scripts/load_snapshot.py` | Load and summarize |
| `analysis/notebooks/explore_run.ipynb` | Population, organic heatmap, concept counts, sound/transfer time series, narrative concept spikes |

## Cross-references

Researcher labels (river, shelter, food cache) applied only in analysis — never in creature code. See [15_information_storage_and_external_memory.md](15_information_storage_and_external_memory.md).
