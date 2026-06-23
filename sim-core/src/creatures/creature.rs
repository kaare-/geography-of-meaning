use serde::{Deserialize, Serialize};

use rand::Rng;

use crate::math::Vec3f;
use crate::memory::{activate_concepts, ActiveConcept, ConceptNode, ConceptNodeId, MemoryGraph};
use crate::world::sound::SoundEmitterContext;

use super::actions::Action;
use super::genome::Genome;
use super::morphology::Morphology;
use super::regulation::RegulatoryState;
use super::sensors::SensorState;

pub const MAX_RECENT_EXPERIENCE: usize = 64;

pub const SLEEP_FATIGUE_THRESHOLD: f32 = 0.65;
pub const SLEEP_LIGHT_THRESHOLD: f32 = 0.45;
pub const SLEEP_WAKE_FATIGUE: f32 = 0.35;
pub const SLEEP_DURATION_TICKS: u32 = 15;

#[derive(Debug, Default, Clone, Copy)]
pub struct SleepUpdateResult {
    pub concepts_formed: u32,
    pub imagination_events: u32,
    pub merge_count: u32,
    pub split_count: u32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SleepState {
    pub sleeping: bool,
    pub ticks_remaining: u32,
}

impl Default for SleepState {
    fn default() -> Self {
        Self {
            sleeping: false,
            ticks_remaining: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub sensory_before: SensorState,
    pub state_before: RegulatoryState,
    pub action: Action,
    pub sensory_after: SensorState,
    pub state_after: RegulatoryState,
    pub outcome: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct Creature {
    pub id: u64,
    pub position: Vec3f,
    pub genome: Genome,
    pub morphology: Morphology,
    pub regulatory: RegulatoryState,
    pub sensor: SensorState,
    pub recent_experience: Vec<Experience>,
    pub memory_graph: MemoryGraph,
    pub concepts: Vec<ConceptNode>,
    pub concept_nodes: Vec<ConceptNodeId>,
    pub active_concepts: Vec<ActiveConcept>,
    pub next_concept_id: ConceptNodeId,
    pub signature: u64,
    pub age: u32,
    pub sleep: SleepState,
}

impl Creature {
    pub fn new(id: u64, position: Vec3f, signature: u64) -> Self {
        let genome = Genome::default();
        let morphology = Morphology::from_genome(&genome);
        Self {
            id,
            position,
            genome,
            morphology,
            regulatory: RegulatoryState::default(),
            sensor: SensorState::default(),
            recent_experience: Vec::new(),
            memory_graph: MemoryGraph::new(),
            concepts: Vec::new(),
            concept_nodes: Vec::new(),
            active_concepts: Vec::new(),
            next_concept_id: 1,
            signature,
            age: 0,
            sleep: SleepState::default(),
        }
    }

    pub fn update_sleep<R: Rng + ?Sized>(
        &mut self,
        dream_noise: bool,
        rng: &mut R,
    ) -> SleepUpdateResult {
        let mut result = SleepUpdateResult::default();
        if !self.sleep.sleeping {
            return result;
        }
        let at_sleep_start = self.sleep.ticks_remaining == SLEEP_DURATION_TICKS;
        if self.sleep.ticks_remaining > 0 {
            self.sleep.ticks_remaining -= 1;
        }
        let at_wake = self.sleep.ticks_remaining == 0;
        if !at_wake {
            if at_sleep_start {
                // Sleep onset: no graph consolidation (deferred to wake).
            } else {
                result.imagination_events = self.memory_graph.imagination_replay(
                    &self.concepts,
                    dream_noise,
                    rng,
                );
            }
            if self.sleep.ticks_remaining == 0 {
                self.sleep.sleeping = false;
            }
            return result;
        }
        let consolidation = self.memory_graph.consolidate_sleep(
            &self.recent_experience,
            &mut self.next_concept_id,
            &self.concepts,
        );
        result.concepts_formed = consolidation.concepts.len() as u32;
        result.merge_count = consolidation.merge_count;
        result.split_count = consolidation.split_count;
        for concept in consolidation.concepts {
            if let Some(existing) = self.concepts.iter_mut().find(|c| c.id == concept.id) {
                existing.prototype = concept.prototype;
                existing.strength = concept.strength.max(existing.strength);
                for mid in concept.member_node_ids {
                    if !existing.member_node_ids.contains(&mid) {
                        existing.member_node_ids.push(mid);
                    }
                }
            } else if !self.concept_nodes.contains(&concept.id) {
                self.concept_nodes.push(concept.id);
                self.concepts.push(concept);
            }
        }
        if at_wake {
            self.sleep.sleeping = false;
        }
        result
    }

    pub fn try_early_wake(&mut self) {
        if self.sleep.sleeping && self.regulatory.fatigue < SLEEP_WAKE_FATIGUE {
            self.sleep.ticks_remaining = 0;
        }
    }

    pub fn try_enter_sleep(&mut self) {
        if !self.sleep.sleeping
            && self.regulatory.fatigue > SLEEP_FATIGUE_THRESHOLD
            && self.sensor.light < SLEEP_LIGHT_THRESHOLD
        {
            self.sleep.sleeping = true;
            self.sleep.ticks_remaining = SLEEP_DURATION_TICKS;
        }
    }

    pub fn refresh_active_concepts(&mut self) {
        self.active_concepts = activate_concepts(self.sensor, &self.concepts);
    }

    pub fn push_experience(&mut self, exp: Experience) {
        self.recent_experience.push(exp);
        if self.recent_experience.len() > MAX_RECENT_EXPERIENCE {
            self.recent_experience.remove(0);
        }
    }

    pub fn sound_emitter_context(&self) -> SoundEmitterContext {
        SoundEmitterContext {
            emitter_id: self.id,
            signature: self.signature,
            age: self.age,
            mass: self.morphology.mass,
            move_speed: self.genome.move_speed,
            carried_mass: self.regulatory.carried_mass,
        }
    }
}
