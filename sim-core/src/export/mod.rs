use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use serde::Serialize;
use thiserror::Error;

use crate::simulation::Simulation;

pub mod logs;
pub mod snapshots;

#[derive(Debug, Error)]
pub enum ExportError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

pub fn write_snapshot(sim: &Simulation, path: &Path) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let snapshot = snapshots::WorldSnapshot::from_simulation(sim);
    let json = serde_json::to_string_pretty(&snapshot)?;
    fs::write(path, json)?;
    Ok(())
}

pub fn write_tick_log(sim: &Simulation, path: &Path) -> Result<(), ExportError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(path)?;
    for entry in &sim.tick_logs {
        let line = serde_json::to_string(entry)?;
        writeln!(file, "{line}")?;
    }
    Ok(())
}

pub fn export_all(sim: &Simulation, output_dir: &Path) -> Result<(), ExportError> {
    let snapshot_path = output_dir.join("snapshots/world_final.json");
    let log_path = output_dir.join("logs/tick_log.jsonl");
    write_snapshot(sim, &snapshot_path)?;
    write_tick_log(sim, &log_path)?;
    Ok(())
}

#[derive(Serialize)]
pub struct ExportSummary<'a> {
    pub snapshot_path: &'a str,
    pub log_path: &'a str,
    pub ticks: u64,
    pub creatures: usize,
    pub chunks: usize,
}
