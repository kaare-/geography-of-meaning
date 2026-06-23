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
    pub energy_depleted_ticks: u8,
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
            energy_depleted_ticks: 0,
        }
    }
}

impl RegulatoryState {
    pub fn tick_passive_drain(&mut self, metabolism: f32, mass_multiplier: f32) {
        let drain = metabolism * mass_multiplier;
        self.energy = (self.energy - drain).max(0.0);
        self.hydration = (self.hydration - drain * 0.35).max(0.0);
        self.fatigue = (self.fatigue + drain * 0.3).min(1.0);
    }

    pub fn apply_environmental_stress(&mut self, sensor: &SensorState, heat_retention: f32, mass: f32) {
        let coupling = (1.0 / (1.0 + mass * heat_retention * 0.6)).clamp(0.15, 1.0);
        self.temperature_stress = sensor.internal_temperature_stress * coupling;
        if self.temperature_stress > 0.88 {
            self.integrity =
                (self.integrity - (self.temperature_stress - 0.88) * 0.005).max(0.0);
        }
        if self.fatigue > 0.92 {
            self.integrity = (self.integrity - 0.0005).max(0.0);
        }
    }

    pub fn apply_passive_hydration(&mut self, sensor: &SensorState) {
        if sensor.chemical_wet_mineral > 0.2 {
            let gain = (sensor.chemical_wet_mineral - 0.2) * 0.05;
            self.hydration = (self.hydration + gain).min(1.0);
        }
    }

    /// Low hydration increases fatigue, drains energy, and erodes integrity.
    pub fn apply_dehydration_stress(&mut self) {
        if self.hydration >= 0.25 {
            return;
        }
        let deficit = 0.25 - self.hydration;
        self.energy = (self.energy - deficit * 0.01).max(0.0);
        self.integrity = (self.integrity - deficit * 0.003).max(0.0);
        self.fatigue = (self.fatigue + deficit * 0.2).min(1.0);
    }

    pub fn apply_reproduction_cost(&mut self, cost: f32) {
        self.energy = (self.energy - cost).max(0.0);
    }

    pub fn apply_ambient_processing_cost(&mut self, sound_ambient: f32) {
        if sound_ambient > 0.25 {
            let excess = sound_ambient - 0.25;
            self.energy = (self.energy - excess * 0.0012).max(0.0);
        }
    }

    pub fn apply_action_cost(&mut self, energy_cost: f32, fatigue_cost: f32) {
        self.energy = (self.energy - energy_cost).max(0.0);
        self.fatigue = (self.fatigue + fatigue_cost).min(1.0);
    }

    pub fn clamp(&mut self, energy_cap: f32) {
        self.energy = self.energy.clamp(0.0, energy_cap);
        self.hydration = self.hydration.clamp(0.0, 1.0);
        self.temperature_stress = self.temperature_stress.clamp(0.0, 1.0);
        self.integrity = self.integrity.clamp(0.0, 1.0);
        self.fatigue = self.fatigue.clamp(0.0, 1.0);
        self.carried_mass = self.carried_mass.max(0.0);
    }
}
