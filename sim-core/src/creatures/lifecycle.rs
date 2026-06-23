use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::math::{Vec3f, Vec3i};
use crate::world::World;

use super::creature::Creature;
use super::genome::Genome;
use super::morphology::Morphology;

/// Organic matter returned to the voxel field when a creature dies.
pub const DEATH_ORGANIC_DEPOSIT: f32 = 0.08;

pub const REPRODUCTION_ENERGY_THRESHOLD: f32 = 0.6;
pub const REPRODUCTION_CHANCE_PER_TICK: f32 = 0.02;
pub const REPRODUCTION_CHANCE_HIGH_ENERGY: f32 = 0.025;
pub const REPRODUCTION_HIGH_ENERGY_THRESHOLD: f32 = 0.6;
pub const ENERGY_DEPLETION_GRACE_TICKS: u8 = 3;
pub const REPRODUCTION_ENERGY_COST: f32 = 0.25;
pub const DEFAULT_MAX_POPULATION: usize = 30;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeathCause {
    EnergyDepletion,
    IntegrityFailure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeathEvent {
    pub creature_id: u64,
    pub position: Vec3f,
    pub age: u32,
    pub cause: DeathCause,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BirthEvent {
    pub parent_id: u64,
    pub offspring_id: u64,
    pub position: Vec3f,
    pub parent_signature: u64,
    pub offspring_signature: u64,
}

impl Creature {
    pub fn is_alive(&self) -> bool {
        self.regulatory.integrity > 0.0
            && (self.regulatory.energy > 0.0
                || self.regulatory.energy_depleted_ticks < ENERGY_DEPLETION_GRACE_TICKS)
    }

    pub fn death_cause(&self) -> Option<DeathCause> {
        if self.regulatory.integrity <= 0.0 {
            Some(DeathCause::IntegrityFailure)
        } else if self.regulatory.energy <= 0.0
            && self.regulatory.energy_depleted_ticks >= ENERGY_DEPLETION_GRACE_TICKS
        {
            Some(DeathCause::EnergyDepletion)
        } else {
            None
        }
    }
}

pub fn deposit_creature_organic(world: &mut World, creature: &Creature) {
    let pos = creature.position.floor_i();
    if let Some(voxel) = world.sample_voxel_mut(pos) {
        *voxel.organic = (*voxel.organic + DEATH_ORGANIC_DEPOSIT).min(1.0);
    }
}

pub fn find_offspring_position<R: Rng + ?Sized>(
    parent: &Creature,
    world: &World,
    rng: &mut R,
) -> Option<Vec3f> {
    let base = parent.position.floor_i();
    let mut candidates = Vec::new();
    for dx in -2..=2 {
        for dy in -2..=2 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                candidates.push(Vec3i::new(base.x + dx, base.y + dy, base.z + dz));
            }
        }
    }
    for i in (1..candidates.len()).rev() {
        let j = rng.gen_range(0..=i);
        candidates.swap(i, j);
    }
    for pos in candidates {
        if let Some(voxel) = world.sample_voxel(pos) {
            if voxel.void_fraction > 0.4 {
                return Some(Vec3f::from_vec3i(pos));
            }
        }
    }
    None
}

fn mutate_signature<R: Rng + ?Sized>(parent: u64, rng: &mut R) -> u64 {
    parent
        .wrapping_mul(0x9e37_79b9_7f4a_7c15)
        .wrapping_add(rng.gen())
}

pub fn try_reproduce<R: Rng + ?Sized>(
    parent: &Creature,
    world: &World,
    rng: &mut R,
    offspring_id: u64,
) -> Option<(Creature, BirthEvent)> {
    if parent.regulatory.energy <= REPRODUCTION_ENERGY_THRESHOLD {
        return None;
    }
    if parent.regulatory.integrity <= 0.5
        || parent.regulatory.hydration <= 0.3
        || parent.regulatory.fatigue >= 0.7
    {
        return None;
    }
    let reproduction_chance = if parent.regulatory.energy > REPRODUCTION_HIGH_ENERGY_THRESHOLD {
        REPRODUCTION_CHANCE_HIGH_ENERGY
    } else {
        REPRODUCTION_CHANCE_PER_TICK
    };
    if rng.gen::<f32>() >= reproduction_chance {
        return None;
    }
    let position = find_offspring_position(parent, world, rng)?;
    let genome = Genome::mutate_from(&parent.genome, rng);
    let morphology = Morphology::mutate_from(&parent.morphology, &genome, rng);
    let offspring_signature = mutate_signature(parent.signature, rng);
    let mut offspring = Creature::new(offspring_id, position, offspring_signature);
    offspring.genome = genome;
    offspring.morphology = morphology;
    offspring.regulatory.energy = 0.5;
    offspring.regulatory.hydration = 0.6;
    let birth = BirthEvent {
        parent_id: parent.id,
        offspring_id,
        position,
        parent_signature: parent.signature,
        offspring_signature,
    };
    Some((offspring, birth))
}
