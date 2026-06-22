use serde::Serialize;

use crate::creatures::DeathEvent;
use crate::creatures::lifecycle::BirthEvent;
use crate::export::snapshots::CreatureSnapshot;
use crate::world::SoundEvent;

#[derive(Debug, Serialize)]
pub struct SoundEventSnapshot {
    pub emitter_id: u64,
    pub signature: u64,
    pub amplitude: f32,
    pub frequency_profile: f32,
    pub age: u32,
    pub duration: u32,
}

impl SoundEventSnapshot {
    pub fn from_event(event: &SoundEvent) -> Self {
        Self {
            emitter_id: event.emitter_id,
            signature: event.signature,
            amplitude: event.amplitude,
            frequency_profile: event.frequency_profile,
            age: event.age,
            duration: event.duration,
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct ActionCounts {
    pub move_count: u32,
    pub dig_count: u32,
    pub carry_count: u32,
    pub drop_count: u32,
}

#[derive(Debug, Serialize)]
pub struct TickLogEntry {
    pub tick: u64,
    pub day_phase: f32,
    pub rain_applied: f32,
    pub sound_event_count: usize,
    pub sound_events: Vec<SoundEventSnapshot>,
    pub deaths: Vec<DeathEvent>,
    pub births: Vec<BirthEvent>,
    pub concepts_formed: u32,
    pub action_counts: ActionCounts,
    pub creatures: Vec<CreatureSnapshot>,
}
