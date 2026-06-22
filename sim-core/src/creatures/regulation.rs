use serde::{Deserialize, Serialize};

use super::sensors::SensorState;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct RegulatoryState {
    pub energy: f32,
    pub hydration: f32,
    pub temperature_stress: f32,
    pub integrity: f32,
    pub fatigue: f32,
    pub carried_mass: f32,
}

impl Default for RegulatoryState {
    fn default() -> Self {
        Self {
            energy: 0.8,
            hydration: 0.7,
            temperature_stress: 0.0,
            integrity: 1.0,
            fatigue: 0.0,
            carried_mass: 0.0,
        }
    }
}

impl RegulatoryState {
    pub fn tick_passive_drain(&mut self, metabolism: f32) {
        self.energy = (self.energy - metabolism).max(0.0);
        self.hydration = (self.hydration - metabolism * 0.35).max(0.0);
        self.fatigue = (self.fatigue + metabolism * 0.3).min(1.0);
    }

    pub fn apply_environmental_stress(&mut self, sensor: &SensorState) {
        self.temperature_stress = sensor.internal_temperature_stress;
        if self.temperature_stress > 0.85 {
            self.integrity =
                (self.integrity - (self.temperature_stress - 0.85) * 0.01).max(0.0);
        }
        if self.fatigue > 0.9 {
            self.integrity = (self.integrity - 0.002).max(0.0);
        }
    }

    pub fn apply_passive_hydration(&mut self, sensor: &SensorState) {
        if sensor.chemical_wet_mineral > 0.2 {
            let gain = (sensor.chemical_wet_mineral - 0.2) * 0.05;
            self.hydration = (self.hydration + gain).min(1.0);
        }
    }

    pub fn apply_reproduction_cost(&mut self, cost: f32) {
        self.energy = (self.energy - cost).max(0.0);
    }

    pub fn apply_action_cost(&mut self, energy_cost: f32, fatigue_cost: f32) {
        self.energy = (self.energy - energy_cost).max(0.0);
        self.fatigue = (self.fatigue + fatigue_cost).min(1.0);
    }

    pub fn clamp(&mut self) {
        self.energy = self.energy.clamp(0.0, 1.0);
        self.hydration = self.hydration.clamp(0.0, 1.0);
        self.temperature_stress = self.temperature_stress.clamp(0.0, 1.0);
        self.integrity = self.integrity.clamp(0.0, 1.0);
        self.fatigue = self.fatigue.clamp(0.0, 1.0);
        self.carried_mass = self.carried_mass.max(0.0);
    }
}
