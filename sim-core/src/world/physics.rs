use std::collections::{HashMap, HashSet};

use super::chunk::{Chunk, ChunkCoord};
use super::voxel::{idx, CHUNK_SIZE};

const UNIT_WEIGHT: f32 = 0.4;
const COLLAPSE_SOLID_REDUCTION: f32 = 0.12;
const COLLAPSE_VOID_INCREASE: f32 = 0.08;
const COLLAPSE_EROSION_BUMP: f32 = 0.04;

/// Propagate load downward through solid columns and collapse overloaded voxels.
pub fn tick_load_physics(
    chunks: &mut HashMap<ChunkCoord, Chunk>,
    active_chunks: &HashSet<ChunkCoord>,
) {
    let coords: Vec<_> = active_chunks.iter().copied().collect();
    for coord in coords {
        if let Some(chunk) = chunks.get_mut(&coord) {
            propagate_load_in_chunk(chunk);
        }
    }
}

fn propagate_load_in_chunk(chunk: &mut Chunk) {
    for x in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            let mut accumulated = 0.0f32;
            for z in (0..CHUNK_SIZE).rev() {
                let i = idx(x, y, z);
                let solid = chunk.fields.solid_fraction[i];
                accumulated += solid * UNIT_WEIGHT;
                chunk.fields.load[i] = accumulated;

                if accumulated > chunk.fields.structural_strength[i] && solid > 0.05 {
                    chunk.fields.solid_fraction[i] =
                        (solid - COLLAPSE_SOLID_REDUCTION).max(0.0);
                    chunk.fields.void_fraction[i] = (chunk.fields.void_fraction[i]
                        + COLLAPSE_VOID_INCREASE)
                        .min(1.0);
                    chunk.fields.erosion_damage[i] = (chunk.fields.erosion_damage[i]
                        + COLLAPSE_EROSION_BUMP)
                        .min(1.0);
                    accumulated *= 0.85;
                    chunk.fields.load[i] = accumulated;
                }
            }
        }
    }
}
