use serde::{Deserialize, Serialize};

use crate::creatures::sensors::SensorState;
use crate::memory::nodes::NodeId;

pub type ConceptNodeId = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: ConceptNodeId,
    pub prototype: SensorState,
    pub member_node_ids: Vec<NodeId>,
    pub strength: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveConcept {
    pub concept_id: ConceptNodeId,
    pub activation: f32,
}

const ACTIVATION_THRESHOLD: f32 = 0.7;

pub fn activate_concepts(sensor: SensorState, concepts: &[ConceptNode]) -> Vec<ActiveConcept> {
    concepts
        .iter()
        .filter_map(|concept| {
            let sim = cosine_similarity(&sensor.as_vector(), &concept.prototype.as_vector());
            if sim >= ACTIVATION_THRESHOLD {
                Some(ActiveConcept {
                    concept_id: concept.id,
                    activation: (sim * concept.strength).min(1.0),
                })
            } else {
                None
            }
        })
        .collect()
}

fn cosine_similarity(a: &[f32; 15], b: &[f32; 15]) -> f32 {
    let mut dot = 0.0f32;
    let mut norm_a = 0.0f32;
    let mut norm_b = 0.0f32;
    for i in 0..15 {
        dot += a[i] * b[i];
        norm_a += a[i] * a[i];
        norm_b += b[i] * b[i];
    }
    if norm_a < 1e-6 || norm_b < 1e-6 {
        return 0.0;
    }
    dot / (norm_a.sqrt() * norm_b.sqrt())
}
