use serde::{Deserialize, Serialize};

use crate::creatures::genome::VocalProfile;
use crate::math::{Vec3f, Vec3i};
use crate::world::World;

#[derive(Debug, Clone, Copy)]
pub enum EnvironmentalSoundKind {
    Rain,
    Collapse,
    WaterFlow,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ActionSoundKind {
    Move,
    Dig,
    Carry,
    Drop,
    Push,
    ConsumeOrganic,
    PlaceMaterial,
}

#[derive(Debug, Clone, Copy)]
pub struct MaterialAcousticProfile {
    pub frequency_profile: f32,
    pub amplitude_scale: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct SoundEmitterContext {
    pub emitter_id: u64,
    pub signature: u64,
    pub age: u32,
    pub mass: f32,
    pub move_speed: f32,
    pub carried_mass: f32,
}

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
    pub intentional: bool,
}

pub fn signal_family_id(profile: &VocalProfile) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325u64;
    for &v in profile {
        hash = hash.wrapping_mul(0x100000001b3).wrapping_add(v.to_bits() as u64);
    }
    hash
}

/// Subtle age-band mixing into emitter signature for listener discrimination.
pub fn signature_with_age_band(base: u64, age: u32) -> u64 {
    let band = if age < 100 {
        1u64
    } else if age > 500 {
        3u64
    } else {
        2u64
    };
    base.wrapping_add(band.wrapping_mul(0x517c_c1b7_2722_0a95))
}

/// Age-dependent pitch/duration/rhythm without semantic life-stage labels.
pub fn age_adjusted_vocal_profile(base: &VocalProfile, age: u32) -> VocalProfile {
    let mut p = *base;
    if age < 100 {
        p[0] = (p[0] + 0.15).min(1.0);
        p[1] = (p[1] - 2.0).max(4.0);
    } else if age > 500 {
        p[0] = (p[0] - 0.12).max(0.05);
        p[3] = (p[3] - 0.1).max(0.05);
    }
    p
}

pub fn sample_material_acoustics(world: &World, pos: Vec3i) -> MaterialAcousticProfile {
    let Some(v) = world.sample_voxel(pos) else {
        return MaterialAcousticProfile {
            frequency_profile: 0.5,
            amplitude_scale: 1.0,
        };
    };
    let hard = v.hard_mineral * v.solid_fraction;
    let soft = (v.soft_mineral + v.clay) * v.solid_fraction;
    let wet = v.water_content + v.surface_water;
    let organic = v.organic;
    let void_f = v.void_fraction;

    let frequency_profile =
        (0.35 + hard * 0.5 - soft * 0.25 + organic * 0.1).clamp(0.05, 1.0);
    let amplitude_scale =
        ((1.0 - wet * 0.4) * (0.85 + void_f * 0.3) * (1.0 + organic * 0.15)).clamp(0.3, 1.5);

    MaterialAcousticProfile {
        frequency_profile,
        amplitude_scale,
    }
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
            intentional: true,
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

fn action_duration(kind: ActionSoundKind) -> u32 {
    match kind {
        ActionSoundKind::Move => 3,
        ActionSoundKind::Dig => 6,
        ActionSoundKind::Carry => 4,
        ActionSoundKind::Drop => 5,
        ActionSoundKind::Push => 5,
        ActionSoundKind::ConsumeOrganic => 4,
        ActionSoundKind::PlaceMaterial => 5,
    }
}

fn action_rhythm_base(kind: ActionSoundKind) -> f32 {
    match kind {
        ActionSoundKind::Move => 0.45,
        ActionSoundKind::Dig => 0.7,
        ActionSoundKind::Carry => 0.35,
        ActionSoundKind::Drop => 0.55,
        ActionSoundKind::Push => 0.6,
        ActionSoundKind::ConsumeOrganic => 0.4,
        ActionSoundKind::PlaceMaterial => 0.5,
    }
}

/// Low-amplitude side-effect sound from physical work (not labeled as message).
pub fn emit_incidental_sound(
    world: &mut World,
    position: Vec3f,
    emitter: SoundEmitterContext,
    action_kind: ActionSoundKind,
    amplitude_base: f32,
    material: MaterialAcousticProfile,
) {
    let mass_amp = (0.4 + emitter.mass * 0.25 + emitter.carried_mass * 0.35).clamp(0.3, 2.0);
    let rhythm = (action_rhythm_base(action_kind) * emitter.move_speed * 0.85).clamp(0.15, 1.2);
    let amplitude = (amplitude_base * material.amplitude_scale * mass_amp).clamp(0.02, 0.18);
    let duration = action_duration(action_kind);
    let signature = signature_with_age_band(emitter.signature, emitter.age);
    let profile = [material.frequency_profile, duration as f32, amplitude, rhythm];

    world.emit_sound(SoundEvent {
        position,
        emitter_id: emitter.emitter_id,
        signature,
        amplitude,
        frequency_profile: material.frequency_profile,
        duration,
        rhythm,
        signal_family_id: signal_family_id(&profile),
        age: 0,
        intentional: false,
    });
}

/// Weak ambient trace from rain, collapse, or water — not creature-emitted.
pub fn emit_environmental_sound(
    world: &mut World,
    position: Vec3f,
    kind: EnvironmentalSoundKind,
    intensity: f32,
) {
    let (frequency_profile, duration, rhythm, amp_scale) = match kind {
        EnvironmentalSoundKind::Rain => (0.22, 10, 0.18, 0.035),
        EnvironmentalSoundKind::Collapse => (0.58, 8, 0.4, 0.05),
        EnvironmentalSoundKind::WaterFlow => (0.32, 14, 0.28, 0.04),
    };
    let amplitude = (intensity * amp_scale).clamp(0.01, 0.1);
    let profile = [frequency_profile, duration as f32, amplitude, rhythm];
    world.emit_sound(SoundEvent {
        position,
        emitter_id: 0,
        signature: 0,
        amplitude,
        frequency_profile,
        duration,
        rhythm,
        signal_family_id: signal_family_id(&profile),
        age: 0,
        intentional: false,
    });
}
