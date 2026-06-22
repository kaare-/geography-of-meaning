#!/usr/bin/env python3
"""Load and summarize a world snapshot export."""

from typing import Optional
import argparse
import json
import sys
from pathlib import Path


def load_snapshot(path: Path) -> dict:
    with path.open() as f:
        return json.load(f)


def load_tick_log(path: Path) -> list[dict]:
    entries = []
    with path.open() as f:
        for line in f:
            line = line.strip()
            if line:
                entries.append(json.loads(line))
    return entries


def summarize_tick_log(entries: list[dict]) -> None:
    if not entries:
        print("Tick log: (empty)")
        return
    final = entries[-1]
    total_births = sum(len(e.get("births", [])) for e in entries)
    total_deaths = sum(len(e.get("deaths", [])) for e in entries)
    final_pop = len(final.get("creatures", []))
    total_sounds = sum(e.get("sound_event_count", 0) for e in entries)
    print(f"Tick log entries: {len(entries)}")
    print(f"Final population: {final_pop}")
    print(f"Total births:     {total_births}")
    print(f"Total deaths:     {total_deaths}")
    print(f"Sound events:     {total_sounds} (sum of per-tick counts)")


def summarize(data: dict, snapshot_path: Optional[Path] = None) -> None:
    chunks = data.get("chunks", [])
    creatures = data.get("creatures", [])
    print(f"Time:      {data.get('time')}")
    print(f"Season:    {data.get('season')}")
    print(f"Chunks:    {len(chunks)}")
    print(f"Creatures: {len(creatures)}")
    if chunks:
        c0 = chunks[0]
        organic = c0.get("organic", [])
        print(f"Slice z:   {c0.get('slice_z')} (chunk {c0.get('coord')})")
        print(f"Grid size: {len(organic)} x {len(organic[0]) if organic else 0}")
    if creatures:
        c = creatures[0]
        mem = c.get("memory_node_count", c.get("memory_nodes"))
        concept_count = c.get("concept_count", 0)
        active = c.get("active_concepts", [])
        by_type = c.get("memory_nodes_by_type", {})
        print(
            f"Sample creature id={c.get('id')} energy={c.get('energy'):.3f} "
            f"memory_node_count={mem} concept_count={concept_count} "
            f"active_concepts={len(active)}"
        )
        if by_type:
            print(f"  memory by type: {by_type}")

    if snapshot_path is not None:
        memory_files = sorted(
            snapshot_path.parent.glob("memory_creature_*.json"),
            key=lambda p: p.name,
        )
        graphml_files = sorted(
            snapshot_path.parent.glob("memory_creature_*.graphml"),
            key=lambda p: p.name,
        )
        if memory_files:
            print(f"Memory graph export: {memory_files[0]}")
        if graphml_files:
            print(f"GraphML export:      {graphml_files[0]}")


def plot_slice(data: dict, field: str = "organic") -> None:
    try:
        import matplotlib.pyplot as plt
    except ImportError:
        print("matplotlib not installed; skipping plot", file=sys.stderr)
        return

    chunks = data.get("chunks", [])
    if not chunks:
        return
    grid = chunks[0].get(field, [])
    if not grid:
        print(f"Field '{field}' not found", file=sys.stderr)
        return
    plt.imshow(grid, origin="lower", cmap="viridis")
    plt.colorbar(label=field)
    plt.title(f"{field} (slice z={chunks[0].get('slice_z')})")
    plt.show()


def main() -> None:
    parser = argparse.ArgumentParser(description="Load Geography of Meaning snapshot")
    parser.add_argument(
        "path",
        nargs="?",
        default="exports/snapshots/world_final.json",
        help="Path to world_final.json",
    )
    parser.add_argument(
        "--tick-log",
        default=None,
        help="Path to tick_log.jsonl (default: sibling logs/tick_log.jsonl)",
    )
    parser.add_argument("--plot", choices=["organic", "surface_water", "temperature"], help="Show heatmap")
    args = parser.parse_args()

    path = Path(args.path)
    if not path.exists():
        print(f"File not found: {path}", file=sys.stderr)
        sys.exit(1)

    data = load_snapshot(path)
    summarize(data, path)

    tick_log_path = Path(args.tick_log) if args.tick_log else path.parent.parent / "logs" / "tick_log.jsonl"
    if tick_log_path.exists():
        print()
        summarize_tick_log(load_tick_log(tick_log_path))
    elif args.tick_log:
        print(f"Tick log not found: {tick_log_path}", file=sys.stderr)

    if args.plot:
        plot_slice(data, args.plot)


if __name__ == "__main__":
    main()
