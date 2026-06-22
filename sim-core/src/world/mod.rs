pub mod chunk;
pub mod climate;
pub mod event;
pub mod material;
pub mod physics;
pub mod sound;
pub mod voxel;
pub mod water;

pub use chunk::{Chunk, ChunkCoord};
pub use sound::SoundEvent;

use std::collections::{HashMap, HashSet};

use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::math::{Vec3f, Vec3i};

use self::climate::{apply_climate_to_chunk, GlobalClimate};
use self::event::WorldEvent;
use self::material::{chunk_height_slice, fill_terrain, generate_height_map};
use self::physics::tick_load_physics;
use self::voxel::{idx, CHUNK_SIZE, VoxelView};
use self::water::{apply_rain, surface_water_total, tick_water};

/// Ticks per simulated day for diurnal phase stub.
pub const TICKS_PER_DAY: u64 = 100;

/// Surface-water change above this marks a chunk as water-active.
const WATER_ACTIVITY_THRESHOLD: f32 = 0.1;

#[derive(Debug, Clone)]
pub struct World {
    pub chunks: HashMap<ChunkCoord, Chunk>,
    pub active_chunks: HashSet<ChunkCoord>,
    pub time: u64,
    pub day_phase: f32,
    pub season: f32,
    pub climate: GlobalClimate,
    pub event_queue: Vec<WorldEvent>,
    pub active_sounds: Vec<SoundEvent>,
    rain_chunks_this_tick: HashSet<ChunkCoord>,
    water_active_chunks: HashSet<ChunkCoord>,
}

impl World {
    pub fn generate_terrain(size_chunks: usize, seed: u64) -> Self {
        let mut rng = StdRng::seed_from_u64(seed);
        let height_data = generate_height_map(&mut rng, size_chunks);
        let mut chunks = HashMap::new();
        let mut active_chunks = HashSet::new();

        for cy in 0..size_chunks {
            for cx in 0..size_chunks {
                let coord = ChunkCoord::new(cx as i32, cy as i32, 0);
                let mut chunk = Chunk::new(coord);
                let height_map = chunk_height_slice(&height_data, cx, cy);
                fill_terrain(&mut chunk, coord.x, coord.y, &height_map);
                active_chunks.insert(coord);
                chunks.insert(coord, chunk);
            }
        }

        Self {
            chunks,
            active_chunks,
            time: 0,
            day_phase: 0.0,
            season: 0.0,
            climate: GlobalClimate::default(),
            event_queue: Vec::new(),
            active_sounds: Vec::new(),
            rain_chunks_this_tick: HashSet::new(),
            water_active_chunks: HashSet::new(),
        }
    }

    pub fn emit_sound(&mut self, event: SoundEvent) {
        self.active_sounds.push(event);
    }

    pub fn tick_sounds(&mut self) {
        for sound in &mut self.active_sounds {
            sound.age += 1;
        }
        self.active_sounds.retain(|s| s.is_active());
    }

    pub fn active_sound_count(&self) -> usize {
        self.active_sounds.len()
    }

    pub fn tick_climate_and_water(&mut self) {
        self.climate.tick();
        self.season = self.climate.season;
        self.day_phase = (self.time % TICKS_PER_DAY) as f32 / TICKS_PER_DAY as f32;

        let humidity = self.climate.humidity;
        let coords: Vec<_> = self.active_chunks.iter().copied().collect();
        let mut water_active = self.rain_chunks_this_tick.clone();

        for coord in coords {
            if let Some(chunk) = self.chunks.get_mut(&coord) {
                let before = surface_water_total(chunk);
                apply_climate_to_chunk(&self.climate, chunk);
                tick_water(chunk, humidity);
                let after = surface_water_total(chunk);
                if (after - before).abs() > WATER_ACTIVITY_THRESHOLD {
                    water_active.insert(coord);
                }
            }
        }

        self.water_active_chunks = water_active;
        self.rain_chunks_this_tick.clear();
        self.time += 1;
    }

    /// Slow geological tick: load propagation and collapse on active chunks.
    pub fn tick_erosion(&mut self, nudge: f32) {
        tick_load_physics(&mut self.chunks, &self.active_chunks);
        let coords: Vec<_> = self.active_chunks.iter().copied().collect();
        for coord in coords {
            if let Some(chunk) = self.chunks.get_mut(&coord) {
                for value in &mut chunk.fields.erosion_damage {
                    *value = (*value + nudge).min(1.0);
                }
            }
        }
    }

    pub fn process_events(&mut self) {
        let events: Vec<_> = self.event_queue.drain(..).collect();
        for event in events {
            match event {
                WorldEvent::Rain { amount } => {
                    let coords: Vec<_> = self.active_chunks.iter().copied().collect();
                    for coord in coords {
                        if let Some(chunk) = self.chunks.get_mut(&coord) {
                            apply_rain(chunk, amount);
                            self.rain_chunks_this_tick.insert(coord);
                        }
                    }
                }
            }
        }
    }

    pub fn queue_rain(&mut self, amount: f32) {
        self.event_queue.push(WorldEvent::Rain { amount });
    }

    pub fn refresh_active_chunks(&mut self, creature_positions: impl Iterator<Item = Vec3f>) {
        for pos in creature_positions {
            if let Some((coord, _, _, _)) = self.world_to_chunk(pos.floor_i()) {
                self.active_chunks.insert(coord);
            }
        }
        for coord in &self.water_active_chunks {
            self.active_chunks.insert(*coord);
        }
    }

    pub fn world_to_chunk(&self, pos: Vec3i) -> Option<(ChunkCoord, usize, usize, usize)> {
        if pos.x < 0 || pos.y < 0 || pos.z < 0 {
            return None;
        }
        let cx = pos.x / CHUNK_SIZE as i32;
        let cy = pos.y / CHUNK_SIZE as i32;
        let cz = pos.z / CHUNK_SIZE as i32;
        let coord = ChunkCoord::new(cx, cy, cz);
        let lx = (pos.x % CHUNK_SIZE as i32) as usize;
        let ly = (pos.y % CHUNK_SIZE as i32) as usize;
        let lz = (pos.z % CHUNK_SIZE as i32) as usize;
        if self.chunks.contains_key(&coord) {
            Some((coord, lx, ly, lz))
        } else {
            None
        }
    }

    pub fn sample_voxel(&self, pos: Vec3i) -> Option<VoxelView> {
        let (coord, lx, ly, lz) = self.world_to_chunk(pos)?;
        let chunk = self.chunks.get(&coord)?;
        Some(chunk.get(lx, ly, lz))
    }

    pub fn sample_voxel_mut(&mut self, pos: Vec3i) -> Option<crate::world::voxel::VoxelViewMut<'_>> {
        let (coord, lx, ly, lz) = self.world_to_chunk(pos)?;
        let chunk = self.chunks.get_mut(&coord)?;
        Some(chunk.get_mut(lx, ly, lz))
    }

    pub fn find_spawn_positions(&self, count: usize) -> Vec<Vec3f> {
        let mut candidates = Vec::new();
        for (coord, chunk) in &self.chunks {
            for z in 0..CHUNK_SIZE {
                for y in 0..CHUNK_SIZE {
                    for x in 0..CHUNK_SIZE {
                        let i = idx(x, y, z);
                        let organic = chunk.fields.organic[i];
                        let temp = chunk.fields.temperature[i];
                        let water = chunk.fields.water_content[i] + chunk.fields.surface_water[i];
                        let void_f = chunk.fields.void_fraction[i];
                        if void_f > 0.3 && organic > 0.05 && temp > 18.0 && water > 0.1 {
                            let wx = coord.x * CHUNK_SIZE as i32 + x as i32;
                            let wy = coord.y * CHUNK_SIZE as i32 + y as i32;
                            let wz = coord.z * CHUNK_SIZE as i32 + z as i32;
                            let score = organic * water * (temp / 25.0);
                            candidates.push((score, Vec3f::new(wx as f32, wy as f32, wz as f32)));
                        }
                    }
                }
            }
        }
        candidates.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        candidates
            .into_iter()
            .take(count)
            .map(|(_, pos)| pos)
            .collect()
    }

    /// Boost organic and moisture around Eden spawn sites so early populations can feed.
    pub fn enrich_spawn_site(&mut self, pos: Vec3f) {
        let base = pos.floor_i();
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    let check = Vec3i::new(base.x + dx, base.y + dy, base.z + dz);
                    if let Some(voxel) = self.sample_voxel_mut(check) {
                        *voxel.organic = (*voxel.organic + 0.12).min(0.5);
                        *voxel.water_content = (*voxel.water_content + 0.05).min(0.4);
                    }
                }
            }
        }
    }
}
