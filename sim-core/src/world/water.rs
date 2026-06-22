use super::chunk::Chunk;
use super::voxel::{idx, CHUNK_SIZE, CHUNK_VOLUME};

pub fn surface_water_total(chunk: &Chunk) -> f32 {
    chunk.fields.surface_water.iter().sum()
}

const MAX_FLOW: f32 = 0.15;
const INFILTRATION_RATE: f32 = 0.05;
const EVAPORATION_RATE: f32 = 0.02;

pub fn apply_rain(chunk: &mut Chunk, amount: f32) {
    for z in (0..CHUNK_SIZE).rev() {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let i = idx(x, y, z);
                if chunk.fields.void_fraction[i] > 0.5 {
                    chunk.fields.surface_water[i] += amount;
                } else if chunk.fields.solid_fraction[i] > 0.3 && chunk.fields.void_fraction[i] > 0.1
                {
                    chunk.fields.surface_water[i] += amount * 0.5;
                }
            }
        }
    }
}

pub fn flow_surface_water(chunk: &mut Chunk) {
    let mut transfers: Vec<(usize, usize, f32)> = Vec::new();

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let i = idx(x, y, z);
                let water = chunk.fields.surface_water[i];
                if water < 0.05 {
                    continue;
                }

                let mut lowest_neighbor = None;
                let mut lowest_z = z;

                for (nx, ny, nz) in neighbors_6(x, y, z) {
                    let ni = idx(nx, ny, nz);
                    let nz_u = nz as i32;
                    if nz_u < lowest_z as i32 {
                        lowest_z = nz;
                        lowest_neighbor = Some(ni);
                    }
                }

                if let Some(target) = lowest_neighbor {
                    let flow = (water * 0.25).min(MAX_FLOW);
                    transfers.push((i, target, flow));
                }
            }
        }
    }

    for (from, to, amount) in transfers {
        chunk.fields.surface_water[from] = (chunk.fields.surface_water[from] - amount).max(0.0);
        chunk.fields.surface_water[to] += amount;
    }
}

pub fn infiltrate(chunk: &mut Chunk) {
    for i in 0..CHUNK_VOLUME {
        let permeability = chunk.fields.permeability[i];
        let transfer = chunk.fields.surface_water[i] * INFILTRATION_RATE * permeability;
        if transfer > 0.0 {
            chunk.fields.surface_water[i] -= transfer;
            chunk.fields.water_content[i] += transfer;
        }
    }
}

pub fn evaporate(chunk: &mut Chunk, global_humidity: f32) {
    for i in 0..CHUNK_VOLUME {
        let temp_factor = (chunk.fields.temperature[i] / 30.0).clamp(0.1, 1.5);
        let evap = chunk.fields.surface_water[i] * EVAPORATION_RATE * temp_factor;
        if evap > 0.0 {
            chunk.fields.surface_water[i] -= evap;
            chunk.fields.humidity[i] = (chunk.fields.humidity[i] + evap * 0.1).min(1.0);
        }
        let _ = global_humidity;
    }
}

pub fn freeze_melt(chunk: &mut Chunk) {
    for i in 0..CHUNK_VOLUME {
        let temp = chunk.fields.temperature[i];
        if temp < 0.0 && chunk.fields.surface_water[i] > 0.01 {
            let freeze = chunk.fields.surface_water[i] * 0.1;
            chunk.fields.surface_water[i] -= freeze;
            chunk.fields.ice[i] += freeze;
        } else if temp > 2.0 && chunk.fields.ice[i] > 0.01 {
            let melt = chunk.fields.ice[i] * 0.1;
            chunk.fields.ice[i] -= melt;
            chunk.fields.surface_water[i] += melt;
        }
        if temp < -2.0 && chunk.fields.surface_water[i] > 0.01 {
            let snow = chunk.fields.surface_water[i] * 0.05;
            chunk.fields.surface_water[i] -= snow;
            chunk.fields.snow[i] += snow;
        } else if temp > 0.0 && chunk.fields.snow[i] > 0.01 {
            let melt = chunk.fields.snow[i] * 0.1;
            chunk.fields.snow[i] -= melt;
            chunk.fields.surface_water[i] += melt;
        }
    }
}

pub fn tick_water(chunk: &mut Chunk, global_humidity: f32) {
    flow_surface_water(chunk);
    infiltrate(chunk);
    evaporate(chunk, global_humidity);
    freeze_melt(chunk);
}

fn neighbors_6(x: usize, y: usize, z: usize) -> Vec<(usize, usize, usize)> {
    let dirs = [
        (1i32, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let mut out = Vec::new();
    for (dx, dy, dz) in dirs {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        let nz = z as i32 + dz;
        if nx >= 0
            && ny >= 0
            && nz >= 0
            && (nx as usize) < CHUNK_SIZE
            && (ny as usize) < CHUNK_SIZE
            && (nz as usize) < CHUNK_SIZE
        {
            out.push((nx as usize, ny as usize, nz as usize));
        }
    }
    out
}
