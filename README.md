# Geography of Meaning

A simulation-art project investigating how **meaning emerges from prediction, memory, and action** in a dynamic voxel world. Organisms regulate internal state through noisy sensor traces — never direct access to world variables.

**Full vision:** [docs/00_project_overview.md](docs/00_project_overview.md)  
**Documentation index:** [docs/README.md](docs/README.md)

## Quick start

```bash
# Run simulation (100 ticks, 2x2 chunks, 5 creatures)
cargo run -- --ticks 100 --seed 42 --world-size 2 --creatures 5 --output exports

# Python analysis
python3 -m venv .venv && source .venv/bin/activate
pip install -r requirements.txt
python analysis/scripts/load_snapshot.py exports/snapshots/world_final.json
```

Exports land in `exports/snapshots/world_final.json` and `exports/logs/tick_log.jsonl`.

## Repository layout

- `sim-core/` — Rust simulation engine
- `analysis/` — Python notebooks and scripts
- `docs/` — Design documentation (00–28; see [docs/README.md](docs/README.md))
- `exports/` — Runtime output (gitignored)

## Design constraint

Creature cognition never hardcodes concepts like food, shelter, wall, or river. These may appear only as researcher labels in analysis.
