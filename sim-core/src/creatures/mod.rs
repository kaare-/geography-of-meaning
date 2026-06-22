pub mod actions;
pub mod creature;
pub mod genome;
pub mod lifecycle;
pub mod regulation;
pub mod sensors;

pub use actions::{apply_action, choose_action, Action};
pub use creature::{Creature, Experience, MAX_RECENT_EXPERIENCE, SleepState};
pub use genome::Genome;
pub use lifecycle::{
    deposit_creature_organic, try_reproduce, BirthEvent, DeathCause, DeathEvent,
    DEFAULT_MAX_POPULATION, REPRODUCTION_CHANCE_PER_TICK, REPRODUCTION_ENERGY_COST,
    REPRODUCTION_ENERGY_THRESHOLD,
};
pub use regulation::RegulatoryState;
pub use sensors::{read_sensors, read_sensors_with_noise, SensorState};
