use rand::Rng;
use serde::{Deserialize, Serialize};

/// Vocal production bias: pitch, duration (ticks), amplitude, rhythm.
pub type VocalProfile = [f32; 4];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Genome {
    pub metabolism_rate: f32,
    pub sensor_noise_scale: f32,
    pub move_speed: f32,
    pub vocal_profile: VocalProfile,
}

impl Default for Genome {
    fn default() -> Self {
        Self {
            metabolism_rate: 0.008,
            sensor_noise_scale: 0.05,
            move_speed: 1.0,
            vocal_profile: [0.5, 8.0, 0.5, 0.5],
        }
    }
}

impl Genome {
    pub fn mutate_from<R: Rng + ?Sized>(parent: &Genome, rng: &mut R) -> Self {
        let mut vocal_profile = parent.vocal_profile;
        for v in &mut vocal_profile {
            *v = (*v + rng.gen_range(-0.05..0.05)).clamp(0.05, 1.0);
        }
        vocal_profile[1] = (vocal_profile[1] + rng.gen_range(-1.0..1.0)).clamp(4.0, 16.0);

        Self {
            metabolism_rate: (parent.metabolism_rate + rng.gen_range(-0.002..0.002)).clamp(0.004, 0.025),
            sensor_noise_scale: (parent.sensor_noise_scale + rng.gen_range(-0.01..0.01)).clamp(0.01, 0.15),
            move_speed: (parent.move_speed + rng.gen_range(-0.1..0.1)).clamp(0.5, 2.0),
            vocal_profile,
        }
    }
}
