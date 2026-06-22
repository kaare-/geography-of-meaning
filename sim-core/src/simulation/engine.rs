use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};

use crate::creatures::{
    apply_action, choose_action, deposit_creature_organic, read_sensors_with_noise,
    try_reproduce, Creature, DeathEvent, Experience, REPRODUCTION_ENERGY_COST,
};
use crate::export::logs::TickLogEntry;
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
        let world = World::generate_terrain(config.world_chunks, config.seed);
        let spawn_positions = world.find_spawn_positions(config.creature_count);

        let mut creatures = Vec::new();
        for (i, pos) in spawn_positions.iter().enumerate() {
            let signature = rng.gen::<u64>();
            let mut creature = Creature::new(i as u64 + 1, *pos, signature);
            creature.sensor = read_sensors_with_noise(&creature, &world, &mut rng, 1.0);
            creatures.push(creature);
        }

        while creatures.len() < config.creature_count {
            let id = creatures.len() as u64 + 1;
            let signature = rng.gen::<u64>();
            let pos = spawn_positions
                .first()
                .copied()
                .unwrap_or(crate::math::Vec3f::new(8.0, 8.0, 4.0));
            creatures.push(Creature::new(id, pos, signature));
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

        let mut deaths = Vec::new();
        let mut surviving = Vec::with_capacity(self.creatures.len());

        for mut creature in self.creatures.drain(..) {
            creature.update_sleep();
            creature.try_enter_sleep();

            let sensory_before = creature.sensor;
            let state_before = creature.regulatory;
            let sleeping = creature.sleep.sleeping;

            creature
                .regulatory
                .tick_passive_drain(creature.genome.metabolism_rate);
            creature
                .regulatory
                .apply_environmental_stress(&creature.sensor);

            if erosion_damage_tick {
                let pos = creature.position.floor_i();
                if let Some(voxel) = self.world.sample_voxel(pos) {
                    if voxel.erosion_damage > 0.5 {
                        creature.regulatory.integrity -=
                            (voxel.erosion_damage - 0.5) * 0.03;
                    }
                }
            }

            let action = choose_action(&creature, &mut self.rng, sleeping);
            let _ = apply_action(&mut creature, action, &mut self.world);

            creature.regulatory.apply_passive_hydration(&creature.sensor);

            let noise_mult = if sleeping { 0.25 } else { 1.0 };
            creature.sensor =
                read_sensors_with_noise(&creature, &self.world, &mut self.rng, noise_mult);
            if !sleeping {
                let heard = creature
                    .sensor
                    .sound_calls
                    .max(creature.sensor.sound_ambient);
                creature.memory_graph.record_heard_sound(creature.sensor, heard);
            }
            creature.refresh_active_concepts();
            creature.regulatory.clamp();
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
            surviving.push(creature);
        }

        self.creatures = surviving;

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
                offspring.sensor =
                    read_sensors_with_noise(&offspring, &self.world, &mut self.rng, 1.0);
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
