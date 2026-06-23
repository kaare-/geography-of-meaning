pub mod actions;
pub mod creature;
pub mod genome;
pub mod lifecycle;
pub mod morphology;
pub mod regulation;
pub mod sensors;
pub mod spatial;

pub use actions::{apply_action, choose_action, Action};
pub use morphology::Morphology;
pub use spatial::{
    compute_follow_direction, push_strength, resolve_position_overlaps, try_creature_move_at,
    try_creature_push_at, PushEvent, FOLLOW_ENERGY_COST, FOLLOW_FATIGUE_COST,
};
pub use creature::{Creature, Experience, MAX_RECENT_EXPERIENCE, SleepState};
pub use genome::Genome;
pub use lifecycle::{
    deposit_creature_organic, try_reproduce, BirthEvent, DeathCause, DeathEvent,
    DEFAULT_MAX_POPULATION, REPRODUCTION_CHANCE_PER_TICK, REPRODUCTION_ENERGY_COST,
    REPRODUCTION_ENERGY_THRESHOLD,
};
pub use regulation::RegulatoryState;
pub use sensors::{
    dominant_heard_signature, read_sensors, read_sensors_with_noise, SensorState,
};
