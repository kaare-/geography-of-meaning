use crate::creatures::actions::Action;
use crate::creatures::creature::Experience;
use crate::creatures::sensors::SensorState;
use serde::Serialize;

use super::concepts::ConceptNode;
use super::edges::{EdgeType, MemoryEdge};
use super::nodes::{MemoryNode, NodeId, NodeKind};

const SIMILARITY_THRESHOLD: f32 = 0.85;
const CONCEPT_CLUSTER_THRESHOLD: f32 = 0.85;
const MIN_CONCEPT_CLUSTER_SIZE: usize = 3;

#[derive(Debug, Clone, Copy, Default)]
pub struct ActionPredictions {
    pub move_delta: f32,
    pub consume_delta: f32,
    pub rest_delta: f32,
    pub emit_sound_delta: f32,
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct MemoryNodeSummary {
    pub sensory: usize,
    pub action: usize,
    pub outcome: usize,
    pub sound: usize,
    pub concept: usize,
}

#[derive(Debug, Clone, Default)]
pub struct MemoryGraph {
    pub nodes: Vec<MemoryNode>,
    pub edges: Vec<MemoryEdge>,
    next_id: NodeId,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_experience(&mut self, exp: &Experience) {
        let sensory_before = self.find_or_create_sensory(exp.sensory_before);
        let sensory_after = self.find_or_create_sensory(exp.sensory_after);
        let action_id = self.create_node(NodeKind::Action(exp.action));
        let outcome_id = self.create_node(NodeKind::Outcome(exp.outcome));

        self.add_or_strengthen_edge(sensory_before, action_id, EdgeType::Precedes);
        self.add_or_strengthen_edge(action_id, outcome_id, EdgeType::ActionLeadsTo);
        self.add_or_strengthen_edge(sensory_before, sensory_after, EdgeType::Precedes);
        self.add_or_strengthen_edge(sensory_before, sensory_after, EdgeType::CoOccurs);
    }

    pub fn find_similar_sensory(&self, pattern: SensorState, threshold: f32) -> Option<NodeId> {
        for node in &self.nodes {
            if let NodeKind::SensoryPattern(stored) = node.kind {
                if cosine_similarity(&pattern.as_vector(), &stored.as_vector()) >= threshold {
                    return Some(node.id);
                }
            }
        }
        None
    }

    pub fn record_heard_sound(&mut self, sensory: SensorState, intensity: f32) {
        if intensity < 0.08 {
            return;
        }
        let sensory_id = self.find_or_create_sensory(sensory);
        let sound_id = self.create_node(NodeKind::Sound(intensity));
        self.add_or_strengthen_edge(sound_id, sensory_id, EdgeType::SoundActivates);
        self.add_or_strengthen_edge(sensory_id, sound_id, EdgeType::CoOccurs);
    }

    pub fn node_summary(&self) -> MemoryNodeSummary {
        let mut summary = MemoryNodeSummary::default();
        for node in &self.nodes {
            match node.kind {
                NodeKind::SensoryPattern(_) => summary.sensory += 1,
                NodeKind::Action(_) => summary.action += 1,
                NodeKind::Outcome(_) => summary.outcome += 1,
                NodeKind::Sound(_) => summary.sound += 1,
                NodeKind::Concept => summary.concept += 1,
            }
        }
        summary
    }

    pub fn consolidate_sleep(
        &mut self,
        recent: &[Experience],
        next_concept_id: &mut u64,
    ) -> Vec<ConceptNode> {
        for exp in recent.iter().rev().take(16) {
            let Some(sensory_id) = self.find_similar_sensory(exp.sensory_before, 0.75) else {
                continue;
            };
            for edge in &mut self.edges {
                if edge.source_id == sensory_id && edge.edge_type == EdgeType::Precedes {
                    if let Some(target) = self.nodes.iter().find(|n| n.id == edge.target_id) {
                        if let NodeKind::Action(action) = target.kind {
                            if action_matches(action, exp.action) {
                                if exp.outcome > 0.0 {
                                    edge.strength = (edge.strength + 0.05).min(1.0);
                                    edge.confidence = (edge.confidence + 0.03).min(1.0);
                                } else if exp.outcome < 0.0 {
                                    edge.strength = (edge.strength - 0.04).max(0.0);
                                }
                            }
                        }
                    }
                }
            }
        }
        for edge in &mut self.edges {
            if edge.observations < 2 {
                edge.strength = (edge.strength - 0.02).max(0.0);
            }
        }
        self.cluster_sensory_concepts(next_concept_id)
    }

    pub fn predict_regulatory_delta(&self, sensory: SensorState) -> f32 {
        let Some(sensory_id) = self.find_similar_sensory(sensory, 0.8) else {
            return 0.0;
        };

        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for edge in &self.edges {
            if edge.source_id != sensory_id || edge.edge_type != EdgeType::Precedes {
                continue;
            }
            let predicted = self.predict_outcome_for_action(edge.target_id);
            let w = edge.strength * edge.confidence;
            total += predicted * w;
            weight_sum += w;
        }

        if weight_sum > 1e-6 {
            total / weight_sum
        } else {
            0.0
        }
    }

    pub fn predict_action_outcomes(&self, sensory: SensorState) -> ActionPredictions {
        let mut predictions = ActionPredictions::default();
        let Some(sensory_id) = self.find_similar_sensory(sensory, 0.8) else {
            return predictions;
        };

        for edge in &self.edges {
            if edge.source_id != sensory_id || edge.edge_type != EdgeType::Precedes {
                continue;
            }
            let action_node = self.nodes.iter().find(|n| n.id == edge.target_id);
            let Some(NodeKind::Action(action)) = action_node.map(|n| n.kind) else {
                continue;
            };
            let predicted_delta = self.predict_outcome_for_action(edge.target_id);
            let weight = edge.strength * edge.confidence;
            match action {
                Action::Move(_) => predictions.move_delta += predicted_delta * weight,
                Action::ConsumeOrganic => predictions.consume_delta += predicted_delta * weight,
                Action::Rest => predictions.rest_delta += predicted_delta * weight,
                Action::EmitSound => predictions.emit_sound_delta += predicted_delta * weight,
            }
        }
        predictions
    }

    fn predict_outcome_for_action(&self, action_id: NodeId) -> f32 {
        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for edge in &self.edges {
            if edge.source_id == action_id && edge.edge_type == EdgeType::ActionLeadsTo {
                if let Some(node) = self.nodes.iter().find(|n| n.id == edge.target_id) {
                    if let NodeKind::Outcome(outcome) = node.kind {
                        let w = edge.strength * edge.confidence;
                        total += outcome * w;
                        weight_sum += w;
                    }
                }
            }
        }
        if weight_sum > 1e-6 {
            total / weight_sum
        } else {
            0.0
        }
    }

    fn find_or_create_sensory(&mut self, pattern: SensorState) -> NodeId {
        if let Some(id) = self.find_similar_sensory(pattern, SIMILARITY_THRESHOLD) {
            return id;
        }
        self.create_node(NodeKind::SensoryPattern(pattern))
    }

    fn create_node(&mut self, kind: NodeKind) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        self.nodes.push(MemoryNode { id, kind });
        id
    }

    fn cluster_sensory_concepts(&mut self, next_concept_id: &mut u64) -> Vec<ConceptNode> {
        use std::collections::HashSet;

        let sensory: Vec<(NodeId, SensorState)> = self
            .nodes
            .iter()
            .filter_map(|node| match node.kind {
                NodeKind::SensoryPattern(pattern) => Some((node.id, pattern)),
                _ => None,
            })
            .collect();

        let mut used = HashSet::new();
        let mut concepts = Vec::new();

        for i in 0..sensory.len() {
            if used.contains(&sensory[i].0) {
                continue;
            }
            let mut cluster = vec![sensory[i].0];
            let mut proto = sensory[i].1;

            for j in (i + 1)..sensory.len() {
                if used.contains(&sensory[j].0) {
                    continue;
                }
                if cosine_similarity(&proto.as_vector(), &sensory[j].1.as_vector())
                    >= CONCEPT_CLUSTER_THRESHOLD
                {
                    cluster.push(sensory[j].0);
                    proto = average_sensor(proto, sensory[j].1);
                }
            }

            if cluster.len() < MIN_CONCEPT_CLUSTER_SIZE {
                continue;
            }

            for member in &cluster {
                used.insert(*member);
            }

            let concept_id = *next_concept_id;
            *next_concept_id += 1;
            let concept_node_id = self.create_node(NodeKind::Concept);
            for member in &cluster {
                self.add_or_strengthen_edge(concept_node_id, *member, EdgeType::ConceptCompresses);
            }

            concepts.push(ConceptNode {
                id: concept_id,
                prototype: proto,
                member_node_ids: cluster,
                strength: 0.5,
            });
        }

        concepts
    }

    fn add_or_strengthen_edge(&mut self, source: NodeId, target: NodeId, edge_type: EdgeType) {
        if let Some(edge) = self
            .edges
            .iter_mut()
            .find(|e| e.source_id == source && e.target_id == target && e.edge_type == edge_type)
        {
            edge.observations += 1;
            edge.strength = (edge.strength + 0.1).min(1.0);
            edge.confidence = (edge.confidence + 0.05).min(1.0);
        } else {
            self.edges.push(MemoryEdge {
                source_id: source,
                target_id: target,
                edge_type,
                strength: 0.1,
                confidence: 0.1,
                observations: 1,
                delay_mean: 1.0,
                delay_variance: 0.5,
            });
        }
    }
}

fn action_matches(a: Action, b: Action) -> bool {
    match (a, b) {
        (Action::Move(_), Action::Move(_)) => true,
        (Action::ConsumeOrganic, Action::ConsumeOrganic) => true,
        (Action::Rest, Action::Rest) => true,
        (Action::EmitSound, Action::EmitSound) => true,
        _ => false,
    }
}

fn average_sensor(a: SensorState, b: SensorState) -> SensorState {
    let va = a.as_vector();
    let vb = b.as_vector();
    let mut out = [0.0f32; 15];
    for i in 0..15 {
        out[i] = (va[i] + vb[i]) * 0.5;
    }
    SensorState::from_vector(out)
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
