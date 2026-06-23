use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

use serde::Serialize;
use thiserror::Error;

use crate::simulation::Simulation;

pub mod logs;
pub mod memory_dump;
pub mod narrative;
pub mod progress;
pub mod snapshots;

pub use memory_dump::write_memory_graphml;

/// Resolve a relative `--output` path against the workspace root (parent of `sim-core`).
/// Running `cargo run -p sim-core -- --output exports` from the workspace root writes to
/// `<workspace>/exports/`. The same flag from `sim-core/` also targets the workspace export
/// directory rather than `sim-core/exports/`.
pub fn resolve_output_dir(output: impl AsRef<Path>) -> PathBuf {
    let path = PathBuf::from(output.as_ref());
    if path.is_absolute() {
        return path;
    }
    workspace_root().join(path)
}

fn workspace_root() -> PathBuf {
    let cwd = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd
        .file_name()
        .is_some_and(|name| name == "sim-core")
    {
        return cwd
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or(cwd);
    }
    if cwd.join("sim-core").is_dir() {
        return cwd;
    }
    if cwd
        .parent()
        .is_some_and(|parent| parent.join("sim-core").is_dir())
    {
        return cwd.parent().map(Path::to_path_buf).unwrap_or(cwd);
    }
    cwd
}

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
    let narrative_path = output_dir.join("logs/narrative_summary.json");
    write_snapshot(sim, &snapshot_path)?;
    write_tick_log(sim, &log_path)?;
    narrative::write_narrative_summary(&sim.tick_logs, &narrative_path)?;
    let _ = memory_dump::export_memory_for_sim(sim, output_dir)?;
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
