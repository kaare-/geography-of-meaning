use rand::Rng;
use serde::{Deserialize, Serialize};

use super::genome::Genome;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Morphology {
    pub mass: f32,
    pub reserve_capacity: f32,
    pub heat_retention: f32,
    pub carry_capacity: f32,
}

impl Morphology {
    /// Displacement strength derived from body mass.
    pub fn push_strength(&self) -> f32 {
        self.mass * 0.85
    }

    /// Metabolism scales with mass — larger bodies cost more per tick.
    pub fn metabolism_multiplier(&self) -> f32 {
        (0.6 + self.mass * 0.5).clamp(0.5, 2.5)
    }

    /// Thermal coupling: high mass + heat retention buffers environmental swings.
    pub fn thermal_coupling(&self) -> f32 {
        (1.0 / (1.0 + self.mass * self.heat_retention * 0.6)).clamp(0.15, 1.0)
    }

    pub fn from_genome(genome: &Genome) -> Self {
        let mass = (0.45 + genome.mass_bias * 0.55).clamp(0.25, 2.5);
        let reserve_capacity = (0.35 + mass * 0.22 + genome.reserve_bias * 0.15).clamp(0.3, 1.0);
        let heat_retention = genome.heat_retention.clamp(0.05, 1.0);
        let carry_capacity =
            (0.12 + genome.carry_bias * 0.18 * mass.sqrt()).clamp(0.08, 0.85);
        Self {
            mass,
            reserve_capacity,
            heat_retention,
            carry_capacity,
        }
    }

    pub fn mutate_from<R: Rng + ?Sized>(parent: &Morphology, genome: &Genome, rng: &mut R) -> Self {
        let mut m = Self::from_genome(genome);
        m.mass = (parent.mass + rng.gen_range(-0.08..0.08) + (m.mass - parent.mass) * 0.3)
            .clamp(0.25, 2.5);
        m.reserve_capacity = (parent.reserve_capacity
            + rng.gen_range(-0.05..0.05)
            + (m.reserve_capacity - parent.reserve_capacity) * 0.3)
            .clamp(0.3, 1.0);
        m.heat_retention = (parent.heat_retention
            + rng.gen_range(-0.04..0.04)
            + (m.heat_retention - parent.heat_retention) * 0.3)
            .clamp(0.05, 1.0);
        m.carry_capacity = (parent.carry_capacity
            + rng.gen_range(-0.03..0.03)
            + (m.carry_capacity - parent.carry_capacity) * 0.3)
            .clamp(0.08, 0.85);
        m
    }
}
