use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::creatures::{
    apply_action, choose_action, compute_follow_direction, deposit_creature_organic,
    dominant_heard_signature, dominant_heard_call, read_sensors_with_noise, resolve_position_overlaps,
    try_creature_move_at, try_creature_push_at, try_reproduce, Action, Creature, DeathEvent,
    Experience, FOLLOW_ENERGY_COST, FOLLOW_FATIGUE_COST, Genome, Morphology,
    REPRODUCTION_ENERGY_COST,
};
use crate::export::logs::{ActionCounts, TickLogEntry};
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
        }
    }

    pub fn tick(&mut self) {
        let rain_amount = if self.rng.gen::<f32>() < self.world.climate.rainfall_rate {
            self.rng.gen_range(0.05..0.2)
        } else {
            0.0
        };

        if rain_amount > 0.0 {
            self.world.queue_rain(rain_amount);
        }

        self.world.process_events();

        if self.config.climate_water_every_tick {
            self.world.tick_climate_and_water();
            self.world.tick_groundwater();
        }

        if self.config.erosion_tick_interval > 0
            && self.world.time % self.config.erosion_tick_interval == 0
        {
            self.world.tick_erosion(EROSION_DAMAGE_NUDGE);
        }

        self.world.tick_sounds();

        let timestamp = self.world.time;
        let day_phase = self.world.day_phase;
        let erosion_damage_tick = self.config.erosion_tick_interval > 0
            && timestamp % self.config.erosion_tick_interval == 0;

        let creature_count = self.creatures.len();
        let mut chosen_actions = Vec::with_capacity(creature_count);
        for creature in &self.creatures {
            let sleeping = creature.sleep.sleeping;
            let heard_signature = dominant_heard_signature(creature, &self.world);
            let heard_call_frequency =
                dominant_heard_call(creature, &self.world).map(|(_, freq)| freq);
            chosen_actions.push(choose_action(
                creature,
                &mut self.rng,
                sleeping,
                heard_signature,
                heard_call_frequency,
            ));
        }

        let mut deaths = Vec::new();
        let mut surviving = Vec::with_capacity(creature_count);
        let mut concepts_formed = 0u32;
        let mut action_counts = ActionCounts::default();
        let mut push_events = Vec::new();

        for idx in 0..creature_count {
            let action = chosen_actions[idx];
            let mut creature = self.creatures[idx].clone();

            concepts_formed += creature.update_sleep();
            creature.try_enter_sleep();

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
                    if voxel.erosion_damage > 0.5 {
                        creature.regulatory.integrity -=
                            (voxel.erosion_damage - 0.5) * 0.02;
                    }
                }
            }

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
                _ => {
                    let _ = apply_action(&mut creature, action, &mut self.world);
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

            creature.regulatory.apply_passive_hydration(&creature.sensor);

            let noise_mult = if sleeping { 0.25 } else { 1.0 };
            creature.sensor = read_sensors_with_noise(
                &creature,
                &self.world,
                &self.creatures,
                &mut self.rng,
                noise_mult,
            );
            if !sleeping {
                let heard = creature
                    .sensor
                    .sound_calls
                    .max(creature.sensor.sound_ambient);
                if let Some(sig) = dominant_heard_signature(&creature, &self.world) {
                    creature
                        .memory_graph
                        .record_heard_sound(creature.sensor, heard, sig);
                }
            }
            creature.refresh_active_concepts();
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

            if !sleeping {
                creature.memory_graph.record_experience(&exp);
            }
            creature.push_experience(exp);
            self.creatures[idx] = creature.clone();
            surviving.push(creature);
        }

        self.creatures = surviving;
        resolve_position_overlaps(&mut self.creatures, &self.world);

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
        for idx in reproduction_parents {
            self.creatures[idx]
                .regulatory
                .apply_reproduction_cost(REPRODUCTION_ENERGY_COST);
        }

        self.world
            .refresh_active_chunks(self.creatures.iter().map(|c| c.position));

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
            concepts_formed,
            action_counts,
            push_events,
            creatures: self
                .creatures
                .iter()
                .map(crate::export::snapshots::CreatureSnapshot::from_creature)
                .collect(),
        });
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
    genome.metabolism_rate = rng.gen_range(0.005..0.014);
    genome.move_speed = rng.gen_range(0.7..1.4);
    genome.developmental_bias = rng.gen_range(0.05..0.35);
    genome
}
