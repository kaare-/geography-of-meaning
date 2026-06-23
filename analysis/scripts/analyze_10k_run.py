#!/usr/bin/env python3
"""Analyze 10k tick log and memory exports."""
import json
import statistics
from collections import Counter, defaultdict
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
TICK_LOG = ROOT / "exports/logs/tick_log.jsonl"
WORLD_FINAL = ROOT / "exports/snapshots/world_final.json"
MEMORY_BEST = ROOT / "exports/snapshots/memory_creature_best_tick_10000.json"
TIMING = ROOT / "exports/logs/timing_10k.csv"


def load_ticks():
    entries = []
    with open(TICK_LOG) as f:
        for line in f:
            entries.append(json.loads(line))
    return entries


def analyze_actions(entries):
    totals = Counter()
    for e in entries:
        ac = e.get("action_counts", {})
        for k, v in ac.items():
            totals[k.replace("_count", "")] += v
    return totals


def analyze_memory(path):
    with open(path) as f:
        data = json.load(f)
    node_count = data.get("node_count", len(data.get("nodes", [])))
    edge_count = data.get("edge_count", 0)
    # Full node scan is slow on 15k+ node exports; prefer summary fields when present.
    kinds = Counter()
    if node_count <= 5000:
        for n in data.get("nodes", []):
            k = n.get("kind")
            if isinstance(k, str):
                kinds[k] += 1
            elif isinstance(k, dict):
                kinds[list(k.keys())[0]] += 1
    else:
        # Heuristic from known 10k run structure when nodes omitted or huge
        kinds = Counter(data.get("node_kinds", {}))
    concepts = kinds.get("Concept", 0)
    sensory = kinds.get("SensoryPattern", 0)
    return {
        "creature_id": data.get("creature_id"),
        "node_count": node_count,
        "edge_count": edge_count,
        "kinds": kinds,
        "concepts_in_graph": concepts,
        "sensory_nodes": sensory,
        "compression_ratio": concepts / max(node_count, 1),
    }


def main():
    entries = load_ticks()
    print(f"=== TICK LOG ({len(entries)} entries) ===\n")

    action_totals = analyze_actions(entries)
    print("Action totals:")
    for a, c in action_totals.most_common():
        print(f"  {a}: {c}")

    transfer_total = sum(e.get("transfer_count", 0) for e in entries)
    transfer_action = action_totals.get("transfer_organic", 0)
    print(f"\nOrganic transfers (transfer_count): {transfer_total}")
    print(f"transfer_organic action attempts logged: {transfer_action}")

    deaths = Counter()
    for e in entries:
        for d in e.get("deaths", []):
            deaths[d.get("cause", str(d))] += 1
    print("\nDeath causes:", dict(deaths))

    print("\nRun aggregates:")
    print(f"  concepts_formed sum: {sum(e.get('concepts_formed',0) for e in entries)}")
    print(f"  concept_merge sum: {sum(e.get('concept_merge_count',0) for e in entries)}")
    print(f"  concept_split sum: {sum(e.get('concept_split_count',0) for e in entries)}")
    print(f"  imagination sum: {sum(e.get('imagination_events',0) for e in entries)}")
    print(f"  mean displacement: {statistics.mean(e.get('mean_displacement',0) for e in entries):.4f}")
    print(f"  mean novel sensor: {statistics.mean(e.get('novel_sensor_fraction',0) for e in entries):.6f}")

    # Hydration / energy from final tick creature snapshots (flat CreatureSnapshot fields)
    last = entries[-1]
    creatures = last.get("creatures", [])
    if creatures:
        hydr = [c["hydration"] for c in creatures]
        energy = [c["energy"] for c in creatures]
        carried = [c.get("carried_mass", 0) for c in creatures]
        print(f"\nFinal tick creatures ({len(creatures)}):")
        print(f"  hydration: min={min(hydr):.3f} mean={statistics.mean(hydr):.3f} max={max(hydr):.3f}")
        print(f"  energy: min={min(energy):.3f} mean={statistics.mean(energy):.3f} max={max(energy):.3f}")
        print(f"  carried_mass: max={max(carried):.4f} mean={statistics.mean(carried):.4f}")

    # World final
    if WORLD_FINAL.exists():
        with open(WORLD_FINAL) as f:
            world = json.load(f)
        wc = world.get("creatures", [])
        print(f"\nworld_final creatures: {len(wc)}")
        if wc:
            wh = [c["hydration"] for c in wc]
            we = [c["energy"] for c in wc]
            wcar = [c.get("carried_mass", 0) for c in wc]
            print(
                f"  hydration min/mean/max: {min(wh):.3f} / {statistics.mean(wh):.3f} / {max(wh):.3f}"
            )
            print(
                f"  energy min/mean/max: {min(we):.3f} / {statistics.mean(we):.3f} / {max(we):.3f}"
            )
            print(f"  carried_mass max: {max(wcar):.4f}")

    # Memory export (skip full parse of huge JSON dumps)
    if MEMORY_BEST.exists() and MEMORY_BEST.stat().st_size < 2_000_000:
        mem = analyze_memory(MEMORY_BEST)
        print(f"\n=== BEST CREATURE MEMORY (tick 10000 export) ===")
        print(f"  id={mem['creature_id']} nodes={mem['node_count']} edges={mem['edge_count']}")
        if mem["kinds"]:
            print(f"  concept nodes in graph: {mem['concepts_in_graph']}")
            print(f"  sensory nodes: {mem['sensory_nodes']}")
            print(f"  compression (concepts/nodes): {mem['compression_ratio']:.4f}")
            print("  node kind breakdown:")
            for k, v in mem["kinds"].most_common():
                print(f"    {k}: {v}")
    elif MEMORY_BEST.exists():
        print(f"\n=== BEST CREATURE MEMORY export ===")
        print(f"  (skipped full load — {MEMORY_BEST.stat().st_size // 1024} KB; see world_final summaries)")
    if WORLD_FINAL.exists():
        with open(WORLD_FINAL) as f:
            world = json.load(f)
        best = max(world.get("creatures", []), key=lambda c: c.get("memory_node_count", 0), default=None)
        if best:
            print(f"\n=== LARGEST MEMORY (world_final id={best['id']}) ===")
            print(f"  nodes={best['memory_node_count']} edges={best['memory_edges']} concepts={best['concept_count']}")
            print(f"  by_type: {best.get('memory_nodes_by_type', {})}")

    # Timing trend
    if TIMING.exists():
        rows = []
        with open(TIMING) as f:
            header = f.readline().strip().split(",")
            for line in f:
                parts = line.strip().split(",")
                if len(parts) >= len(header):
                    rows.append(dict(zip(header, parts)))
        for tick in ["100", "1000", "2000", "5000", "8000", "10000"]:
            r = next((x for x in rows if x.get("tick") == tick), None)
            if r:
                print(f"\nTiming tick {tick}: total={r.get('total_tick_ms')}ms prediction={r.get('prediction_ms')}ms sleep={r.get('sleep_ms')}ms")


if __name__ == "__main__":
    main()
