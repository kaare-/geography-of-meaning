use rand::Rng;

use super::chunk::Chunk;
use super::voxel::{idx, CHUNK_SIZE, CHUNK_VOLUME};

pub fn normalize_fractions(chunk: &mut Chunk) {
    for i in 0..CHUNK_VOLUME {
        let solid = chunk.fields.solid_fraction[i].clamp(0.0, 1.0);
        chunk.fields.solid_fraction[i] = solid;
        chunk.fields.void_fraction[i] = (1.0 - solid).max(0.0);
    }
}

pub fn fill_terrain(chunk: &mut Chunk, world_x: i32, world_y: i32, height_map: &[[f32; CHUNK_SIZE]; CHUNK_SIZE]) {
    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let wx = world_x * CHUNK_SIZE as i32 + x as i32;
                let wy = world_y * CHUNK_SIZE as i32 + y as i32;
                let height = height_map[x][y];
                let i = idx(x, y, z);

                if (z as f32) < height - 2.0 {
                    chunk.fields.hard_mineral[i] = 0.4;
                    chunk.fields.soft_mineral[i] = 0.2;
                    chunk.fields.coarse_mineral[i] = 0.3;
                    chunk.fields.clay[i] = 0.05;
                    chunk.fields.organic[i] = 0.02;
                    chunk.fields.binder[i] = 0.03;
                    chunk.fields.solid_fraction[i] = 0.95;
                    chunk.fields.void_fraction[i] = 0.05;
                    chunk.fields.porosity[i] = 0.15;
                    chunk.fields.permeability[i] = 0.1;
                    chunk.fields.structural_strength[i] = 0.9;
                } else if (z as f32) < height {
                    let depth = height - z as f32;
                    let organic = (0.15 / depth.max(0.5)).min(0.4);
                    let clay = if height < 6.0 { 0.25 } else { 0.1 };
                    let coarse = 0.2;
                    chunk.fields.hard_mineral[i] = 0.1;
                    chunk.fields.soft_mineral[i] = 0.15;
                    chunk.fields.coarse_mineral[i] = coarse;
                    chunk.fields.clay[i] = clay;
                    chunk.fields.organic[i] = organic;
                    chunk.fields.binder[i] = 0.05;
                    chunk.fields.solid_fraction[i] = 0.7;
                    chunk.fields.void_fraction[i] = 0.3;
                    chunk.fields.porosity[i] = 0.35 + clay * 0.3;
                    chunk.fields.permeability[i] = 0.25 * (1.0 - clay);
                    chunk.fields.structural_strength[i] = 0.5;
                    chunk.fields.water_content[i] = if height < 5.0 { 0.2 } else { 0.05 };
                    chunk.fields.humidity[i] = if height < 5.0 { 0.7 } else { 0.4 };
                    chunk.fields.temperature[i] = if height < 5.0 { 22.0 } else { 18.0 };
                } else {
                    chunk.fields.solid_fraction[i] = 0.0;
                    chunk.fields.void_fraction[i] = 1.0;
                }

                let _ = wx;
                let _ = wy;
            }
        }
    }
    normalize_fractions(chunk);
}

pub fn generate_height_map<R: Rng + ?Sized>(rng: &mut R, size: usize) -> Vec<Vec<f32>> {
    let grid = size * CHUNK_SIZE;
    let mut raw = vec![vec![0.0f32; grid]; grid];

    for y in 0..grid {
        for x in 0..grid {
            raw[x][y] = rng.gen_range(0.0..1.0);
        }
    }

    // Simple smoothing pass
    let mut smoothed = raw.clone();
    for _ in 0..3 {
        for y in 1..grid - 1 {
            for x in 1..grid - 1 {
                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        sum += raw[(x as i32 + dx) as usize][(y as i32 + dy) as usize];
                    }
                }
                smoothed[x][y] = sum / 9.0;
            }
        }
        raw = smoothed.clone();
    }

    let mut height_map = vec![vec![0.0f32; CHUNK_SIZE]; CHUNK_SIZE];
    for cy in 0..size {
        for cx in 0..size {
            for y in 0..CHUNK_SIZE {
                for x in 0..CHUNK_SIZE {
                    let gx = cx * CHUNK_SIZE + x;
                    let gy = cy * CHUNK_SIZE + y;
                    height_map[x][y] = 4.0 + smoothed[gx][gy] * 8.0;
                }
            }
        }
    }

    smoothed
}

pub fn chunk_height_slice(
    height_data: &[Vec<f32>],
    chunk_x: usize,
    chunk_y: usize,
) -> [[f32; CHUNK_SIZE]; CHUNK_SIZE] {
    let mut map = [[0.0f32; CHUNK_SIZE]; CHUNK_SIZE];
    for y in 0..CHUNK_SIZE {
        for x in 0..CHUNK_SIZE {
            let gx = chunk_x * CHUNK_SIZE + x;
            let gy = chunk_y * CHUNK_SIZE + y;
            map[x][y] = height_data[gx][gy];
        }
    }
    map
}
