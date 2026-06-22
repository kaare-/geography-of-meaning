use std::fs;
use std::path::Path;

use serde::Serialize;

use crate::creatures::Creature;
use crate::memory::edges::{EdgeType, MemoryEdge};
use crate::memory::nodes::{MemoryNode, NodeKind};
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

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn node_kind_label(kind: &NodeKind) -> String {
    match kind {
        NodeKind::SensoryPattern(_) => "sensory".to_string(),
        NodeKind::Action(action) => format!("action:{}", action.label()),
        NodeKind::Outcome(v) => format!("outcome:{v:.4}"),
        NodeKind::Sound(s) => format!("sound:{:.4}:sig{}", s.intensity, s.signature),
        NodeKind::Concept => "concept".to_string(),
    }
}

fn edge_type_label(edge_type: EdgeType) -> &'static str {
    match edge_type {
        EdgeType::CoOccurs => "co_occurs",
        EdgeType::Precedes => "precedes",
        EdgeType::ActionLeadsTo => "action_leads_to",
        EdgeType::SoundActivates => "sound_activates",
        EdgeType::ConceptCompresses => "concept_compresses",
    }
}

pub fn write_memory_graphml(creature: &Creature, path: &Path) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let graph = &creature.memory_graph;
    let mut out = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<graphml xmlns="http://graphml.graphdrawing.org/xmlns">
  <key id="kind" for="node" attr.name="kind" attr.type="string"/>
  <key id="strength" for="edge" attr.name="strength" attr.type="double"/>
  <key id="type" for="edge" attr.name="type" attr.type="string"/>
  <key id="confidence" for="edge" attr.name="confidence" attr.type="double"/>
  <graph id="memory" edgedefault="directed">
"#,
    );

    for node in &graph.nodes {
        let kind = xml_escape(&node_kind_label(&node.kind));
        out.push_str(&format!(
            "    <node id=\"n{}\"><data key=\"kind\">{kind}</data></node>\n",
            node.id
        ));
    }

    for edge in &graph.edges {
        let edge_type = edge_type_label(edge.edge_type);
        out.push_str(&format!(
            "    <edge source=\"n{}\" target=\"n{}\"><data key=\"strength\">{:.4}</data><data key=\"type\">{edge_type}</data><data key=\"confidence\">{:.4}</data></edge>\n",
            edge.source_id,
            edge.target_id,
            edge.strength,
            edge.confidence,
        ));
    }

    out.push_str("  </graph>\n</graphml>\n");
    fs::write(path, out)?;
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

    let json_path = output_dir.join(format!("snapshots/memory_creature_{}.json", creature.id));
    let graphml_path = output_dir.join(format!("snapshots/memory_creature_{}.graphml", creature.id));
    write_memory_graph(creature, &json_path)?;
    write_memory_graphml(creature, &graphml_path)?;
    Ok(Some(json_path.to_string_lossy().into_owned()))
}
