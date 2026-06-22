use serde::{Deserialize, Serialize};

use crate::creatures::actions::Action;
use crate::creatures::sensors::SensorState;

pub type NodeId = u64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeKind {
    SensoryPattern(SensorState),
    Action(Action),
    Outcome(f32),
    Sound(f32),
    Concept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: NodeId,
    pub kind: NodeKind,
}
