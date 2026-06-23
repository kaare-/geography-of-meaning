use std::path::PathBuf;

use crate::creatures::lifecycle::DEFAULT_MAX_POPULATION;
use crate::world::TICKS_PER_DAY;

/// Erosion and other slow geological processes run every N simulation ticks.
pub const DEFAULT_EROSION_TICK_INTERVAL: u64 = 10;

/// Deep `water_content` horizontal flow on active chunks.
pub const GROUNDWATER_TICK_INTERVAL: u64 = 5;

/// Small nudge applied to `erosion_damage` on active chunks each erosion tick.
pub const EROSION_DAMAGE_NUDGE: f32 = 0.0001;

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub ticks: u64,
    pub seed: u64,
    pub world_chunks: usize,
    pub creature_count: usize,
    pub output_dir: PathBuf,
    /// Maximum living creatures; reproduction is suppressed at cap.
    pub max_population: usize,
    /// Climate and water update every creature tick on active chunks only.
    pub climate_water_every_tick: bool,
    /// Interval for erosion_damage placeholder updates (see `World::tick_erosion`).
    pub erosion_tick_interval: u64,
    /// Write `world_tick_{t}.json` every N ticks; 0 = final snapshot only.
    pub snapshot_interval: u64,
    /// Optional edge noise during sleep imagination replay.
    pub dream_noise: bool,
    /// Print a one-line progress message to stdout every N ticks; 0 = disabled.
    pub progress_every: u64,
    /// When set, append the same progress lines to this file (parent dirs created).
    pub progress_log: Option<PathBuf>,
    /// Ticks per simulated day (diurnal phase); mirrored from `world::TICKS_PER_DAY`.
    pub ticks_per_day: u64,
}

impl Default for SimulationConfig {
    fn default() -> Self {
        Self {
            ticks: 100,
            seed: 42,
            world_chunks: 2,
            creature_count: 5,
            output_dir: PathBuf::from("exports"),
            max_population: DEFAULT_MAX_POPULATION,
            climate_water_every_tick: true,
            erosion_tick_interval: DEFAULT_EROSION_TICK_INTERVAL,
            snapshot_interval: 0,
            dream_noise: false,
            progress_every: 0,
            progress_log: None,
            ticks_per_day: TICKS_PER_DAY,
        }
    }
}
