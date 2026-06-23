use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::creatures::{
    apply_action, choose_action, compute_follow_direction, deposit_creature_organic,
    dominant_heard_signature, dominant_heard_call, read_sensors_with_noise, resolve_position_overlaps,
    try_creature_move_at, try_creature_push_at, try_transfer_organic_at, try_reproduce, Action,
    Creature, DeathEvent, Experience, FOLLOW_ENERGY_COST, FOLLOW_FATIGUE_COST, Genome, Morphology,
    REPRODUCTION_ENERGY_COST,
};
use crate::world::{emit_environmental_sound, EnvironmentalSoundKind};
use crate::export::logs::{ActionCounts, TickLogEntry};
use crate::export::timing::{elapsed_ms, TickTimingMs, TimingWindow};
use crate::simulation::scheduler::EROSION_DAMAGE_NUDGE;
use crate::world::World;

use super::scheduler::SimulationConfig;

pub struct Simulation {
    pub world: World,
    pub creatures: Vec<Creature>,
    pub config: SimulationConfig,
    pub rng: StdRng,
    pub tick_logs: Vec<TickLogEntry>,
    next_creature_id: u64,
    pub(crate) run_births: u64,
    pub(crate) run_deaths: u64,
    pub(crate) sleep_creature_ticks: u64,
    pub(crate) timing_window: TimingWindow,
    pub(crate) pending_export_ms: f64,
    pub(crate) pending_snapshot_ms: f64,
}

impl Simulation {
    pub fn new(config: SimulationConfig) -> Self {
        let mut rng = StdRng::seed_from_u64(config.seed);
        let mut world = World::generate_terrain(config.world_chunks, config.seed);
        let spawn_positions = world.find_spawn_positions(config.creature_count);
        for pos in &spawn_positions {
            world.enrich_spawn_site(*pos);
        }

        let mut creatures = Vec::new();
        for (i, pos) in spawn_positions.iter().enumerate() {
            let signature = rng.gen::<u64>();
            let genome = random_spawn_genome(&mut rng);
            let morphology = Morphology::from_genome(&genome);
            let mut creature = Creature::new(i as u64 + 1, *pos, signature);
            creature.genome = genome;
            creature.morphology = morphology;
            creature.sensor =
                read_sensors_with_noise(&creature, &world, &creatures, &mut rng, 1.0);
            creatures.push(creature);
        }

        while creatures.len() < config.creature_count {
            let id = creatures.len() as u64 + 1;
            let signature = rng.gen::<u64>();
            let genome = random_spawn_genome(&mut rng);
            let morphology = Morphology::from_genome(&genome);
            let pos = spawn_positions
                .first()
                .copied()
                .unwrap_or(crate::math::Vec3f::new(8.0, 8.0, 4.0));
            let mut creature = Creature::new(id, pos, signature);
            creature.genome = genome;
            creature.morphology = morphology;
            creatures.push(creature);
        }

        let creature_count = config.creature_count;
        Self {
            world,
            creatures,
            config,
            rng,
            tick_logs: Vec::new(),
            next_creature_id: creature_count as u64 + 1,
            run_births: 0,
            run_deaths: 0,
            sleep_creature_ticks: 0,
            timing_window: TimingWindow::default(),
            pending_export_ms: 0.0,
            pending_snapshot_ms: 0.0,
        }
    }

    pub fn record_export_ms(&mut self, ms: f64) {
        self.pending_export_ms += ms;
    }

    pub fn record_snapshot_ms(&mut self, ms: f64) {
        self.pending_snapshot_ms += ms;
    }

    pub fn flush_timing_report(&mut self) -> std::io::Result<()> {
        if self.timing_window.is_empty() {
            return Ok(());
        }
        self.timing_window
            .emit(self.world.time, self.config.timing_log.as_deref())
    }

    pub fn tick(&mut self) {
        let tick_start = std::time::Instant::now();
        let mut timing = TickTimingMs::default();

        let rain_amount = if self.rng.gen::<f32>() < self.world.climate.rainfall_rate {
            self.rng.gen_range(0.05..0.2)
        } else {
            0.0
        };

        if rain_amount > 0.0 {
            self.world.queue_rain(rain_amount);
        }

        let world_start = std::time::Instant::now();
        self.world.process_events();
        timing.world_update_ms += elapsed_ms(world_start);

        if self.config.climate_water_every_tick {
            let (climate_ms, water_ms) = self.world.tick_climate_and_water();
            timing.climate_ms += climate_ms;
            timing.water_ms += water_ms;
            let groundwater_start = std::time::Instant::now();
            self.world.tick_groundwater();
            timing.groundwater_ms += elapsed_ms(groundwater_start);
        }

        if self.config.erosion_tick_interval > 0
            && self.world.time % self.config.erosion_tick_interval == 0
        {
            let (collapses, physics_ms, erosion_ms) =
                self.world.tick_erosion(EROSION_DAMAGE_NUDGE);
            timing.physics_ms += physics_ms;
            timing.erosion_ms += erosion_ms;
            for pos in collapses {
                emit_environmental_sound(
                    &mut self.world,
                    pos,
                    EnvironmentalSoundKind::Collapse,
                    0.7,
                );
            }
        }

        let sounds_start = std::time::Instant::now();
        self.world.tick_sounds();
        timing.world_update_ms += elapsed_ms(sounds_start);

        let timestamp = self.world.time;
        let day_phase = self.world.day_phase;
        let erosion_damage_tick = self.config.erosion_tick_interval > 0
            && timestamp % self.config.erosion_tick_interval == 0;

        let creature_count = self.creatures.len();
        let dream_noise = self.config.dream_noise;
        let mut chosen_actions = Vec::with_capacity(creature_count);
        let mut tick_concepts_formed = 0u32;
        let mut tick_imagination = 0u32;
        let mut tick_merge = 0u32;
        let mut tick_split = 0u32;

        for creature in &mut self.creatures {
            if creature.sleep.sleeping {
                self.sleep_creature_ticks += 1;
            }
            let (sleep_result, sleep_timing) = creature.update_sleep(dream_noise, &mut self.rng);
            timing.sleep_ms += sleep_timing.sleep_ms;
            timing.imagination_ms += sleep_timing.imagination_ms;
            timing.concept_creation_ms += sleep_timing.concept_creation_ms;
            timing.concept_merge_ms += sleep_timing.concept_merge_ms;
            tick_concepts_formed += sleep_result.concepts_formed;
            tick_imagination += sleep_result.imagination_events;
            tick_merge += sleep_result.merge_count;
            tick_split += sleep_result.split_count;
            creature.try_enter_sleep();
            creature.try_early_wake();
            let concept_start = std::time::Instant::now();
            creature.refresh_active_concepts();
            timing.concept_activation_ms += elapsed_ms(concept_start);
            let sleeping = creature.sleep.sleeping;
            let comm_start = std::time::Instant::now();
            let heard_signature = dominant_heard_signature(creature, &self.world);
            let heard_call_frequency =
                dominant_heard_call(creature, &self.world).map(|(_, freq)| freq);
            timing.communication_ms += elapsed_ms(comm_start);
            let action_start = std::time::Instant::now();
            chosen_actions.push(choose_action(
                creature,
                &mut self.rng,
                sleeping,
                heard_signature,
                heard_call_frequency,
                Some(&mut timing.prediction_ms),
            ));
            timing.action_selection_ms += elapsed_ms(action_start);
        }

        let mut deaths = Vec::new();
        let mut surviving = Vec::with_capacity(creature_count);
        let mut action_counts = ActionCounts::default();
        let mut push_events = Vec::new();
        let mut total_displacement = 0.0f32;
        let mut novel_sensor_ticks = 0u32;

        for idx in 0..creature_count {
            let creature_start = std::time::Instant::now();
            let action = chosen_actions[idx];
            let mut creature = self.creatures[idx].clone();
            let position_before = creature.position;

            let sensory_before = creature.sensor;
            let state_before = creature.regulatory;
            let sleeping = creature.sleep.sleeping;

            creature
                .regulatory
                .tick_passive_drain(
                    creature.genome.metabolism_rate,
                    creature.morphology.metabolism_multiplier(),
                );
            creature.regulatory.apply_environmental_stress(
                &creature.sensor,
                creature.morphology.heat_retention,
                creature.morphology.mass,
            );

            if erosion_damage_tick {
                let pos = creature.position.floor_i();
                if let Some(voxel) = self.world.sample_voxel(pos) {
                    if voxel.erosion_damage > 0.55 {
                        creature.regulatory.integrity -=
                            (voxel.erosion_damage - 0.55) * 0.008;
                    }
                }
            }

            let movement_start = std::time::Instant::now();
            match action {
                Action::Move(delta) => {
                    if try_creature_move_at(&mut self.creatures, idx, delta, &mut self.world) {
                        creature.position = self.creatures[idx].position;
                        creature.regulatory = self.creatures[idx].regulatory;
                        action_counts.move_count += 1;
                    }
                }
                Action::Push(delta) => {
                    if let Some(event) =
                        try_creature_push_at(&mut self.creatures, idx, delta, &mut self.world)
                    {
                        creature.position = self.creatures[idx].position;
                        creature.regulatory = self.creatures[idx].regulatory;
                        push_events.push(event);
                        action_counts.push_count += 1;
                    }
                }
                Action::Follow => {
                    if let Some(delta) =
                        compute_follow_direction(&self.creatures[idx], &self.creatures)
                    {
                        if try_creature_move_at(&mut self.creatures, idx, delta, &mut self.world) {
                            creature.position = self.creatures[idx].position;
                            creature.regulatory = self.creatures[idx].regulatory;
                            creature.regulatory.apply_action_cost(
                                FOLLOW_ENERGY_COST,
                                FOLLOW_FATIGUE_COST,
                            );
                            self.creatures[idx].regulatory = creature.regulatory;
                            action_counts.follow_count += 1;
                        }
                    }
                }
                Action::TransferOrganic => {
                    if try_transfer_organic_at(&mut self.creatures, idx, &mut self.rng) {
                        creature.regulatory = self.creatures[idx].regulatory;
                        action_counts.transfer_organic_count += 1;
                    }
                }
                _ => {
                    let comm_action_start = std::time::Instant::now();
                    let _ = apply_action(&mut creature, action, &mut self.world);
                    if matches!(action, Action::EmitSound) {
                        timing.communication_ms += elapsed_ms(comm_action_start);
                    }
                    self.creatures[idx].regulatory = creature.regulatory;
                    self.creatures[idx].position = creature.position;
                    match action {
                        Action::Dig => action_counts.dig_count += 1,
                        Action::Carry => action_counts.carry_count += 1,
                        Action::Drop => action_counts.drop_count += 1,
                        Action::PlaceMaterial => action_counts.place_material_count += 1,
                        Action::ApplyBinder => action_counts.apply_binder_count += 1,
                        _ => {}
                    }
                }
            }
            timing.movement_ms += elapsed_ms(movement_start);

            creature.regulatory.apply_passive_hydration(&creature.sensor);
            creature.regulatory.apply_dehydration_stress();
            creature
                .regulatory
                .apply_ambient_processing_cost(creature.sensor.sound_ambient);

            let noise_mult = if sleeping { 0.25 } else { 1.0 };
            creature.sensor = read_sensors_with_noise(
                &creature,
                &self.world,
                &self.creatures,
                &mut self.rng,
                noise_mult,
            );
            let memory_graph_start = std::time::Instant::now();
            if !sleeping {
                let heard = creature
                    .sensor
                    .sound_calls
                    .max(creature.sensor.sound_ambient);
                if let Some(sig) = dominant_heard_signature(&creature, &self.world) {
                    creature
                        .memory_graph
                        .record_heard_sound(creature.sensor, heard, sig);
                    timing.communication_ms += elapsed_ms(memory_graph_start);
                }
            }
            timing.memory_graph_ms += elapsed_ms(memory_graph_start);
            let concept_start = std::time::Instant::now();
            creature.refresh_active_concepts();
            timing.concept_activation_ms += elapsed_ms(concept_start);
            if !sleeping && creature.memory_graph.novelty_score(creature.sensor) > 0.65 {
                novel_sensor_ticks += 1;
            }
            total_displacement += {
                let dx = creature.position.x - position_before.x;
                let dy = creature.position.y - position_before.y;
                let dz = creature.position.z - position_before.z;
                (dx * dx + dy * dy + dz * dz).sqrt()
            };
            creature
                .regulatory
                .clamp(creature.morphology.reserve_capacity);
            if creature.regulatory.energy <= 0.0 {
                creature.regulatory.energy_depleted_ticks =
                    creature.regulatory.energy_depleted_ticks.saturating_add(1);
            } else {
                creature.regulatory.energy_depleted_ticks = 0;
            }
            creature.age += 1;

            if let Some(cause) = creature.death_cause() {
                deaths.push(DeathEvent {
                    creature_id: creature.id,
                    position: creature.position,
                    age: creature.age,
                    cause,
                });
                deposit_creature_organic(&mut self.world, &creature);
                timing.creature_update_ms += elapsed_ms(creature_start);
                continue;
            }

            let outcome = creature.regulatory.energy - state_before.energy;
            let exp = Experience {
                sensory_before,
                state_before,
                action,
                sensory_after: creature.sensor,
                state_after: creature.regulatory,
                outcome,
                timestamp,
            };

            let memory_start = std::time::Instant::now();
            if !sleeping {
                creature.memory_graph.record_experience(&exp);
            }
            creature.push_experience(exp);
            timing.memory_ms += elapsed_ms(memory_start);
            timing.creature_update_ms += elapsed_ms(creature_start);
            surviving.push(creature);
        }

        self.creatures = surviving;
        resolve_position_overlaps(&mut self.creatures, &self.world);

        self.run_deaths += deaths.len() as u64;

        let mut births = Vec::new();
        let mut new_offspring = Vec::new();
        let mut reproduction_parents = Vec::new();
        for (idx, creature) in self.creatures.iter().enumerate() {
            if self.creatures.len() + new_offspring.len() >= self.config.max_population {
                break;
            }
            if let Some((offspring, birth)) = try_reproduce(
                creature,
                &self.world,
                &mut self.rng,
                self.next_creature_id,
            ) {
                self.next_creature_id += 1;
                births.push(birth);
                reproduction_parents.push(idx);
                let mut offspring = offspring;
                offspring.sensor = read_sensors_with_noise(
                    &offspring,
                    &self.world,
                    &self.creatures,
                    &mut self.rng,
                    1.0,
                );
                new_offspring.push(offspring);
            }
        }
        self.creatures.extend(new_offspring);
        self.run_births += births.len() as u64;
        for idx in reproduction_parents {
            self.creatures[idx]
                .regulatory
                .apply_reproduction_cost(REPRODUCTION_ENERGY_COST);
        }

        let refresh_start = std::time::Instant::now();
        self.world
            .refresh_active_chunks(self.creatures.iter().map(|c| c.position));
        timing.world_update_ms += elapsed_ms(refresh_start);

        self.tick_logs.push(TickLogEntry {
            tick: timestamp,
            day_phase,
            rain_applied: rain_amount,
            sound_event_count: self.world.active_sound_count(),
            sound_events: self
                .world
                .active_sounds
                .iter()
                .take(8)
                .map(crate::export::logs::SoundEventSnapshot::from_event)
                .collect(),
            deaths,
            births,
            concepts_formed: tick_concepts_formed,
            concept_merge_count: tick_merge,
            concept_split_count: tick_split,
            imagination_events: tick_imagination,
            mean_displacement: if creature_count > 0 {
                total_displacement / creature_count as f32
            } else {
                0.0
            },
            novel_sensor_fraction: if creature_count > 0 {
                novel_sensor_ticks as f32 / creature_count as f32
            } else {
                0.0
            },
            transfer_count: action_counts.transfer_organic_count,
            action_counts,
            push_events,
            creatures: self
                .creatures
                .iter()
                .map(crate::export::snapshots::CreatureSnapshot::from_creature)
                .collect(),
        });

        timing.export_ms = self.pending_export_ms;
        timing.snapshot_ms = self.pending_snapshot_ms;
        self.pending_export_ms = 0.0;
        self.pending_snapshot_ms = 0.0;
        timing.total_tick_ms = elapsed_ms(tick_start);
        self.timing_window.record(timing);

        if self.config.progress_every > 0 && self.world.time % self.config.progress_every == 0 {
            if let Err(e) = crate::export::progress::emit_progress(self, tick_imagination) {
                eprintln!("Progress report failed: {e}");
            }
            if let Err(e) = self
                .timing_window
                .emit(self.world.time, self.config.timing_log.as_deref())
            {
                eprintln!("Timing report failed: {e}");
            }
        }
    }

    pub fn run(&mut self) {
        for _ in 0..self.config.ticks {
            self.tick();
        }
    }
}

fn random_spawn_genome<R: Rng + ?Sized>(rng: &mut R) -> Genome {
    let mut genome = Genome::default();
    genome.mass_bias = rng.gen_range(0.5..1.8);
    genome.heat_retention = rng.gen_range(0.2..0.9);
    genome.carry_bias = rng.gen_range(0.5..1.6);
    genome.reserve_bias = rng.gen_range(0.3..0.9);
    genome.metabolism_rate = rng.gen_range(0.005..0.012);
    genome.move_speed = rng.gen_range(0.7..1.4);
    genome.developmental_bias = rng.gen_range(0.05..0.35);
    genome
}
