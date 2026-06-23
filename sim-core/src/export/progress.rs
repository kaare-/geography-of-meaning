use std::fs::{self, OpenOptions};
use std::io::{self, Write};

use crate::simulation::Simulation;

#[derive(Debug, Clone, Copy)]
pub struct ProgressSnapshot {
    pub tick: u64,
    pub total_ticks: u64,
    pub population: usize,
    pub births: u64,
    pub deaths: u64,
    pub concepts: usize,
    pub sleep_creature_ticks: u64,
    pub imagination_last_tick: u32,
}

impl ProgressSnapshot {
    pub fn from_simulation(sim: &Simulation, imagination_last_tick: u32) -> Self {
        let concept_total: usize = sim.creatures.iter().map(|c| c.concepts.len()).sum();
        Self {
            tick: sim.world.time,
            total_ticks: sim.config.ticks,
            population: sim.creatures.len(),
            births: sim.run_births,
            deaths: sim.run_deaths,
            concepts: concept_total,
            sleep_creature_ticks: sim.sleep_creature_ticks,
            imagination_last_tick,
        }
    }

    pub fn format_line(&self) -> String {
        format!(
            "tick {}/{} | pop={} births={} deaths={} concepts={} sleep={} imagine={}",
            self.tick,
            self.total_ticks,
            self.population,
            self.births,
            self.deaths,
            self.concepts,
            self.sleep_creature_ticks,
            self.imagination_last_tick,
        )
    }
}

pub fn emit_progress(sim: &Simulation, imagination_last_tick: u32) -> io::Result<()> {
    let line = ProgressSnapshot::from_simulation(sim, imagination_last_tick).format_line();
    println!("{line}");
    io::stdout().flush()?;

    if let Some(path) = &sim.config.progress_log {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)?;
        writeln!(file, "{line}")?;
    }

    Ok(())
}
