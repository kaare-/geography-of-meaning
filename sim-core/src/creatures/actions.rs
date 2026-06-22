use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::math::{Vec3f, Vec3i};
use crate::world::{SoundEvent, World};

use super::creature::Creature;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Move(Vec3i),
    ConsumeOrganic,
    Rest,
    EmitSound,
}

impl Action {
    pub fn label(&self) -> &'static str {
        match self {
            Action::Move(_) => "move",
            Action::ConsumeOrganic => "consume_organic",
            Action::Rest => "rest",
            Action::EmitSound => "emit_sound",
        }
    }
}

const EXPLORATION_RATE: f32 = 0.15;
const PREDICTION_WEIGHT: f32 = 1.5;
const EMIT_SOUND_BASE_WEIGHT: f32 = 0.15;
const EMIT_SOUND_ENERGY_BOOST: f32 = 2.5;
const EMIT_SOUND_ENERGY_COST: f32 = 0.08;

pub fn choose_action<R: Rng + ?Sized>(creature: &Creature, rng: &mut R, sleeping: bool) -> Action {
    let mut weights = if sleeping {
        vec![
            (Action::ConsumeOrganic, 1.0f32),
            (Action::Rest, 2.0),
            (Action::EmitSound, 0.05),
        ]
    } else {
        vec![
            (
                Action::Move(Vec3i::new(
                    rng.gen_range(-1..=1),
                    rng.gen_range(-1..=1),
                    rng.gen_range(-1..=1),
                )),
                1.0f32,
            ),
            (Action::ConsumeOrganic, 1.0),
            (Action::Rest, 1.0),
            (Action::EmitSound, EMIT_SOUND_BASE_WEIGHT),
        ]
    };

    if creature.regulatory.energy > 0.7 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::EmitSound)) {
            weights[i].1 += EMIT_SOUND_ENERGY_BOOST;
        }
    }

    if creature.regulatory.energy < 0.4 {
        let consume_idx = weights.iter().position(|(a, _)| matches!(a, Action::ConsumeOrganic));
        let rest_idx = weights.iter().position(|(a, _)| matches!(a, Action::Rest));
        if let Some(i) = consume_idx {
            weights[i].1 += 2.0;
        }
        if let Some(i) = rest_idx {
            weights[i].1 += 1.5;
        }
    }
    if creature.regulatory.fatigue > 0.6 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Rest)) {
            weights[i].1 += 2.0;
        }
    }

    if !sleeping && rng.gen::<f32>() >= EXPLORATION_RATE {
        let predictions = creature
            .memory_graph
            .predict_action_outcomes(creature.sensor);
        for (action, weight) in &mut weights {
            let predicted = match action {
                Action::Move(_) => predictions.move_delta,
                Action::ConsumeOrganic => predictions.consume_delta,
                Action::Rest => predictions.rest_delta,
                Action::EmitSound => predictions.emit_sound_delta,
            };
            if predicted > 0.0 {
                *weight += predicted * PREDICTION_WEIGHT;
            } else if predicted < 0.0 {
                *weight = (*weight + predicted * PREDICTION_WEIGHT).max(0.1);
            }
        }
    }

    let total: f32 = weights.iter().map(|(_, w)| w).sum();
    let mut roll = rng.gen_range(0.0..total);
    for (action, weight) in &weights {
        roll -= weight;
        if roll <= 0.0 {
            return *action;
        }
    }
    Action::Rest
}

pub fn apply_action(creature: &mut Creature, action: Action, world: &mut World) -> bool {
    match action {
        Action::Move(delta) => {
            let target = creature.position.floor_i();
            let speed = creature.genome.move_speed;
            let new_pos = Vec3i::new(
                target.x + delta.x,
                target.y + delta.y,
                target.z + delta.z,
            );
            if let Some(voxel) = world.sample_voxel(new_pos) {
                if voxel.void_fraction > 0.4 {
                    creature.position = Vec3f::from_vec3i(new_pos);
                    creature.regulatory.apply_action_cost(0.05 * speed, 0.1);
                    return true;
                }
            }
            false
        }
        Action::ConsumeOrganic => {
            let pos = creature.position.floor_i();
            let wet_trace = creature.sensor.chemical_wet_mineral;
            let mut consumed = false;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let check = Vec3i::new(pos.x + dx, pos.y + dy, pos.z + dz);
                        if let Some(voxel) = world.sample_voxel_mut(check) {
                            if *voxel.organic > 0.05 {
                                let transfer = (*voxel.organic * 0.2).min(0.1);
                                *voxel.organic -= transfer;
                                creature.regulatory.energy =
                                    (creature.regulatory.energy + transfer * 2.0).min(1.0);
                                creature.regulatory.apply_action_cost(0.02, 0.05);
                                consumed = true;
                            }
                        }
                    }
                }
            }
            if wet_trace > 0.15 {
                creature.regulatory.hydration =
                    (creature.regulatory.hydration + wet_trace * 0.04).min(1.0);
            }
            consumed
        }
        Action::Rest => {
            creature.regulatory.fatigue = (creature.regulatory.fatigue - 0.15).max(0.0);
            creature.regulatory.energy = (creature.regulatory.energy + 0.05).min(1.0);
            if creature.sensor.chemical_wet_mineral > 0.15 {
                creature.regulatory.hydration = (creature.regulatory.hydration
                    + creature.sensor.chemical_wet_mineral * 0.03)
                    .min(1.0);
            }
            creature.regulatory.apply_action_cost(0.01, -0.1);
            true
        }
        Action::EmitSound => {
            let amplitude = (creature.regulatory.energy * 0.6 + 0.2).min(1.0);
            let frequency_profile = (creature.signature as f32 / u64::MAX as f32).fract();
            world.emit_sound(SoundEvent::new(
                creature.position,
                creature.id,
                creature.signature,
                amplitude,
                frequency_profile,
                8,
            ));
            creature.regulatory.apply_action_cost(EMIT_SOUND_ENERGY_COST, 0.05);
            true
        }
    }
}
