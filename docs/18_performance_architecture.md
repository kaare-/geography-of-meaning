# Performance Architecture

> **Guiding question:** How do we simulate millions of years efficiently?

## Status

**Partial** — data-oriented design and multithreading-ready structure; parallel updates not yet enabled.

## Principles

- **Structure-of-Arrays** voxel storage for cache-friendly iteration
- **Chunked world** with `active_chunks` set for sparse updates
- **Pure tick functions** over borrowed data — `rayon` ready
- **Downsampled export** — 2D slice per chunk, not full 16³ every tick
- **Multi-timescale scheduling** — planned coupling with [14_time_and_scales.md](14_time_and_scales.md)

## Current implementation

| Aspect | Status |
|--------|--------|
| SoA voxels | Implemented |
| Chunk grid | Implemented |
| Active chunk set | Populated; full update in skeleton |
| `rayon` dependency | Present, unused |
| JSON export | Slice + final snapshot |
| Single-threaded tick | Implemented |

## Planned

- Parallel per-chunk climate/water updates
- GPU field updates for large worlds
- Incremental / streaming serialization
- Memory graph compression for scale
- LOD terrain for distant chunks
- Geological fast-forward (many ticks per wall-clock second)

## Open questions

- Chunk activation driven by creature proximity vs change detection?
- Binary snapshot format (e.g. Arrow) for large runs?
