use serde::{Deserialize, Serialize};

use crate::creatures::actions::Action;
use crate::creatures::sensors::SensorState;

pub type NodeId = u64;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct SoundNode {
    pub intensity: f32,
    pub signature: u64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum NodeKind {
    SensoryPattern(SensorState),
    Action(Action),
    Outcome(f32),
    Sound(SoundNode),
    Concept,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryNode {
    pub id: NodeId,
    pub kind: NodeKind,
}
