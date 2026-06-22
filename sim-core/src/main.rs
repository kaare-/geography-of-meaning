use std::path::PathBuf;

use clap::Parser;
use sim_core::export::export_all;
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
}

fn main() {
    let args = Args::parse();

    let config = SimulationConfig {
        ticks: args.ticks,
        seed: args.seed,
        world_chunks: args.world_size,
        creature_count: args.creatures,
        output_dir: PathBuf::from(&args.output),
        ..SimulationConfig::default()
    };

    let mut sim = Simulation::new(config.clone());
    sim.run();

    let output_dir = &config.output_dir;
    if let Err(e) = export_all(&sim, output_dir) {
        eprintln!("Export failed: {e}");
        std::process::exit(1);
    }

    let snapshot_path = output_dir.join("snapshots/world_final.json");
    let log_path = output_dir.join("logs/tick_log.jsonl");

    let total_births: usize = sim.tick_logs.iter().map(|e| e.births.len()).sum();
    let total_deaths: usize = sim.tick_logs.iter().map(|e| e.deaths.len()).sum();
    let sleep_events: usize = sim
        .tick_logs
        .iter()
        .flat_map(|e| e.creatures.iter())
        .filter(|c| c.sleeping)
        .count();
    let total_sounds: usize = sim.tick_logs.iter().map(|e| e.sound_event_count).sum();
    let concept_total: usize = sim.creatures.iter().map(|c| c.concepts.len()).sum();

    println!("Simulation complete.");
    println!("  Ticks:         {}", config.ticks);
    println!("  Population:    {}", sim.creatures.len());
    println!("  Births:        {total_births}");
    println!("  Deaths:        {total_deaths}");
    println!("  Sleep events:  {sleep_events} creature-ticks");
    println!("  Sound events:  {total_sounds} (sum per-tick counts)");
    println!("  Concepts:      {concept_total} total across population");
    println!("  Chunks:        {}", sim.world.chunks.len());
    println!("  Snapshot:      {}", snapshot_path.display());
    println!("  Log:           {}", log_path.display());
}
