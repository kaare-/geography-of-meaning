use serde::{Deserialize, Serialize};

use crate::creatures::genome::VocalProfile;
use crate::math::Vec3f;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEvent {
    pub position: Vec3f,
    pub emitter_id: u64,
    pub signature: u64,
    pub amplitude: f32,
    pub frequency_profile: f32,
    pub duration: u32,
    pub rhythm: f32,
    pub signal_family_id: u64,
    pub age: u32,
}

pub fn signal_family_id(profile: &VocalProfile) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &v in profile {
        hash = hash.wrapping_mul(0x100000001b3).wrapping_add(v.to_bits() as u64);
    }
    hash
}

impl SoundEvent {
    pub fn from_vocal_profile(
        position: Vec3f,
        emitter_id: u64,
        signature: u64,
        vocal_profile: &VocalProfile,
        energy_scale: f32,
    ) -> Self {
        let pitch = vocal_profile[0];
        let duration = vocal_profile[1].round().clamp(4.0, 16.0) as u32;
        let amplitude = (vocal_profile[2] * energy_scale).clamp(0.1, 1.0);
        let rhythm = vocal_profile[3];
        let signal_family_id = signal_family_id(vocal_profile);
        Self {
            position,
            emitter_id,
            signature,
            amplitude,
            frequency_profile: pitch,
            duration,
            rhythm,
            signal_family_id,
            age: 0,
        }
    }

    pub fn new(
        position: Vec3f,
        emitter_id: u64,
        signature: u64,
        amplitude: f32,
        frequency_profile: f32,
        duration: u32,
    ) -> Self {
        let profile = [frequency_profile, duration as f32, amplitude, 0.5];
        Self::from_vocal_profile(position, emitter_id, signature, &profile, 1.0)
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
