use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalClimate {
    pub season: f32,
    pub base_temperature: f32,
    pub humidity: f32,
    pub rainfall_rate: f32,
}

impl Default for GlobalClimate {
    fn default() -> Self {
        Self {
            season: 0.0,
            base_temperature: 18.0,
            humidity: 0.6,
            rainfall_rate: 0.02,
        }
    }
}

impl GlobalClimate {
    pub fn tick(&mut self) {
        self.season = (self.season + 0.01) % 1.0;
        let seasonal_offset = (self.season * std::f32::consts::TAU).sin() * 5.0;
        self.base_temperature = 18.0 + seasonal_offset;
        self.humidity = (0.5 + 0.2 * (self.season * std::f32::consts::TAU * 2.0).cos())
            .clamp(0.1, 1.0);
        // Humidity boosts rainfall probability so wet seasons see more frequent rain.
        self.rainfall_rate = (0.015 + self.humidity * 0.025).clamp(0.01, 0.06);
    }
}

use super::chunk::Chunk;
use super::voxel::{idx, CHUNK_SIZE, CHUNK_VOLUME};

pub fn apply_climate_to_chunk(climate: &GlobalClimate, chunk: &mut Chunk) {
    for i in 0..CHUNK_VOLUME {
        let surface_bias = if i % CHUNK_SIZE < 4 { 1.0 } else { 0.3 };
        chunk.fields.temperature[i] += (climate.base_temperature - chunk.fields.temperature[i])
            * 0.01
            * surface_bias;
        chunk.fields.humidity[i] += (climate.humidity - chunk.fields.humidity[i]) * 0.005;
        if chunk.fields.void_fraction[i] > 0.35 && chunk.fields.organic[i] < 0.14 {
            chunk.fields.organic[i] =
                (chunk.fields.organic[i] + 0.0003 * climate.humidity).min(0.16);
        }
    }
}

pub fn surface_indices(chunk: &Chunk) -> Vec<usize> {
    let mut indices = Vec::new();
    for z in 0..CHUNK_SIZE {
        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let i = idx(x, y, z);
                if chunk.fields.solid_fraction[i] > 0.1 && chunk.fields.void_fraction[i] > 0.2 {
                    indices.push(i);
                }
            }
        }
    }
    indices
}
