use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::creatures::sensors::SensorState;
use crate::math::{Vec3f, Vec3i};
use crate::memory::ConceptNode;
use crate::world::World;

use super::creature::Creature;
use super::genome::Genome;
use super::morphology::Morphology;

/// Organic matter returned to the voxel field when a creature dies.
pub const DEATH_ORGANIC_DEPOSIT: f32 = 0.12;

pub const REPRODUCTION_ENERGY_THRESHOLD: f32 = 0.6;
pub const REPRODUCTION_CHANCE_PER_TICK: f32 = 0.02;
pub const REPRODUCTION_CHANCE_HIGH_ENERGY: f32 = 0.025;
pub const REPRODUCTION_HIGH_ENERGY_THRESHOLD: f32 = 0.6;
pub const ENERGY_DEPLETION_GRACE_TICKS: u8 = 3;
pub const REPRODUCTION_ENERGY_COST: f32 = 0.25;
pub const DEFAULT_MAX_POPULATION: usize = 30;

const MAX_INHERITED_CONCEPTS: usize = 3;
const INHERITANCE_STRENGTH_FACTOR: f32 = 0.8;
const PROTOTYPE_NOISE_SCALE: f32 = 0.03;

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

fn noise_prototype<R: Rng + ?Sized>(proto: SensorState, rng: &mut R) -> SensorState {
    let mut v = proto.as_vector();
    for channel in &mut v {
        *channel = (*channel + rng.gen_range(-PROTOTYPE_NOISE_SCALE..PROTOTYPE_NOISE_SCALE))
            .clamp(0.0, 1.0);
    }
    SensorState::from_vector(v)
}

/// Weak concept biases for offspring — not a full memory copy (see doc 13).
pub fn inherit_parent_concepts<R: Rng + ?Sized>(
    offspring: &mut Creature,
    parent: &Creature,
    rng: &mut R,
) {
    if parent.concepts.is_empty() {
        return;
    }
    let mut indices: Vec<usize> = (0..parent.concepts.len()).collect();
    for i in (1..indices.len()).rev() {
        let j = rng.gen_range(0..=i);
        indices.swap(i, j);
    }
    for idx in indices.into_iter().take(MAX_INHERITED_CONCEPTS) {
        let parent_concept = &parent.concepts[idx];
        let concept_id = offspring.next_concept_id;
        offspring.next_concept_id += 1;
        let inherited = ConceptNode {
            id: concept_id,
            prototype: noise_prototype(parent_concept.prototype, rng),
            member_node_ids: Vec::new(),
            strength: (parent_concept.strength * INHERITANCE_STRENGTH_FACTOR).clamp(0.05, 1.0),
        };
        offspring
            .memory_graph
            .seed_inherited_concept(&parent.memory_graph, parent_concept, &inherited);
        offspring.concept_nodes.push(concept_id);
        offspring.concepts.push(inherited);
    }
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
    inherit_parent_concepts(&mut offspring, parent, rng);
    let birth = BirthEvent {
        parent_id: parent.id,
        offspring_id,
        position,
        parent_signature: parent.signature,
        offspring_signature,
    };
    Some((offspring, birth))
}
