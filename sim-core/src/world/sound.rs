use serde::{Deserialize, Serialize};

use crate::math::Vec3f;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEvent {
    pub position: Vec3f,
    pub emitter_id: u64,
    pub signature: u64,
    pub amplitude: f32,
    pub frequency_profile: f32,
    pub duration: u32,
    pub age: u32,
}

impl SoundEvent {
    pub fn new(
        position: Vec3f,
        emitter_id: u64,
        signature: u64,
        amplitude: f32,
        frequency_profile: f32,
        duration: u32,
    ) -> Self {
        Self {
            position,
            emitter_id,
            signature,
            amplitude,
            frequency_profile,
            duration,
            age: 0,
        }
    }

    pub fn is_active(&self) -> bool {
        self.age < self.duration
    }

    pub fn attenuated_at(&self, listener_pos: Vec3f) -> f32 {
        let dx = self.position.x - listener_pos.x;
        let dy = self.position.y - listener_pos.y;
        let dz = self.position.z - listener_pos.z;
        let dist_sq = dx * dx + dy * dy + dz * dz;
        let age_fade = 1.0 - (self.age as f32 / self.duration.max(1) as f32);
        self.amplitude * age_fade / (1.0 + dist_sq)
    }
}
