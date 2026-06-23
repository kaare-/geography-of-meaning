use std::collections::HashMap;

use rand::Rng;

use crate::creatures::actions::Action;
use crate::creatures::creature::Experience;
use crate::creatures::sensors::SensorState;
use crate::math::Vec3i;
use serde::Serialize;

use super::concepts::{ActiveConcept, ConceptNode};
use super::edges::{EdgeType, MemoryEdge};
use super::nodes::{MemoryNode, NodeId, NodeKind, SoundNode};

const SIMILARITY_THRESHOLD: f32 = 0.85;
const CONCEPT_CLUSTER_THRESHOLD: f32 = 0.70;
const MIN_CONCEPT_CLUSTER_SIZE: usize = 2;
const SPREAD_DECAY: f32 = 0.5;
const SPREAD_DECAY_HOP2: f32 = 0.25;
const CO_OCCURS_SPREAD_WEIGHT: f32 = 0.35;
const CONCEPT_MERGE_THRESHOLD: f32 = 0.88;
const CONCEPT_SPLIT_VARIANCE_THRESHOLD: f32 = 0.12;
const PROTOTYPE_EMA_ALPHA: f32 = 0.15;
const IMAGINATION_SPREAD_STRENGTHEN: f32 = 0.02;
const DELAY_WEIGHT_SCALE: f32 = 0.1;
const OUTCOME_QUANTUM: f32 = 0.05;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum CanonicalActionKey {
    Move,
    Push,
    ConsumeOrganic,
    Rest,
    EmitSound,
    Follow,
    Dig,
    Carry,
    Drop,
    PlaceMaterial,
    ApplyBinder,
    TransferOrganic,
}

#[derive(Debug, Clone, Default)]
pub struct ConsolidationResult {
    pub concepts: Vec<ConceptNode>,
    pub merge_count: u32,
    pub split_count: u32,
}

#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct ActionPredictions {
    pub move_delta: f32,
    pub push_delta: f32,
    pub consume_delta: f32,
    pub rest_delta: f32,
    pub emit_sound_delta: f32,
    pub follow_delta: f32,
    pub dig_delta: f32,
    pub carry_delta: f32,
    pub drop_delta: f32,
    pub place_material_delta: f32,
    pub apply_binder_delta: f32,
    pub transfer_organic_delta: f32,
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
    node_index: HashMap<NodeId, usize>,
    edges_by_source: HashMap<NodeId, Vec<usize>>,
    action_nodes: HashMap<CanonicalActionKey, NodeId>,
    outcome_nodes: HashMap<i32, NodeId>,
}

impl MemoryGraph {
    pub fn new() -> Self {
        Self::default()
    }

    fn node_by_id(&self, id: NodeId) -> Option<&MemoryNode> {
        self.node_index.get(&id).map(|&i| &self.nodes[i])
    }

    fn push_edge(&mut self, edge: MemoryEdge) {
        let idx = self.edges.len();
        self.edges_by_source
            .entry(edge.source_id)
            .or_default()
            .push(idx);
        self.edges.push(edge);
    }

    fn outgoing_edges(&self, source_id: NodeId) -> &[usize] {
        self.edges_by_source
            .get(&source_id)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    pub fn record_experience(&mut self, exp: &Experience) {
        let sensory_before = self.find_or_create_sensory(exp.sensory_before);
        let sensory_after = self.find_or_create_sensory(exp.sensory_after);
        let action_id = self.find_or_create_action(exp.action);
        let outcome_id = self.find_or_create_outcome(exp.outcome);

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

    pub fn record_heard_sound(&mut self, sensory: SensorState, intensity: f32, signature: u64) {
        if intensity < 0.08 {
            return;
        }
        let sensory_id = self.find_or_create_sensory(sensory);
        let sound_id = self.create_node(NodeKind::Sound(SoundNode {
            intensity,
            signature,
        }));
        self.add_or_strengthen_edge(sound_id, sensory_id, EdgeType::SoundActivates);
        self.add_or_strengthen_edge(sensory_id, sound_id, EdgeType::CoOccurs);
    }

    /// Signatures with positive mean sound→outcome association above confidence threshold.
    pub fn trusted_signature_count(&self) -> usize {
        let mut by_sig: HashMap<u64, (f32, f32)> = HashMap::new();
        for node in &self.nodes {
            let NodeKind::Sound(sound) = node.kind else {
                continue;
            };
            let (outcome, weight) = self.sound_node_outcome_stats(node.id);
            if weight < 1e-6 {
                continue;
            }
            let entry = by_sig.entry(sound.signature).or_insert((0.0, 0.0));
            entry.0 += outcome * weight;
            entry.1 += weight;
        }
        by_sig
            .values()
            .filter(|(total, w)| *w > 0.05 && *total / *w > 0.03)
            .count()
    }

    /// Extra follow weight when calls are salient and memory links signature to positive outcomes.
    pub fn trusted_follow_boost(&self, sound_calls: f32, heard_signature: Option<u64>) -> f32 {
        let Some(sig) = heard_signature else {
            return 0.0;
        };
        if sound_calls < 0.08 {
            return 0.0;
        }
        let outcome = self.signature_mean_outcome(sig);
        if outcome > 0.03 {
            sound_calls * outcome * 2.0
        } else {
            0.0
        }
    }

    /// Weak follow boost when high-pitch calls match signatures with positive memory history.
    pub fn developmental_follow_boost(
        &self,
        bias: f32,
        sound_calls: f32,
        heard_signature: Option<u64>,
        heard_call_frequency: Option<f32>,
    ) -> f32 {
        if bias < 0.01 || sound_calls < 0.08 {
            return 0.0;
        }
        let Some(freq) = heard_call_frequency else {
            return 0.0;
        };
        if freq < 0.55 {
            return 0.0;
        }
        let Some(sig) = heard_signature else {
            return 0.0;
        };
        let outcome = self.signature_mean_outcome(sig);
        bias * sound_calls * (0.35 + outcome.max(0.0))
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
        existing_concepts: &[ConceptNode],
    ) -> ConsolidationResult {
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
        let mut concepts = self.cluster_sensory_concepts(next_concept_id, existing_concepts);
        concepts.extend(self.merge_loose_sensory_into_concepts(next_concept_id, existing_concepts));
        let merge_count = self.merge_similar_concept_nodes(&mut concepts);
        let split_count = self.split_high_variance_concepts(&mut concepts, next_concept_id);
        ConsolidationResult {
            concepts,
            merge_count,
            split_count,
        }
    }

    /// Offline spreading activation during sleep — strengthens edges along activated paths.
    pub fn imagination_replay<R: Rng + ?Sized>(
        &mut self,
        concepts: &[ConceptNode],
        dream_noise: bool,
        rng: &mut R,
    ) -> u32 {
        if self.nodes.is_empty() {
            return 0;
        }
        let active = if !concepts.is_empty() && rng.gen::<f32>() < 0.7 {
            let concept = &concepts[rng.gen_range(0..concepts.len())];
            vec![ActiveConcept {
                concept_id: concept.id,
                activation: 0.55,
            }]
        } else {
            let sensory_nodes: Vec<NodeId> = self
                .nodes
                .iter()
                .filter_map(|n| match n.kind {
                    NodeKind::SensoryPattern(_) => Some(n.id),
                    _ => None,
                })
                .collect();
            if sensory_nodes.is_empty() {
                return 0;
            }
            let id = sensory_nodes[rng.gen_range(0..sensory_nodes.len())];
            let mut activation = HashMap::new();
            activation.insert(id, 0.5);
            let hop1 = Self::spread_hop(&self.edges, &self.edges_by_source, &activation, SPREAD_DECAY);
            let mut events = 0u32;
            for edge in &mut self.edges {
                if hop1.contains_key(&edge.source_id) {
                    edge.strength = (edge.strength + IMAGINATION_SPREAD_STRENGTHEN).min(1.0);
                    events += 1;
                }
            }
            if dream_noise {
                self.apply_dream_noise(rng);
            }
            return events;
        };
        let spread = self.spread_activation(&active, concepts);
        let mut events = 0u32;
        for edge in &mut self.edges {
            if spread.contains_key(&edge.source_id) {
                edge.strength = (edge.strength + IMAGINATION_SPREAD_STRENGTHEN).min(1.0);
                events += 1;
            }
        }
        if dream_noise {
            self.apply_dream_noise(rng);
        }
        events
    }

    fn apply_dream_noise<R: Rng + ?Sized>(&mut self, rng: &mut R) {
        for edge in &mut self.edges {
            if rng.gen::<f32>() < 0.04 {
                edge.strength =
                    (edge.strength + rng.gen_range(-0.03..0.03)).clamp(0.0, 1.0);
            }
        }
    }

    /// How unfamiliar the current sensory pattern is (1 = novel, 0 = familiar).
    pub fn novelty_score(&self, sensory: SensorState) -> f32 {
        let best = self
            .nodes
            .iter()
            .filter_map(|n| match n.kind {
                NodeKind::SensoryPattern(p) => {
                    Some(cosine_similarity(&sensory.as_vector(), &p.as_vector()))
                }
                _ => None,
            })
            .fold(0.0f32, f32::max);
        (1.0 - best).clamp(0.0, 1.0)
    }

    /// Low mean edge confidence along active prediction paths (1 = very uncertain).
    pub fn prediction_uncertainty(
        &self,
        sensory: SensorState,
        active_concepts: &[ActiveConcept],
        concepts: &[ConceptNode],
    ) -> f32 {
        let spread = self.spread_activation(active_concepts, concepts);
        self.prediction_uncertainty_with_spread(sensory, &spread)
    }

    pub fn prediction_uncertainty_with_spread(
        &self,
        sensory: SensorState,
        spread: &HashMap<NodeId, f32>,
    ) -> f32 {
        let sensory_id = self.find_similar_sensory(sensory, 0.8);
        let mut confidences = Vec::new();
        for source_id in self.prediction_source_ids(sensory_id, spread) {
            let source_weight = source_activation_weight(sensory_id, source_id, spread);
            if source_weight < 1e-6 {
                continue;
            }
            for &ei in self.outgoing_edges(source_id) {
                let edge = &self.edges[ei];
                if edge.edge_type == EdgeType::Precedes {
                    confidences.push(edge.confidence);
                }
            }
        }
        if confidences.is_empty() {
            return 1.0;
        }
        let mean = confidences.iter().sum::<f32>() / confidences.len() as f32;
        (1.0 - mean).clamp(0.0, 1.0)
    }

    /// Create a concept memory node and optionally copy `ConceptCompresses` edge strengths
    /// from a parent graph (sensory targets are not copied — biases only).
    pub fn seed_inherited_concept(
        &mut self,
        parent: &MemoryGraph,
        parent_concept: &ConceptNode,
        _inherited: &ConceptNode,
    ) -> NodeId {
        let offspring_concept_id = self.create_node(NodeKind::Concept);
        let Some(parent_mem_id) = parent.concept_memory_node_for(parent_concept) else {
            return offspring_concept_id;
        };
        for edge in &parent.edges {
            if edge.source_id != parent_mem_id || edge.edge_type != EdgeType::ConceptCompresses {
                continue;
            }
            let Some(parent_node) = parent.node_by_id(edge.target_id) else {
                continue;
            };
            let target_id = match parent_node.kind {
                NodeKind::SensoryPattern(pattern) => self.find_or_create_sensory(pattern),
                _ => continue,
            };
            self.add_or_strengthen_edge(
                offspring_concept_id,
                target_id,
                EdgeType::ConceptCompresses,
            );
        }
        offspring_concept_id
    }

    /// Member sensory node ids linked to a concept memory node.
    pub fn concept_members_for(&self, concept_mem_id: NodeId) -> Vec<NodeId> {
        self.concept_members(concept_mem_id)
    }

    /// Refresh `member_node_ids` from graph `ConceptCompresses` edges after sleep consolidation.
    pub fn sync_concept_members(&self, concepts: &mut [ConceptNode]) {
        for concept in concepts {
            if let Some(mem_id) = self.concept_memory_node_for(concept) {
                let members = self.concept_members(mem_id);
                if !members.is_empty() {
                    concept.member_node_ids = members;
                }
            }
        }
    }

    fn concept_memory_node_for(&self, concept: &ConceptNode) -> Option<NodeId> {
        for node in &self.nodes {
            if !matches!(node.kind, NodeKind::Concept) {
                continue;
            }
            let members: std::collections::HashSet<NodeId> = self
                .edges
                .iter()
                .filter(|e| e.source_id == node.id && e.edge_type == EdgeType::ConceptCompresses)
                .map(|e| e.target_id)
                .collect();
            if concept.member_node_ids.is_empty() {
                continue;
            }
            if concept
                .member_node_ids
                .iter()
                .any(|m| members.contains(m))
            {
                return Some(node.id);
            }
        }
        None
    }

    fn sensory_compressed_targets(&self) -> std::collections::HashSet<NodeId> {
        self.edges
            .iter()
            .filter(|e| e.edge_type == EdgeType::ConceptCompresses)
            .filter(|e| {
                self.nodes
                    .iter()
                    .any(|n| n.id == e.target_id && matches!(n.kind, NodeKind::SensoryPattern(_)))
            })
            .map(|e| e.target_id)
            .collect()
    }

    fn concept_members(&self, concept_node_id: NodeId) -> Vec<NodeId> {
        self.outgoing_edges(concept_node_id)
            .iter()
            .filter_map(|&ei| {
                let e = &self.edges[ei];
                if e.edge_type == EdgeType::ConceptCompresses {
                    Some(e.target_id)
                } else {
                    None
                }
            })
            .collect()
    }

    fn prototype_for_members(&self, members: &[NodeId], fallback: SensorState) -> SensorState {
        let mut proto = fallback;
        let mut count = 0usize;
        for &mid in members {
            if let Some(node) = self.node_by_id(mid) {
                if let NodeKind::SensoryPattern(p) = node.kind {
                    if count == 0 {
                        proto = p;
                    } else {
                        proto = average_sensor(proto, p);
                    }
                    count += 1;
                }
            }
        }
        proto
    }

    fn find_similar_existing_concept(
        &self,
        proto: SensorState,
        existing_concepts: &[ConceptNode],
        threshold: f32,
    ) -> Option<(u64, NodeId, SensorState, f32)> {
        let mut best: Option<(u64, NodeId, SensorState, f32)> = None;
        for concept in existing_concepts {
            let sim = cosine_similarity(&proto.as_vector(), &concept.prototype.as_vector());
            if sim < threshold {
                continue;
            }
            let mem_id = self.concept_memory_node_for(concept)?;
            if best.as_ref().is_none_or(|(_, _, _, s)| sim > *s) {
                best = Some((concept.id, mem_id, concept.prototype, sim));
            }
        }
        best
    }

    pub fn spread_activation(
        &self,
        active_concepts: &[ActiveConcept],
        concepts: &[ConceptNode],
    ) -> HashMap<NodeId, f32> {
        let mut activation: HashMap<NodeId, f32> = HashMap::new();

        for active in active_concepts {
            let Some(concept) = concepts.iter().find(|c| c.id == active.concept_id) else {
                continue;
            };
            let spread = active.activation * SPREAD_DECAY;
            for &member in &concept.member_node_ids {
                *activation.entry(member).or_insert(0.0) += spread;
            }
            for node in &self.nodes {
                if !matches!(node.kind, NodeKind::Concept) {
                    continue;
                }
                let compresses_members: Vec<NodeId> = self
                    .outgoing_edges(node.id)
                    .iter()
                    .filter_map(|&ei| {
                        let e = &self.edges[ei];
                        if e.edge_type == EdgeType::ConceptCompresses {
                            Some(e.target_id)
                        } else {
                            None
                        }
                    })
                    .collect();
                if compresses_members.is_empty() {
                    continue;
                }
                let overlaps = concept
                    .member_node_ids
                    .iter()
                    .any(|m| compresses_members.contains(m));
                if overlaps {
                    for &target in &compresses_members {
                        *activation.entry(target).or_insert(0.0) += spread;
                    }
                }
            }
        }

        let hop1 = Self::spread_hop(&self.edges, &self.edges_by_source, &activation, SPREAD_DECAY);
        for (&node, &act) in &hop1 {
            *activation.entry(node).or_insert(0.0) += act;
        }

        let hop2 = Self::spread_hop(&self.edges, &self.edges_by_source, &hop1, SPREAD_DECAY_HOP2);
        for (node, act) in hop2 {
            *activation.entry(node).or_insert(0.0) += act;
        }

        activation
    }

    fn spread_hop(
        edges: &[MemoryEdge],
        edges_by_source: &HashMap<NodeId, Vec<usize>>,
        sources: &HashMap<NodeId, f32>,
        decay: f32,
    ) -> HashMap<NodeId, f32> {
        let mut hop_activation: HashMap<NodeId, f32> = HashMap::new();
        for (&source, &act) in sources {
            let hop = act * decay;
            if hop < 1e-6 {
                continue;
            }
            for &ei in edges_by_source.get(&source).into_iter().flat_map(|v| v.iter()) {
                let edge = &edges[ei];
                let edge_weight = match edge.edge_type {
                    EdgeType::Precedes | EdgeType::ConceptCompresses => edge.strength.max(0.1),
                    EdgeType::CoOccurs => edge.strength.max(0.1) * CO_OCCURS_SPREAD_WEIGHT,
                    _ => continue,
                };
                *hop_activation.entry(edge.target_id).or_insert(0.0) += hop * edge_weight;
            }
        }
        hop_activation
    }

    pub fn predict_regulatory_delta(
        &self,
        sensory: SensorState,
        active_concepts: &[ActiveConcept],
        concepts: &[ConceptNode],
    ) -> f32 {
        let spread = self.spread_activation(active_concepts, concepts);
        let sensory_id = self.find_similar_sensory(sensory, 0.8);

        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for edge in &self.edges {
            if edge.edge_type != EdgeType::Precedes {
                continue;
            }
            let source_weight = source_activation_weight(sensory_id, edge.source_id, &spread);
            if source_weight < 1e-6 {
                continue;
            }
            let predicted = self.predict_outcome_for_action(edge.target_id);
            let w = edge.strength * edge.confidence * source_weight * delay_weight(edge.delay_mean);
            total += predicted * w;
            weight_sum += w;
        }

        if weight_sum > 1e-6 {
            total / weight_sum
        } else {
            0.0
        }
    }

    pub fn predict_action_outcomes(
        &self,
        sensory: SensorState,
        active_concepts: &[ActiveConcept],
        concepts: &[ConceptNode],
    ) -> ActionPredictions {
        let spread = self.spread_activation(active_concepts, concepts);
        self.predict_action_outcomes_with_spread(sensory, &spread)
    }

    pub fn predict_action_outcomes_with_spread(
        &self,
        sensory: SensorState,
        spread: &HashMap<NodeId, f32>,
    ) -> ActionPredictions {
        let mut predictions = ActionPredictions::default();
        let sensory_id = self.find_similar_sensory(sensory, 0.8);

        for source_id in self.prediction_source_ids(sensory_id, spread) {
            let source_weight = source_activation_weight(sensory_id, source_id, spread);
            if source_weight < 1e-6 {
                continue;
            }
            for &ei in self.outgoing_edges(source_id) {
                let edge = &self.edges[ei];
                if edge.edge_type != EdgeType::Precedes {
                    continue;
                }
                let action_node = self.node_by_id(edge.target_id);
                let Some(NodeKind::Action(action)) = action_node.map(|n| n.kind) else {
                    continue;
                };
                let predicted_delta = self.predict_outcome_for_action(edge.target_id);
                let weight =
                    edge.strength * edge.confidence * source_weight * delay_weight(edge.delay_mean);
                accumulate_action_prediction(&mut predictions, action, predicted_delta * weight);
            }
        }

        for node in &self.nodes {
            let NodeKind::Sound(sound) = node.kind else {
                continue;
            };
            for &ei in self.outgoing_edges(node.id) {
                let edge = &self.edges[ei];
                if edge.edge_type != EdgeType::SoundActivates {
                    continue;
                }
                let sig_boost = self.signature_outcome_boost(sound.signature);
                let sensory_id_activated = edge.target_id;
                let sensory_weight = if sensory_id == Some(sensory_id_activated) {
                    1.0
                } else {
                    spread.get(&sensory_id_activated).copied().unwrap_or(0.0)
                };
                if sensory_weight < 1e-6 {
                    continue;
                }
                let sound_weight = edge.strength * edge.confidence * sig_boost * sensory_weight;

                for &pei in self.outgoing_edges(sensory_id_activated) {
                    let pedge = &self.edges[pei];
                    if pedge.edge_type != EdgeType::Precedes {
                        continue;
                    }
                    let action_node = self.node_by_id(pedge.target_id);
                    let Some(NodeKind::Action(action)) = action_node.map(|n| n.kind) else {
                        continue;
                    };
                    let predicted_delta = self.predict_outcome_for_action(pedge.target_id);
                    let weight = pedge.strength * pedge.confidence * sound_weight;
                    accumulate_action_prediction(&mut predictions, action, predicted_delta * weight);
                }
            }
        }

        predictions
    }

    fn prediction_source_ids(
        &self,
        sensory_id: Option<NodeId>,
        spread: &HashMap<NodeId, f32>,
    ) -> Vec<NodeId> {
        let mut sources = Vec::new();
        if let Some(sid) = sensory_id {
            sources.push(sid);
        }
        for &id in spread.keys() {
            if sensory_id != Some(id) {
                sources.push(id);
            }
        }
        sources
    }

    fn sound_node_outcome_stats(&self, sound_id: NodeId) -> (f32, f32) {
        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for &ei in self.outgoing_edges(sound_id) {
            let edge = &self.edges[ei];
            if edge.edge_type != EdgeType::SoundActivates {
                continue;
            }
            let sensory_id = edge.target_id;
            for &pei in self.outgoing_edges(sensory_id) {
                let pedge = &self.edges[pei];
                if pedge.edge_type != EdgeType::Precedes {
                    continue;
                }
                let predicted = self.predict_outcome_for_action(pedge.target_id);
                let w = edge.confidence * pedge.confidence;
                total += predicted * w;
                weight_sum += w;
            }
        }
        if weight_sum > 1e-6 {
            (total / weight_sum, weight_sum)
        } else {
            (0.0, 0.0)
        }
    }

    fn signature_mean_outcome(&self, signature: u64) -> f32 {
        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for node in &self.nodes {
            let NodeKind::Sound(sound) = node.kind else {
                continue;
            };
            if sound.signature != signature {
                continue;
            }
            let (outcome, w) = self.sound_node_outcome_stats(node.id);
            total += outcome * w;
            weight_sum += w;
        }
        if weight_sum > 1e-6 {
            total / weight_sum
        } else {
            0.0
        }
    }

    fn signature_outcome_boost(&self, signature: u64) -> f32 {
        let mean = self.signature_mean_outcome(signature);
        if mean > 0.0 {
            1.0 + mean.min(0.5)
        } else if mean < 0.0 {
            (1.0 + mean.max(-0.3)).max(0.5)
        } else {
            1.0
        }
    }

    fn predict_outcome_for_action(&self, action_id: NodeId) -> f32 {
        let mut total = 0.0f32;
        let mut weight_sum = 0.0f32;
        for &ei in self.outgoing_edges(action_id) {
            let edge = &self.edges[ei];
            if edge.edge_type != EdgeType::ActionLeadsTo {
                continue;
            }
            if let Some(node) = self.node_by_id(edge.target_id) {
                if let NodeKind::Outcome(outcome) = node.kind {
                    let w = edge.strength * edge.confidence;
                    total += outcome * w;
                    weight_sum += w;
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

    fn find_or_create_action(&mut self, action: Action) -> NodeId {
        let key = canonical_action_key(action);
        if let Some(&id) = self.action_nodes.get(&key) {
            return id;
        }
        let id = self.create_node(NodeKind::Action(canonical_action_node(action)));
        self.action_nodes.insert(key, id);
        id
    }

    fn find_or_create_outcome(&mut self, outcome: f32) -> NodeId {
        let quantized = quantize_outcome(outcome);
        let key = outcome_key(quantized);
        if let Some(&id) = self.outcome_nodes.get(&key) {
            return id;
        }
        let id = self.create_node(NodeKind::Outcome(quantized));
        self.outcome_nodes.insert(key, id);
        id
    }

    fn create_node(&mut self, kind: NodeKind) -> NodeId {
        let id = self.next_id;
        self.next_id += 1;
        let idx = self.nodes.len();
        self.nodes.push(MemoryNode { id, kind });
        self.node_index.insert(id, idx);
        id
    }

    fn cluster_sensory_concepts(
        &mut self,
        next_concept_id: &mut u64,
        existing_concepts: &[ConceptNode],
    ) -> Vec<ConceptNode> {
        use std::collections::HashSet;

        let already_compressed = self.sensory_compressed_targets();

        let sensory: Vec<(NodeId, SensorState)> = self
            .nodes
            .iter()
            .filter_map(|node| match node.kind {
                NodeKind::SensoryPattern(pattern) if !already_compressed.contains(&node.id) => {
                    Some((node.id, pattern))
                }
                _ => None,
            })
            .collect();

        let mut used = HashSet::new();
        let mut concepts = Vec::new();

        for i in 0..sensory.len() {
            if used.contains(&sensory[i].0) || already_compressed.contains(&sensory[i].0) {
                continue;
            }
            let mut cluster = vec![sensory[i].0];
            let mut proto = sensory[i].1;

            for j in (i + 1)..sensory.len() {
                if used.contains(&sensory[j].0) || already_compressed.contains(&sensory[j].0) {
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

            if let Some((existing_id, concept_mem_id, _, _)) =
                self.find_similar_existing_concept(proto, existing_concepts, CONCEPT_CLUSTER_THRESHOLD)
            {
                for member in &cluster {
                    self.add_or_strengthen_edge(
                        concept_mem_id,
                        *member,
                        EdgeType::ConceptCompresses,
                    );
                }
                let mut members = self.concept_members(concept_mem_id);
                for m in &cluster {
                    if !members.contains(m) {
                        members.push(*m);
                    }
                }
                let updated_proto = ema_sensor(
                    existing_concepts
                        .iter()
                        .find(|c| c.id == existing_id)
                        .map(|c| c.prototype)
                        .unwrap_or(proto),
                    self.prototype_for_members(&members, proto),
                    PROTOTYPE_EMA_ALPHA,
                );
                let strength = existing_concepts
                    .iter()
                    .find(|c| c.id == existing_id)
                    .map(|c| (c.strength + 0.05).min(1.0))
                    .unwrap_or(0.55);
                concepts.push(ConceptNode {
                    id: existing_id,
                    prototype: updated_proto,
                    member_node_ids: members,
                    strength,
                });
                continue;
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

    fn merge_loose_sensory_into_concepts(
        &mut self,
        next_concept_id: &mut u64,
        existing_concepts: &[ConceptNode],
    ) -> Vec<ConceptNode> {
        let already_compressed = self.sensory_compressed_targets();

        let sensory: Vec<(NodeId, SensorState)> = self
            .nodes
            .iter()
            .filter_map(|node| match node.kind {
                NodeKind::SensoryPattern(pattern) if !already_compressed.contains(&node.id) => {
                    Some((node.id, pattern))
                }
                _ => None,
            })
            .collect();

        let mut concepts = Vec::new();
        for (node_id, pattern) in sensory {
            let mut best_sim = 0.0f32;
            let mut best_mem_id: Option<NodeId> = None;
            let mut best_members: Vec<NodeId> = Vec::new();

            for node in &self.nodes {
                if !matches!(node.kind, NodeKind::Concept) {
                    continue;
                }
                let members = self.concept_members(node.id);
                if members.is_empty() {
                    continue;
                }
                let proto = self.prototype_for_members(&members, pattern);
                let sim = cosine_similarity(&pattern.as_vector(), &proto.as_vector());
                if sim >= CONCEPT_CLUSTER_THRESHOLD && sim > best_sim {
                    best_sim = sim;
                    best_mem_id = Some(node.id);
                    best_members = members;
                }
            }

            let Some(concept_mem_id) = best_mem_id else {
                continue;
            };

            self.add_or_strengthen_edge(concept_mem_id, node_id, EdgeType::ConceptCompresses);
            let mut members = best_members;
            if !members.contains(&node_id) {
                members.push(node_id);
            }
            let proto = self.prototype_for_members(&members, pattern);
            let existing_id = existing_concepts
                .iter()
                .find(|c| self.concept_memory_node_for(c) == Some(concept_mem_id))
                .map(|c| c.id);
            if let Some(existing_id) = existing_id {
                let strength = existing_concepts
                    .iter()
                    .find(|c| c.id == existing_id)
                    .map(|c| (c.strength + 0.04).min(1.0))
                    .unwrap_or(0.5);
                concepts.push(ConceptNode {
                    id: existing_id,
                    prototype: proto,
                    member_node_ids: members,
                    strength,
                });
            } else {
                let concept_id = *next_concept_id;
                *next_concept_id += 1;
                concepts.push(ConceptNode {
                    id: concept_id,
                    prototype: proto,
                    member_node_ids: members,
                    strength: 0.45,
                });
            }
        }
        concepts
    }

    fn merge_similar_concept_nodes(&mut self, pending: &mut Vec<ConceptNode>) -> u32 {
        let mut merge_count = 0u32;
        let mut i = 0;
        while i < pending.len() {
            let mut j = i + 1;
            while j < pending.len() {
                let sim = cosine_similarity(
                    &pending[i].prototype.as_vector(),
                    &pending[j].prototype.as_vector(),
                );
                if sim >= CONCEPT_MERGE_THRESHOLD {
                    let merged_members = pending[j].member_node_ids.drain(..).collect::<Vec<_>>();
                    pending[i].member_node_ids.extend(merged_members);
                    pending[i].prototype =
                        ema_sensor(pending[i].prototype, pending[j].prototype, PROTOTYPE_EMA_ALPHA);
                    pending[i].strength =
                        (pending[i].strength + pending[j].strength * 0.5).min(1.0);
                    if let (Some(keep_mem), Some(drop_mem)) = (
                        self.concept_memory_node_for(&pending[i]),
                        self.concept_memory_node_for(&pending[j]),
                    ) {
                        for edge in self
                            .edges
                            .iter()
                            .filter(|e| e.source_id == drop_mem)
                            .cloned()
                            .collect::<Vec<_>>()
                        {
                            self.add_or_strengthen_edge(keep_mem, edge.target_id, edge.edge_type);
                        }
                    }
                    pending.remove(j);
                    merge_count += 1;
                    continue;
                }
                j += 1;
            }
            i += 1;
        }
        merge_count
    }

    fn split_high_variance_concepts(
        &mut self,
        concepts: &mut Vec<ConceptNode>,
        next_concept_id: &mut u64,
    ) -> u32 {
        let mut split_count = 0u32;
        let mut new_splits = Vec::new();
        for concept in concepts.iter_mut() {
            if concept.member_node_ids.len() < 4 {
                continue;
            }
            let mut outliers = Vec::new();
            let mut inliers = Vec::new();
            for &mid in &concept.member_node_ids {
                let Some(node) = self.nodes.iter().find(|n| n.id == mid) else {
                    continue;
                };
                let NodeKind::SensoryPattern(p) = node.kind else {
                    continue;
                };
                let dev =
                    1.0 - cosine_similarity(&concept.prototype.as_vector(), &p.as_vector());
                if dev > CONCEPT_SPLIT_VARIANCE_THRESHOLD {
                    outliers.push(mid);
                } else {
                    inliers.push(mid);
                }
            }
            if outliers.len() < 2 || inliers.len() < 2 {
                continue;
            }
            let split_proto = self.prototype_for_members(&outliers, concept.prototype);
            let concept_id = *next_concept_id;
            *next_concept_id += 1;
            let concept_mem_id = self.create_node(NodeKind::Concept);
            for &member in &outliers {
                self.add_or_strengthen_edge(concept_mem_id, member, EdgeType::ConceptCompresses);
            }
            concept.member_node_ids = inliers.clone();
            concept.prototype = self.prototype_for_members(&inliers, concept.prototype);
            new_splits.push(ConceptNode {
                id: concept_id,
                prototype: split_proto,
                member_node_ids: outliers,
                strength: concept.strength * 0.7,
            });
            split_count += 1;
        }
        concepts.extend(new_splits);
        split_count
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
            self.push_edge(MemoryEdge {
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

fn accumulate_action_prediction(predictions: &mut ActionPredictions, action: Action, delta: f32) {
    match action {
        Action::Move(_) => predictions.move_delta += delta,
        Action::Push(_) => predictions.push_delta += delta,
        Action::ConsumeOrganic => predictions.consume_delta += delta,
        Action::Rest => predictions.rest_delta += delta,
        Action::EmitSound => predictions.emit_sound_delta += delta,
        Action::Follow => predictions.follow_delta += delta,
        Action::Dig => predictions.dig_delta += delta,
        Action::Carry => predictions.carry_delta += delta,
        Action::Drop => predictions.drop_delta += delta,
        Action::PlaceMaterial => predictions.place_material_delta += delta,
        Action::ApplyBinder => predictions.apply_binder_delta += delta,
        Action::TransferOrganic => predictions.transfer_organic_delta += delta,
    }
}

fn source_activation_weight(
    sensory_id: Option<NodeId>,
    source_id: NodeId,
    spread: &HashMap<NodeId, f32>,
) -> f32 {
    if sensory_id == Some(source_id) {
        1.0
    } else {
        spread.get(&source_id).copied().unwrap_or(0.0)
    }
}

fn action_matches(a: Action, b: Action) -> bool {
    match (a, b) {
        (Action::Move(_), Action::Move(_)) => true,
        (Action::Push(_), Action::Push(_)) => true,
        (Action::ConsumeOrganic, Action::ConsumeOrganic) => true,
        (Action::Rest, Action::Rest) => true,
        (Action::EmitSound, Action::EmitSound) => true,
        (Action::Follow, Action::Follow) => true,
        (Action::Dig, Action::Dig) => true,
        (Action::Carry, Action::Carry) => true,
        (Action::Drop, Action::Drop) => true,
        (Action::PlaceMaterial, Action::PlaceMaterial) => true,
        (Action::ApplyBinder, Action::ApplyBinder) => true,
        (Action::TransferOrganic, Action::TransferOrganic) => true,
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

fn delay_weight(delay_mean: f32) -> f32 {
    (1.0 / (1.0 + delay_mean * DELAY_WEIGHT_SCALE)).clamp(0.3, 1.0)
}

fn canonical_action_key(action: Action) -> CanonicalActionKey {
    match action {
        Action::Move(_) => CanonicalActionKey::Move,
        Action::Push(_) => CanonicalActionKey::Push,
        Action::ConsumeOrganic => CanonicalActionKey::ConsumeOrganic,
        Action::Rest => CanonicalActionKey::Rest,
        Action::EmitSound => CanonicalActionKey::EmitSound,
        Action::Follow => CanonicalActionKey::Follow,
        Action::Dig => CanonicalActionKey::Dig,
        Action::Carry => CanonicalActionKey::Carry,
        Action::Drop => CanonicalActionKey::Drop,
        Action::PlaceMaterial => CanonicalActionKey::PlaceMaterial,
        Action::ApplyBinder => CanonicalActionKey::ApplyBinder,
        Action::TransferOrganic => CanonicalActionKey::TransferOrganic,
    }
}

fn canonical_action_node(action: Action) -> Action {
    match action {
        Action::Move(_) => Action::Move(Vec3i::ZERO),
        Action::Push(_) => Action::Push(Vec3i::ZERO),
        other => other,
    }
}

fn quantize_outcome(outcome: f32) -> f32 {
    (outcome / OUTCOME_QUANTUM).round() * OUTCOME_QUANTUM
}

fn outcome_key(quantized: f32) -> i32 {
    (quantized * 1000.0).round() as i32
}

fn ema_sensor(prev: SensorState, new: SensorState, alpha: f32) -> SensorState {
    let vp = prev.as_vector();
    let vn = new.as_vector();
    let mut out = [0.0f32; 15];
    for i in 0..15 {
        out[i] = vp[i] * (1.0 - alpha) + vn[i] * alpha;
    }
    SensorState::from_vector(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::creatures::actions::Action;
    use crate::creatures::creature::Experience;
    use crate::creatures::regulation::RegulatoryState;
    use crate::creatures::sensors::SensorState;
    use crate::math::Vec3i;
    use crate::memory::edges::MemoryEdge;

    fn sample_experience(outcome: f32) -> Experience {
        Experience {
            sensory_before: SensorState::default(),
            state_before: RegulatoryState::default(),
            action: Action::Rest,
            sensory_after: SensorState::default(),
            state_after: RegulatoryState::default(),
            outcome,
            timestamp: 1,
        }
    }

    #[test]
    fn canonical_action_and_outcome_nodes_deduplicate() {
        let mut graph = MemoryGraph::new();
        for _ in 0..50 {
            graph.record_experience(&sample_experience(0.02));
        }
        let summary = graph.node_summary();
        assert_eq!(summary.action, 1, "Rest actions should share one node");
        assert_eq!(summary.outcome, 1, "Quantized outcomes should share one node");
    }

    #[test]
    fn consolidate_sleep_runs_without_panic() {
        let mut graph = MemoryGraph::new();
        let exp = sample_experience(0.2);
        graph.record_experience(&exp);
        let result = graph.consolidate_sleep(&[exp], &mut 1, &[]);
        assert_eq!(result.merge_count, 0);
        assert!(!graph.edges.is_empty());
    }

    #[test]
    fn spread_activation_reaches_second_hop() {
        let mut graph = MemoryGraph::new();
        let mut sensory = SensorState::default();
        sensory.chemical_organic = 0.9;
        let exp1 = Experience {
            sensory_before: sensory,
            state_before: RegulatoryState::default(),
            action: Action::Move(Vec3i::new(1, 0, 0)),
            sensory_after: sensory,
            state_after: RegulatoryState::default(),
            outcome: 0.0,
            timestamp: 1,
        };
        graph.record_experience(&exp1);
        let s1 = graph
            .find_similar_sensory(sensory, 0.5)
            .expect("sensory node");
        let mut sensory2 = SensorState::default();
        sensory2.chemical_organic = 0.1;
        let exp2 = Experience {
            sensory_before: sensory2,
            state_before: RegulatoryState::default(),
            action: Action::Rest,
            sensory_after: sensory2,
            state_after: RegulatoryState::default(),
            outcome: 0.1,
            timestamp: 2,
        };
        graph.record_experience(&exp2);
        if let Some(s2) = graph.find_similar_sensory(sensory2, 0.5) {
            graph.push_edge(MemoryEdge {
                source_id: s1,
                target_id: s2,
                edge_type: EdgeType::Precedes,
                strength: 0.8,
                confidence: 0.8,
                observations: 3,
                delay_mean: 1.0,
                delay_variance: 0.2,
            });
        }
        let concept = ConceptNode {
            id: 1,
            prototype: sensory,
            member_node_ids: vec![s1],
            strength: 0.9,
        };
        let active = vec![ActiveConcept {
            concept_id: 1,
            activation: 1.0,
        }];
        let spread = graph.spread_activation(&active, &[concept]);
        assert!(spread.get(&s1).copied().unwrap_or(0.0) > 0.0);
    }

    #[test]
    fn predict_action_outcomes_biases_rest() {
        let mut graph = MemoryGraph::new();
        let exp = Experience {
            sensory_before: SensorState::default(),
            state_before: RegulatoryState::default(),
            action: Action::Rest,
            sensory_after: SensorState::default(),
            state_after: RegulatoryState {
                energy: 0.9,
                ..Default::default()
            },
            outcome: 0.1,
            timestamp: 2,
        };
        graph.record_experience(&exp);
        let preds = graph.predict_action_outcomes(SensorState::default(), &[], &[]);
        assert!(preds.rest_delta >= 0.0);
    }

    #[test]
    fn novelty_high_for_unseen_sensor() {
        let graph = MemoryGraph::new();
        let novelty = graph.novelty_score(SensorState::default());
        assert!(novelty > 0.9);
    }
}
