use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::math::Vec3i;
use crate::world::World;

use super::creature::Creature;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SensorState {
    pub light: f32,
    pub thermal: f32,
    pub chemical_organic: f32,
    pub chemical_wet_mineral: f32,
    pub chemical_decay: f32,
    pub chemical_binder: f32,
    pub chemical_creature: f32,
    pub sound_ambient: f32,
    pub sound_calls: f32,
    pub contact_hard: f32,
    pub contact_soft: f32,
    pub contact_occupied: f32,
    pub internal_energy: f32,
    pub internal_temperature_stress: f32,
    pub internal_hydration: f32,
}

impl Default for SensorState {
    fn default() -> Self {
        Self {
            light: 0.0,
            thermal: 0.0,
            chemical_organic: 0.0,
            chemical_wet_mineral: 0.0,
            chemical_decay: 0.0,
            chemical_binder: 0.0,
            chemical_creature: 0.0,
            sound_ambient: 0.0,
            sound_calls: 0.0,
            contact_hard: 0.0,
            contact_soft: 0.0,
            contact_occupied: 0.0,
            internal_energy: 0.0,
            internal_temperature_stress: 0.0,
            internal_hydration: 0.0,
        }
    }
}

impl SensorState {
    pub fn as_vector(&self) -> [f32; 15] {
        [
            self.light,
            self.thermal,
            self.chemical_organic,
            self.chemical_wet_mineral,
            self.chemical_decay,
            self.chemical_binder,
            self.chemical_creature,
            self.sound_ambient,
            self.sound_calls,
            self.contact_hard,
            self.contact_soft,
            self.contact_occupied,
            self.internal_energy,
            self.internal_temperature_stress,
            self.internal_hydration,
        ]
    }

    pub fn from_vector(v: [f32; 15]) -> Self {
        Self {
            light: v[0],
            thermal: v[1],
            chemical_organic: v[2],
            chemical_wet_mineral: v[3],
            chemical_decay: v[4],
            chemical_binder: v[5],
            chemical_creature: v[6],
            sound_ambient: v[7],
            sound_calls: v[8],
            contact_hard: v[9],
            contact_soft: v[10],
            contact_occupied: v[11],
            internal_energy: v[12],
            internal_temperature_stress: v[13],
            internal_hydration: v[14],
        }
    }
}

pub fn read_sensors<R: Rng + ?Sized>(creature: &Creature, world: &World, rng: &mut R) -> SensorState {
    read_sensors_with_noise(creature, world, &[], rng, 1.0)
}

pub fn read_sensors_with_noise<R: Rng + ?Sized>(
    creature: &Creature,
    world: &World,
    other_creatures: &[Creature],
    rng: &mut R,
    noise_multiplier: f32,
) -> SensorState {
    let center = creature.position.floor_i();
    let noise_scale = creature.genome.sensor_noise_scale * noise_multiplier;

    let mut light_sum = 0.0f32;
    let mut temp_sum = 0.0;
    let mut organic_sum = 0.0;
    let mut wet_mineral_sum = 0.0;
    let mut decay_sum = 0.0;
    let mut binder_sum = 0.0;
    let mut contact_hard: f32 = 0.0;
    let mut contact_soft: f32 = 0.0;
    let mut sound_ambient = 0.0f32;
    let mut sound_calls = 0.0f32;
    let mut count: f32 = 0.0;

    let center_voxel = world.sample_voxel(center);

    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                let pos = Vec3i::new(center.x + dx, center.y + dy, center.z + dz);
                if let Some(v) = world.sample_voxel(pos) {
                    light_sum += 1.0 - v.solid_fraction;
                    temp_sum += v.temperature;
                    organic_sum += v.organic;
                    wet_mineral_sum += v.wet_mineral();
                    decay_sum += v.decay_signal();
                    binder_sum += v.binder;
                    contact_hard = contact_hard.max(v.hard_mineral * v.solid_fraction);
                    contact_soft = contact_soft.max(v.soft_mineral * v.solid_fraction);
                    count += 1.0;
                }
            }
        }
    }

    let n = count.max(1.0);
    let center_temp = center_voxel.map(|v| v.temperature).unwrap_or(15.0);

    let thermal_gradient = if let Some(cv) = center_voxel {
        let neighbor_temp = temp_sum / n;
        neighbor_temp - cv.temperature
    } else {
        0.0
    };

    let env_temp_stress = ((center_temp - 20.0).abs() / 15.0).min(1.0);
    let diurnal = (world.day_phase * std::f32::consts::TAU).sin() * 0.5 + 0.5;

    for sound in &world.active_sounds {
        let attenuated = sound.attenuated_at(creature.position);
        sound_ambient += attenuated * 0.35;
        if sound.emitter_id != creature.id {
            sound_calls = sound_calls.max(attenuated);
        }
    }

    let mut contact_occupied = 0.0f32;
    let mut chemical_creature = 0.0f32;
    for other in other_creatures {
        if other.id == creature.id {
            continue;
        }
        let other_pos = other.position.floor_i();
        let dx = (other_pos.x - center.x).abs();
        let dy = (other_pos.y - center.y).abs();
        let dz = (other_pos.z - center.z).abs();
        if dx <= 1 && dy <= 1 && dz <= 1 {
            contact_occupied = contact_occupied.max(1.0);
            chemical_creature = chemical_creature.max(0.15);
        }
    }

    let state = SensorState {
        light: ((light_sum / n) * diurnal) + gaussian_noise(rng) * noise_scale,
        thermal: thermal_gradient + gaussian_noise(rng) * noise_scale,
        chemical_organic: (organic_sum / n) + gaussian_noise(rng) * noise_scale,
        chemical_wet_mineral: (wet_mineral_sum / n) + gaussian_noise(rng) * noise_scale,
        chemical_decay: (decay_sum / n) + gaussian_noise(rng) * noise_scale,
        chemical_binder: (binder_sum / n) + gaussian_noise(rng) * noise_scale,
        chemical_creature: chemical_creature + gaussian_noise(rng) * noise_scale,
        sound_ambient: sound_ambient + gaussian_noise(rng).abs() * 0.05 * noise_scale,
        sound_calls: sound_calls + gaussian_noise(rng).abs() * 0.03 * noise_scale,
        contact_hard: contact_hard + gaussian_noise(rng) * noise_scale,
        contact_soft: contact_soft + gaussian_noise(rng) * noise_scale,
        contact_occupied: contact_occupied + gaussian_noise(rng) * noise_scale,
        internal_energy: creature.regulatory.energy,
        internal_temperature_stress: env_temp_stress,
        internal_hydration: creature.regulatory.hydration,
    };

    state
}

/// Strongest non-self sound at the listener position, if above noise floor.
pub fn dominant_heard_signature(creature: &Creature, world: &World) -> Option<u64> {
    world
        .active_sounds
        .iter()
        .filter(|s| s.emitter_id != creature.id)
        .map(|s| (s.attenuated_at(creature.position), s.signature))
        .max_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal))
        .filter(|(att, _)| *att > 0.05)
        .map(|(_, sig)| sig)
}

fn gaussian_noise<R: Rng + ?Sized>(rng: &mut R) -> f32 {
    let u1: f32 = rng.gen_range(0.0001..1.0);
    let u2: f32 = rng.gen_range(0.0..1.0);
    (-2.0 * u1.ln()).sqrt() * (2.0 * std::f32::consts::PI * u2).cos()
}
