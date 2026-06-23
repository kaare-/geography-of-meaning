use std::fs::{self, OpenOptions};
use std::io::{self, Write};

/// Per-tick subsystem timings in milliseconds (one simulation tick).
#[derive(Debug, Default, Clone, Copy)]
pub struct TickTimingMs {
    pub world_update_ms: f64,
    pub climate_ms: f64,
    pub water_ms: f64,
    pub groundwater_ms: f64,
    pub erosion_ms: f64,
    pub physics_ms: f64,
    pub creature_update_ms: f64,
    pub action_selection_ms: f64,
    pub movement_ms: f64,
    pub communication_ms: f64,
    pub memory_ms: f64,
    pub memory_graph_ms: f64,
    pub prediction_ms: f64,
    pub concept_activation_ms: f64,
    pub sleep_ms: f64,
    pub concept_creation_ms: f64,
    pub concept_merge_ms: f64,
    pub imagination_ms: f64,
    pub export_ms: f64,
    pub snapshot_ms: f64,
    pub total_tick_ms: f64,
}

impl TickTimingMs {
    pub fn add(&mut self, other: TickTimingMs) {
        self.world_update_ms += other.world_update_ms;
        self.climate_ms += other.climate_ms;
        self.water_ms += other.water_ms;
        self.groundwater_ms += other.groundwater_ms;
        self.erosion_ms += other.erosion_ms;
        self.physics_ms += other.physics_ms;
        self.creature_update_ms += other.creature_update_ms;
        self.action_selection_ms += other.action_selection_ms;
        self.movement_ms += other.movement_ms;
        self.communication_ms += other.communication_ms;
        self.memory_ms += other.memory_ms;
        self.memory_graph_ms += other.memory_graph_ms;
        self.prediction_ms += other.prediction_ms;
        self.concept_activation_ms += other.concept_activation_ms;
        self.sleep_ms += other.sleep_ms;
        self.concept_creation_ms += other.concept_creation_ms;
        self.concept_merge_ms += other.concept_merge_ms;
        self.imagination_ms += other.imagination_ms;
        self.export_ms += other.export_ms;
        self.snapshot_ms += other.snapshot_ms;
        self.total_tick_ms += other.total_tick_ms;
    }

    pub fn scale(&self, divisor: f64) -> TickTimingMs {
        if divisor <= 0.0 {
            return *self;
        }
        let mut out = *self;
        out.world_update_ms /= divisor;
        out.climate_ms /= divisor;
        out.water_ms /= divisor;
        out.groundwater_ms /= divisor;
        out.erosion_ms /= divisor;
        out.physics_ms /= divisor;
        out.creature_update_ms /= divisor;
        out.action_selection_ms /= divisor;
        out.movement_ms /= divisor;
        out.communication_ms /= divisor;
        out.memory_ms /= divisor;
        out.memory_graph_ms /= divisor;
        out.prediction_ms /= divisor;
        out.concept_activation_ms /= divisor;
        out.sleep_ms /= divisor;
        out.concept_creation_ms /= divisor;
        out.concept_merge_ms /= divisor;
        out.imagination_ms /= divisor;
        out.export_ms /= divisor;
        out.snapshot_ms /= divisor;
        out.total_tick_ms /= divisor;
        out
    }

    pub fn format_csv_header() -> &'static str {
        "world_update_ms,climate_ms,water_ms,groundwater_ms,erosion_ms,physics_ms,\
creature_update_ms,action_selection_ms,movement_ms,communication_ms,\
memory_ms,memory_graph_ms,prediction_ms,concept_activation_ms,\
sleep_ms,concept_creation_ms,concept_merge_ms,imagination_ms,\
export_ms,snapshot_ms,total_tick_ms"
    }

    pub fn format_csv_values(&self) -> String {
        format!(
            "{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3},{:.3}",
            self.world_update_ms,
            self.climate_ms,
            self.water_ms,
            self.groundwater_ms,
            self.erosion_ms,
            self.physics_ms,
            self.creature_update_ms,
            self.action_selection_ms,
            self.movement_ms,
            self.communication_ms,
            self.memory_ms,
            self.memory_graph_ms,
            self.prediction_ms,
            self.concept_activation_ms,
            self.sleep_ms,
            self.concept_creation_ms,
            self.concept_merge_ms,
            self.imagination_ms,
            self.export_ms,
            self.snapshot_ms,
            self.total_tick_ms,
        )
    }
}

#[derive(Debug, Default)]
pub struct TimingWindow {
    accumulated: TickTimingMs,
    tick_count: u64,
    header_written: bool,
}

impl TimingWindow {
    pub fn record(&mut self, tick: TickTimingMs) {
        self.accumulated.add(tick);
        self.tick_count += 1;
    }

    pub fn avg(&self) -> TickTimingMs {
        self.accumulated.scale(self.tick_count as f64)
    }

    pub fn reset(&mut self) {
        self.accumulated = TickTimingMs::default();
        self.tick_count = 0;
    }

    pub fn is_empty(&self) -> bool {
        self.tick_count == 0
    }

    pub fn emit(&mut self, tick: u64, timing_log: Option<&std::path::Path>) -> io::Result<()> {
        if self.tick_count == 0 {
            return Ok(());
        }
        let avg = self.avg();
        let line = format!("tick {tick} timing_avg_ms: {}", avg.format_csv_values());
        println!("{line}");
        io::stdout().flush()?;

        if let Some(path) = timing_log {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(path)?;
            if !self.header_written {
                writeln!(file, "tick,{}", TickTimingMs::format_csv_header())?;
                self.header_written = true;
            }
            writeln!(file, "{tick},{}", avg.format_csv_values())?;
        }

        self.reset();
        Ok(())
    }
}

pub fn elapsed_ms(start: std::time::Instant) -> f64 {
    start.elapsed().as_secs_f64() * 1000.0
}
