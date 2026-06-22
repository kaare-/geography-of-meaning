use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::creatures::Creature;
use crate::memory::edges::MemoryEdge;
use crate::memory::nodes::MemoryNode;
use crate::simulation::Simulation;

use super::ExportError;

#[derive(Debug, Serialize)]
pub struct MemoryGraphExport {
    pub creature_id: u64,
    pub node_count: usize,
    pub edge_count: usize,
    pub nodes: Vec<MemoryNode>,
    pub edges: Vec<MemoryEdge>,
    pub nodes_by_type: crate::memory::graph::MemoryNodeSummary,
}

impl MemoryGraphExport {
    pub fn from_creature(creature: &Creature) -> Self {
        Self {
            creature_id: creature.id,
            node_count: creature.memory_graph.nodes.len(),
            edge_count: creature.memory_graph.edges.len(),
            nodes: creature.memory_graph.nodes.clone(),
            edges: creature.memory_graph.edges.clone(),
            nodes_by_type: creature.memory_graph.node_summary(),
        }
    }
}

pub fn write_memory_graph(creature: &Creature, path: &Path) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let export = MemoryGraphExport::from_creature(creature);
    let json = serde_json::to_string_pretty(&export)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn export_memory_for_sim(sim: &Simulation, output_dir: &Path) -> Result<Option<String>, ExportError> {
    let sample = sim
        .creatures
        .iter()
        .min_by_key(|c| c.id)
        .or_else(|| sim.creatures.first());

    let Some(creature) = sample else {
        return Ok(None);
    };

    let path = output_dir.join(format!("snapshots/memory_creature_{}.json", creature.id));
    write_memory_graph(creature, &path)?;
    Ok(Some(path.to_string_lossy().into_owned()))
}
