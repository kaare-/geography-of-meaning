use std::collections::HashMap;

use super::chunk::Chunk;
use super::voxel::{idx, CHUNK_SIZE, CHUNK_VOLUME};

pub fn surface_water_total(chunk: &Chunk) -> f32 {
    chunk.fields.surface_water.iter().sum()
}

const MAX_FLOW: f32 = 0.15;
const INFILTRATION_RATE: f32 = 0.05;
const EVAPORATION_RATE: f32 = 0.02;
const GROUNDWATER_MIN_CONTENT: f32 = 0.15;
const GROUNDWATER_FLOW_RATE: f32 = 0.02;
const GROUNDWATER_MAX_TRANSFER: f32 = 0.02;
const GROUNDWATER_MAX_VOID: f32 = 0.45;
const SEDIMENT_TRANSFER_RATE: f32 = 0.018;
const FLOW_EROSION_RATE: f32 = 0.035;
const DEPOSITION_WATER_THRESHOLD: f32 = 0.1;
const EROSION_PERMEABILITY_BUMP: f32 = 0.015;
const CAVE_PERMEABILITY_THRESHOLD: f32 = 0.38;
const CAVE_WATER_CONTENT_MIN: f32 = 0.35;
const CAVE_VOID_BUMP: f32 = 0.0006;

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

    for (from, to, amount) in &transfers {
        apply_flow_sediment_and_erosion(chunk, *from, *to, *amount);
    }

    for (from, to, amount) in &transfers {
        chunk.fields.surface_water[*from] =
            (chunk.fields.surface_water[*from] - amount).max(0.0);
        chunk.fields.surface_water[*to] += amount;
    }

    deposit_sediment_on_slow_flow(chunk, &transfers);
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

/// Slow horizontal redistribution of deep `water_content` toward lower neighbors.
pub fn flow_groundwater(chunk: &mut Chunk) {
    let mut transfers: Vec<(usize, usize, f32)> = Vec::new();

    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let i = idx(x, y, z);
                let wc = chunk.fields.water_content[i];
                if wc < GROUNDWATER_MIN_CONTENT {
                    continue;
                }
                if chunk.fields.void_fraction[i] > GROUNDWATER_MAX_VOID {
                    continue;
                }

                let mut lowest = wc;
                let mut target = None;
                for (dx, dy) in [(1i32, 0), (-1, 0), (0, 1), (0, -1)] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    if nx < 0
                        || ny < 0
                        || (nx as usize) >= CHUNK_SIZE
                        || (ny as usize) >= CHUNK_SIZE
                    {
                        continue;
                    }
                    let ni = idx(nx as usize, ny as usize, z);
                    if chunk.fields.void_fraction[ni] > GROUNDWATER_MAX_VOID {
                        continue;
                    }
                    let neighbor_wc = chunk.fields.water_content[ni];
                    if neighbor_wc < lowest {
                        lowest = neighbor_wc;
                        target = Some(ni);
                    }
                }

                if let Some(to) = target {
                    let flow = ((wc - lowest) * GROUNDWATER_FLOW_RATE).min(GROUNDWATER_MAX_TRANSFER);
                    if flow > 0.0 {
                        transfers.push((i, to, flow));
                    }
                }
            }
        }
    }

    for (from, to, amount) in &transfers {
        apply_flow_sediment_and_erosion(chunk, *from, *to, *amount);
    }

    for (from, to, amount) in &transfers {
        chunk.fields.water_content[*from] =
            (chunk.fields.water_content[*from] - amount).max(0.0);
        chunk.fields.water_content[*to] += amount;
    }

    deposit_sediment_on_slow_flow(chunk, &transfers);
    apply_cave_seed_feedback(chunk);
}

fn apply_flow_sediment_and_erosion(chunk: &mut Chunk, from: usize, to: usize, flow: f32) {
    if flow <= 0.0 {
        return;
    }
    let organic_move = (chunk.fields.organic[from] * flow * SEDIMENT_TRANSFER_RATE).min(0.004);
    let clay_move = (chunk.fields.clay[from] * flow * SEDIMENT_TRANSFER_RATE).min(0.004);
    chunk.fields.organic[from] = (chunk.fields.organic[from] - organic_move).max(0.0);
    chunk.fields.clay[from] = (chunk.fields.clay[from] - clay_move).max(0.0);
    chunk.fields.organic[to] = (chunk.fields.organic[to] + organic_move).min(1.0);
    chunk.fields.clay[to] = (chunk.fields.clay[to] + clay_move).min(1.0);

    let erosion = flow * FLOW_EROSION_RATE;
    let from_before = chunk.fields.erosion_damage[from];
    let to_before = chunk.fields.erosion_damage[to];
    chunk.fields.erosion_damage[from] = (from_before + erosion).min(1.0);
    chunk.fields.erosion_damage[to] = (to_before + erosion * 0.35).min(1.0);
    if chunk.fields.erosion_damage[from] > from_before {
        chunk.fields.permeability[from] =
            (chunk.fields.permeability[from] + erosion * EROSION_PERMEABILITY_BUMP).min(1.0);
    }
    if chunk.fields.erosion_damage[to] > to_before {
        chunk.fields.permeability[to] =
            (chunk.fields.permeability[to] + erosion * 0.35 * EROSION_PERMEABILITY_BUMP).min(1.0);
    }
}

/// Placeholder cave seed: high permeability + wet voxels slowly gain void.
fn apply_cave_seed_feedback(chunk: &mut Chunk) {
    for i in 0..CHUNK_VOLUME {
        if chunk.fields.permeability[i] < CAVE_PERMEABILITY_THRESHOLD {
            continue;
        }
        if chunk.fields.water_content[i] < CAVE_WATER_CONTENT_MIN {
            continue;
        }
        if chunk.fields.void_fraction[i] >= GROUNDWATER_MAX_VOID {
            continue;
        }
        chunk.fields.void_fraction[i] =
            (chunk.fields.void_fraction[i] + CAVE_VOID_BUMP).min(GROUNDWATER_MAX_VOID);
    }
}

/// When inflow exceeds outflow, suspended sediment settles as coarse mineral and clay.
fn deposit_sediment_on_slow_flow(chunk: &mut Chunk, transfers: &[(usize, usize, f32)]) {
    let mut outflow: HashMap<usize, f32> = HashMap::new();
    let mut inflow: HashMap<usize, f32> = HashMap::new();
    for (from, to, amount) in transfers {
        *outflow.entry(*from).or_default() += amount;
        *inflow.entry(*to).or_default() += amount;
    }

    for i in 0..CHUNK_VOLUME {
        let water = chunk.fields.surface_water[i].max(chunk.fields.water_content[i]);
        if water < DEPOSITION_WATER_THRESHOLD {
            continue;
        }
        let out = outflow.get(&i).copied().unwrap_or(0.0);
        let inp = inflow.get(&i).copied().unwrap_or(0.0);
        if inp <= out {
            continue;
        }
        let settle = ((inp - out) * 0.01).min(0.006);
        chunk.fields.coarse_mineral[i] =
            (chunk.fields.coarse_mineral[i] + settle * 0.55).min(1.0);
        chunk.fields.clay[i] = (chunk.fields.clay[i] + settle * 0.45).min(1.0);
    }
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
