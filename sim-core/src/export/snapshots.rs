use serde::Serialize;

use crate::creatures::Creature;
use crate::math::Vec3f;
use crate::memory::{ActiveConcept, MemoryNodeSummary};
use crate::simulation::Simulation;
use crate::world::chunk::Chunk;
use crate::world::voxel::{idx, CHUNK_SIZE};
use crate::world::ChunkCoord;

#[derive(Debug, Serialize)]
pub struct WorldSnapshot {
    pub time: u64,
    pub day_phase: f32,
    pub season: f32,
    pub chunk_size: usize,
    pub chunks: Vec<ChunkSnapshot>,
    pub creatures: Vec<CreatureSnapshot>,
}

#[derive(Debug, Serialize)]
pub struct ChunkSnapshot {
    pub coord: ChunkCoord,
    pub slice_z: usize,
    pub organic: Vec<Vec<f32>>,
    pub surface_water: Vec<Vec<f32>>,
    pub temperature: Vec<Vec<f32>>,
    pub solid_fraction: Vec<Vec<f32>>,
}

#[derive(Debug, Serialize)]
pub struct CreatureSnapshot {
    pub id: u64,
    pub position: Vec3f,
    pub energy: f32,
    pub hydration: f32,
    pub temperature_stress: f32,
    pub integrity: f32,
    pub fatigue: f32,
    pub carried_mass: f32,
    pub age: u32,
    pub sleeping: bool,
    pub sleep_ticks_remaining: u32,
    pub signature: u64,
    pub vocal_profile: crate::creatures::genome::VocalProfile,
    pub sensor: crate::creatures::SensorState,
    pub memory_node_count: usize,
    pub memory_edges: usize,
    pub memory_nodes_by_type: MemoryNodeSummary,
    pub concept_count: usize,
    pub active_concepts: Vec<ActiveConcept>,
}

impl CreatureSnapshot {
    pub fn from_creature(creature: &Creature) -> Self {
        Self {
            id: creature.id,
            position: creature.position,
            energy: creature.regulatory.energy,
            hydration: creature.regulatory.hydration,
            temperature_stress: creature.regulatory.temperature_stress,
            integrity: creature.regulatory.integrity,
            fatigue: creature.regulatory.fatigue,
            carried_mass: creature.regulatory.carried_mass,
            age: creature.age,
            sleeping: creature.sleep.sleeping,
            sleep_ticks_remaining: creature.sleep.ticks_remaining,
            signature: creature.signature,
            vocal_profile: creature.genome.vocal_profile,
            sensor: creature.sensor,
            memory_node_count: creature.memory_graph.nodes.len(),
            memory_edges: creature.memory_graph.edges.len(),
            memory_nodes_by_type: creature.memory_graph.node_summary(),
            concept_count: creature.concepts.len(),
            active_concepts: creature.active_concepts.clone(),
        }
    }
}

impl WorldSnapshot {
    pub fn from_simulation(sim: &Simulation) -> Self {
        let slice_z = CHUNK_SIZE / 2;
        let chunks = sim
            .world
            .chunks
            .values()
            .map(|chunk| ChunkSnapshot::from_chunk(chunk, slice_z))
            .collect();
        let creatures = sim
            .creatures
            .iter()
            .map(CreatureSnapshot::from_creature)
            .collect();

        Self {
            time: sim.world.time,
            day_phase: sim.world.day_phase,
            season: sim.world.season,
            chunk_size: CHUNK_SIZE,
            chunks,
            creatures,
        }
    }
}

impl ChunkSnapshot {
    pub fn from_chunk(chunk: &Chunk, slice_z: usize) -> Self {
        let mut organic = vec![vec![0.0; CHUNK_SIZE]; CHUNK_SIZE];
        let mut surface_water = vec![vec![0.0; CHUNK_SIZE]; CHUNK_SIZE];
        let mut temperature = vec![vec![0.0; CHUNK_SIZE]; CHUNK_SIZE];
        let mut solid_fraction = vec![vec![0.0; CHUNK_SIZE]; CHUNK_SIZE];

        for y in 0..CHUNK_SIZE {
            for x in 0..CHUNK_SIZE {
                let i = idx(x, y, slice_z);
                organic[x][y] = chunk.fields.organic[i];
                surface_water[x][y] = chunk.fields.surface_water[i];
                temperature[x][y] = chunk.fields.temperature[i];
                solid_fraction[x][y] = chunk.fields.solid_fraction[i];
            }
        }

        Self {
            coord: chunk.coord,
            slice_z,
            organic,
            surface_water,
            temperature,
            solid_fraction,
        }
    }
}
