use clap::Parser;
use sim_core::export::{export_all, resolve_output_dir, write_snapshot};
use sim_core::export::memory_dump::write_interval_memory_export;
use sim_core::simulation::{Simulation, SimulationConfig};

#[derive(Parser, Debug)]
#[command(name = "geography-of-meaning")]
#[command(about = "Simulation engine for predictive organisms in a dynamic voxel world")]
struct Args {
    #[arg(long, default_value_t = 100)]
    ticks: u64,

    #[arg(long, default_value_t = 42)]
    seed: u64,

    #[arg(long, default_value_t = 2)]
    world_size: usize,

    #[arg(long, default_value_t = 5)]
    creatures: usize,

    #[arg(long, default_value = "exports")]
    output: String,

    /// Write world_tick_{t}.json every N ticks (0 = final snapshot only).
    #[arg(long, default_value_t = 0)]
    snapshot_interval: u64,

    /// Print progress every N ticks (0 = disabled).
    #[arg(long, default_value_t = 0)]
    progress_every: u64,

    /// Append progress lines to this file (same format as stdout).
    #[arg(long)]
    progress_log: Option<std::path::PathBuf>,

    /// Append per-window timing CSV rows (header written once).
    #[arg(long)]
    timing_log: Option<std::path::PathBuf>,

    /// Append creature position samples for 2D trajectory maps (x–y plane).
    #[arg(long)]
    trajectory_log: Option<std::path::PathBuf>,

    /// Sample trajectory rows every N ticks (default 10).
    #[arg(long, default_value_t = 10)]
    trajectory_every: u64,

    /// Only log these creature ids (comma-separated). Default: all creatures.
    #[arg(long, value_delimiter = ',')]
    trajectory_creatures: Option<Vec<u64>>,
}

fn main() {
    let args = Args::parse();

    let config = SimulationConfig {
        ticks: args.ticks,
        seed: args.seed,
        world_chunks: args.world_size,
        creature_count: args.creatures,
        output_dir: resolve_output_dir(&args.output),
        snapshot_interval: args.snapshot_interval,
        progress_every: args.progress_every,
        progress_log: args.progress_log,
        timing_log: args.timing_log,
        trajectory_log: args.trajectory_log.clone(),
        trajectory_every: args.trajectory_every,
        trajectory_creature_ids: args.trajectory_creatures.clone(),
        ..SimulationConfig::default()
    };

    let mut sim = Simulation::new(config.clone());
    let output_dir = config.output_dir.clone();
    let snapshot_interval = config.snapshot_interval;

    for _ in 0..config.ticks {
        sim.tick();
        if snapshot_interval > 0 && sim.world.time % snapshot_interval == 0 {
            let snapshot_start = std::time::Instant::now();
            let path = output_dir.join(format!("snapshots/world_tick_{}.json", sim.world.time));
            if let Err(e) = write_snapshot(&sim, &path) {
                eprintln!("Interval snapshot failed: {e}");
                std::process::exit(1);
            }
            if let Err(e) = write_interval_memory_export(&sim, &output_dir, sim.world.time) {
                eprintln!("Interval memory export failed: {e}");
                std::process::exit(1);
            }
            sim.record_snapshot_ms(snapshot_start.elapsed().as_secs_f64() * 1000.0);
        }
    }

    let export_start = std::time::Instant::now();
    if let Err(e) = export_all(&sim, &output_dir) {
        eprintln!("Export failed: {e}");
        std::process::exit(1);
    }
    sim.record_export_ms(export_start.elapsed().as_secs_f64() * 1000.0);
    if let Err(e) = sim.flush_timing_report() {
        eprintln!("Final timing report failed: {e}");
    }

    let snapshot_path = output_dir.join("snapshots/world_final.json");
    let log_path = output_dir.join("logs/tick_log.jsonl");
    let narrative_path = output_dir.join("logs/narrative_summary.json");

    let total_births: usize = sim.tick_logs.iter().map(|e| e.births.len()).sum();
    let total_deaths: usize = sim.tick_logs.iter().map(|e| e.deaths.len()).sum();
    let sleep_events: usize = sim
        .tick_logs
        .iter()
        .flat_map(|e| e.creatures.iter())
        .filter(|c| c.sleeping)
        .count();
    let total_sounds: usize = sim.tick_logs.iter().map(|e| e.sound_event_count).sum();
    let total_transfers: u32 = sim.tick_logs.iter().map(|e| e.transfer_count).sum();
    let concept_total: usize = sim.creatures.iter().map(|c| c.concepts.len()).sum();
    let sample_creature = sim.creatures.iter().min_by_key(|c| c.id);
    let memory_path = sample_creature.map(|c| {
        output_dir
            .join(format!("snapshots/memory_creature_{}.json", c.id))
            .display()
            .to_string()
    });
    let graphml_path = sample_creature.map(|c| {
        output_dir
            .join(format!("snapshots/memory_creature_{}.graphml", c.id))
            .display()
            .to_string()
    });

    println!("Simulation complete.");
    println!("  Ticks:         {}", config.ticks);
    println!("  Population:    {}", sim.creatures.len());
    println!("  Births:        {total_births}");
    println!("  Deaths:        {total_deaths}");
    println!("  Sleep events:  {sleep_events} creature-ticks");
    println!("  Sound events:  {total_sounds} (sum per-tick counts)");
    println!("  Transfers:     {total_transfers} organic proximity transfers");
    println!("  Concepts:      {concept_total} total across population");
    println!("  Chunks:        {}", sim.world.chunks.len());
    println!("  Snapshot:      {}", snapshot_path.display());
    if snapshot_interval > 0 {
        let interval_count = sim.world.time / snapshot_interval;
        println!("  Interval snaps:{interval_count} (every {snapshot_interval} ticks)");
    }
    println!("  Log:           {}", log_path.display());
    println!("  Narrative:     {}", narrative_path.display());
    if let Some(mem) = memory_path {
        println!("  Memory export: {mem}");
    }
    if let Some(gml) = graphml_path {
        println!("  GraphML export:{gml}");
    }
    if let Some(path) = &args.trajectory_log {
        println!("  Trajectory:    {}", path.display());
        println!(
            "  Trajectory map: run `python3 analysis/scripts/plot_trajectories.py {}`",
            path.display()
        );
    }
}
