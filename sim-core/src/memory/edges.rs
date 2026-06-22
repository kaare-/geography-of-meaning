use serde::{Deserialize, Serialize};

use super::nodes::NodeId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EdgeType {
    CoOccurs,
    Precedes,
    ActionLeadsTo,
    SoundActivates,
    ConceptCompresses,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEdge {
    pub source_id: NodeId,
    pub target_id: NodeId,
    pub edge_type: EdgeType,
    pub strength: f32,
    pub confidence: f32,
    pub observations: u32,
    pub delay_mean: f32,
    pub delay_variance: f32,
}
