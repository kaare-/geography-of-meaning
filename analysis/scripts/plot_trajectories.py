#!/usr/bin/env python3
"""Render creature trajectories from sim-core CSV logs as SVG (stdlib only)."""

from __future__ import annotations

import argparse
import csv
import sys
from collections import defaultdict
from pathlib import Path


PALETTE = [
    "#e41a1c",
    "#377eb8",
    "#4daf4a",
    "#984ea3",
    "#ff7f00",
    "#a65628",
    "#f781bf",
    "#999999",
    "#66c2a5",
    "#fc8d62",
    "#8da0cb",
    "#e78ac3",
]


def load_trajectories(path: Path) -> dict[int, list[tuple[float, float, int]]]:
    """Return creature_id -> [(x, y, tick), ...]."""
    tracks: dict[int, list[tuple[float, float, int]]] = defaultdict(list)
    with path.open(newline="") as f:
        reader = csv.DictReader(f)
        for row in reader:
            cid = int(row["creature_id"])
            tracks[cid].append(
                (float(row["x"]), float(row["y"]), int(row["tick"]))
            )
    return dict(tracks)


def bounds(
    tracks: dict[int, list[tuple[float, float, int]]], pad: float = 2.0
) -> tuple[float, float, float, float]:
    xs = [p[0] for pts in tracks.values() for p in pts]
    ys = [p[1] for pts in tracks.values() for p in pts]
    if not xs:
        return 0.0, 1.0, 0.0, 1.0
    return min(xs) - pad, max(xs) + pad, min(ys) - pad, max(ys) + pad


def to_svg(
    tracks: dict[int, list[tuple[float, float, int]]],
    width: int = 900,
    height: int = 700,
) -> str:
    xmin, xmax, ymin, ymax = bounds(tracks)
    span_x = max(xmax - xmin, 1e-6)
    span_y = max(ymax - ymin, 1e-6)
    margin = 40

    def px(x: float) -> float:
        return margin + (x - xmin) / span_x * (width - 2 * margin)

    def py(y: float) -> float:
        # SVG y grows downward; flip world y-up feel
        return height - margin - (y - ymin) / span_y * (height - 2 * margin)

    lines: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" '
        f'viewBox="0 0 {width} {height}">',
        '<rect width="100%" height="100%" fill="#0f1419"/>',
        f'<text x="{margin}" y="22" fill="#9aa5b1" font-family="monospace" font-size="13">'
        f"Creature trajectories (x–y plane)</text>",
    ]

    for i, (cid, pts) in enumerate(sorted(tracks.items())):
        if len(pts) < 2:
            continue
        color = PALETTE[i % len(PALETTE)]
        d = " ".join(
            f"{'M' if j == 0 else 'L'}{px(x):.2f},{py(y):.2f}"
            for j, (x, y, _) in enumerate(pts)
        )
        lines.append(
            f'<path d="{d}" fill="none" stroke="{color}" stroke-width="1.6" '
            f'stroke-opacity="0.85"/>'
        )
        sx, sy, st = pts[0]
        ex, ey, et = pts[-1]
        lines.append(
            f'<circle cx="{px(sx):.2f}" cy="{py(sy):.2f}" r="4" fill="{color}" '
            f'opacity="0.5"/>'
        )
        lines.append(
            f'<circle cx="{px(ex):.2f}" cy="{py(ey):.2f}" r="5" fill="{color}"/>'
        )
        lines.append(
            f'<text x="{px(ex) + 6:.2f}" y="{py(ey) + 4:.2f}" fill="{color}" '
            f'font-family="monospace" font-size="11">id {cid} (t{st}→{et})</text>'
        )

    lines.append(
        f'<text x="{margin}" y="{height - 12}" fill="#6b7785" font-family="monospace" '
        f'font-size="11">x: {xmin:.1f}…{xmax:.1f}  y: {ymin:.1f}…{ymax:.1f}  '
        f"creatures: {len(tracks)}</text>"
    )
    lines.append("</svg>")
    return "\n".join(lines)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("csv", type=Path, help="trajectory CSV from --trajectory-log")
    parser.add_argument(
        "-o",
        "--output",
        type=Path,
        help="output SVG path (default: same name as CSV with .svg)",
    )
    args = parser.parse_args()
    if not args.csv.is_file():
        print(f"File not found: {args.csv}", file=sys.stderr)
        return 1

    tracks = load_trajectories(args.csv)
    if not tracks:
        print("No trajectory rows found.", file=sys.stderr)
        return 1

    out = args.output or args.csv.with_suffix(".svg")
    out.write_text(to_svg(tracks), encoding="utf-8")
    print(f"Wrote {out} ({len(tracks)} creatures)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
