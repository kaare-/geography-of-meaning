use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::math::Vec3i;
use crate::world::{
    age_adjusted_vocal_profile, emit_incidental_sound, sample_material_acoustics,
    signature_with_age_band, ActionSoundKind, SoundEvent, World,
};

use super::creature::Creature;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Action {
    Move(Vec3i),
    Push(Vec3i),
    ConsumeOrganic,
    Rest,
    EmitSound,
    Follow,
    Dig,
    Carry,
    Drop,
    PlaceMaterial,
    ApplyBinder,
    TransferOrganic,
}

impl Action {
    pub fn label(&self) -> &'static str {
        match self {
            Action::Move(_) => "move",
            Action::Push(_) => "push",
            Action::ConsumeOrganic => "consume_organic",
            Action::Rest => "rest",
            Action::EmitSound => "emit_sound",
            Action::Follow => "follow",
            Action::Dig => "dig",
            Action::Carry => "carry",
            Action::Drop => "drop",
            Action::PlaceMaterial => "place_material",
            Action::ApplyBinder => "apply_binder",
            Action::TransferOrganic => "transfer_organic",
        }
    }
}

const FOLLOW_BASE_WEIGHT: f32 = 0.3;
const EXPLORATION_RATE: f32 = 0.15;
const PREDICTION_WEIGHT: f32 = 1.5;
const EMIT_SOUND_BASE_WEIGHT: f32 = 0.15;
const EMIT_SOUND_ENERGY_BOOST: f32 = 2.5;
const EMIT_SOUND_ENERGY_COST: f32 = 0.08;
const DIG_ENERGY_COST: f32 = 0.09;
const DIG_FATIGUE_COST: f32 = 0.15;
const CARRY_ENERGY_COST: f32 = 0.04;
const CARRY_FATIGUE_COST: f32 = 0.08;
const DROP_ENERGY_COST: f32 = 0.03;
const DROP_FATIGUE_COST: f32 = 0.05;
const PLACE_MATERIAL_ENERGY_COST: f32 = 0.05;
const PLACE_MATERIAL_FATIGUE_COST: f32 = 0.1;
const APPLY_BINDER_ENERGY_COST: f32 = 0.06;
const APPLY_BINDER_FATIGUE_COST: f32 = 0.12;
const BINDER_ORGANIC_COST: f32 = 0.04;
const TRANSFER_ORGANIC_BASE_WEIGHT: f32 = 0.12;

const UNCERTAINTY_EXPLORATION_BOOST: f32 = 0.12;

fn effective_exploration_rate(creature: &Creature) -> f32 {
    let uncertainty = creature.memory_graph.prediction_uncertainty(
        creature.sensor,
        &creature.active_concepts,
        &creature.concepts,
    );
    (EXPLORATION_RATE + uncertainty * UNCERTAINTY_EXPLORATION_BOOST).clamp(0.05, 0.45)
}

pub fn choose_action<R: Rng + ?Sized>(
    creature: &Creature,
    rng: &mut R,
    sleeping: bool,
    heard_signature: Option<u64>,
    heard_call_frequency: Option<f32>,
    prediction_ms: Option<&mut f64>,
) -> Action {
    let push_dir = Vec3i::new(
        rng.gen_range(-1..=1),
        rng.gen_range(-1..=1),
        rng.gen_range(-1..=1),
    );
    let move_dir = Vec3i::new(
        rng.gen_range(-1..=1),
        rng.gen_range(-1..=1),
        rng.gen_range(-1..=1),
    );

    let mut weights = if sleeping {
        vec![
            (Action::ConsumeOrganic, 1.0f32),
            (Action::Rest, 2.0),
            (Action::EmitSound, 0.05),
            (Action::Dig, 0.2),
            (Action::Carry, 0.3),
            (Action::Drop, 0.4),
            (Action::PlaceMaterial, 0.15),
            (Action::ApplyBinder, 0.1),
            (Action::TransferOrganic, 0.08),
        ]
    } else {
        vec![
            (Action::Move(move_dir), 1.0f32),
            (Action::Push(push_dir), 0.25),
            (Action::ConsumeOrganic, 1.0),
            (Action::Rest, 1.0),
            (Action::EmitSound, EMIT_SOUND_BASE_WEIGHT),
            (Action::Follow, FOLLOW_BASE_WEIGHT),
            (Action::Dig, 0.5),
            (Action::Carry, 0.6),
            (Action::Drop, 0.4),
            (Action::PlaceMaterial, 0.35),
            (Action::ApplyBinder, 0.3),
            (Action::TransferOrganic, TRANSFER_ORGANIC_BASE_WEIGHT),
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

    if creature.sensor.chemical_creature > 0.05 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Follow)) {
            weights[i].1 += creature.sensor.chemical_creature * 2.0;
        }
    }
    if creature.sensor.sound_calls > 0.08 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Follow)) {
            weights[i].1 += creature.sensor.sound_calls * 1.5;
            weights[i].1 += creature
                .memory_graph
                .trusted_follow_boost(creature.sensor.sound_calls, heard_signature);
            weights[i].1 += creature.memory_graph.developmental_follow_boost(
                creature.genome.developmental_bias,
                creature.sensor.sound_calls,
                heard_signature,
                heard_call_frequency,
            );
        }
    }

    if creature.sensor.contact_hard > 0.3 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Dig)) {
            weights[i].1 += creature.sensor.contact_hard * 1.5;
        }
    }
    if creature.regulatory.carried_mass < creature.morphology.carry_capacity
        && creature.sensor.chemical_organic > 0.08
    {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Carry)) {
            weights[i].1 += creature.sensor.chemical_organic * 2.0;
        }
    }
    if creature.regulatory.carried_mass > 0.15 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Drop)) {
            weights[i].1 += 1.5 + creature.regulatory.carried_mass;
        }
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::PlaceMaterial)) {
            weights[i].1 += creature.regulatory.carried_mass;
        }
    }
    if creature.sensor.contact_occupied > 0.3 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Push(_))) {
            weights[i].1 += creature.sensor.contact_occupied * 2.0;
        }
    }
    if creature.sensor.chemical_binder > 0.05 || creature.regulatory.carried_mass > 0.1 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::ApplyBinder)) {
            weights[i].1 += creature.sensor.chemical_binder + 0.3;
        }
    }
    if creature.regulatory.carried_mass > 0.1 && creature.sensor.chemical_creature > 0.05 {
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::TransferOrganic)) {
            weights[i].1 += creature.regulatory.carried_mass * 0.5
                + creature.sensor.chemical_creature * 1.0;
        }
    }

    if !sleeping {
        let novelty = creature.memory_graph.novelty_score(creature.sensor);
        let uncertainty = creature.memory_graph.prediction_uncertainty(
            creature.sensor,
            &creature.active_concepts,
            &creature.concepts,
        );
        if let Some(i) = weights.iter().position(|(a, _)| matches!(a, Action::Move(_))) {
            weights[i].1 += novelty * 1.5;
            weights[i].1 *= 1.0 / creature.genome.move_speed.max(0.5);
        }
        let explore_boost = uncertainty * novelty * 0.8;
        for (action, weight) in &mut weights {
            if matches!(action, Action::Move(_)) || matches!(action, Action::Dig) {
                *weight += explore_boost;
            }
        }
    }

    if !sleeping && rng.gen::<f32>() >= effective_exploration_rate(creature) {
        let prediction_start = std::time::Instant::now();
        let predictions = creature.memory_graph.predict_action_outcomes(
            creature.sensor,
            &creature.active_concepts,
            &creature.concepts,
        );
        if let Some(ms) = prediction_ms {
            *ms += prediction_start.elapsed().as_secs_f64() * 1000.0;
        }
        for (action, weight) in &mut weights {
            let predicted = match action {
                Action::Move(_) => predictions.move_delta,
                Action::Push(_) => predictions.push_delta,
                Action::ConsumeOrganic => predictions.consume_delta,
                Action::Rest => predictions.rest_delta,
                Action::EmitSound => predictions.emit_sound_delta,
                Action::Follow => predictions.follow_delta,
                Action::Dig => predictions.dig_delta,
                Action::Carry => predictions.carry_delta,
                Action::Drop => predictions.drop_delta,
                Action::PlaceMaterial => predictions.place_material_delta,
                Action::ApplyBinder => predictions.apply_binder_delta,
                Action::TransferOrganic => predictions.transfer_organic_delta,
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
    let emitter = creature.sound_emitter_context();
    let material_pos = creature.position.floor_i();
    let material = sample_material_acoustics(world, material_pos);

    let emit = |world: &mut World, kind: ActionSoundKind, amp: f32| {
        emit_incidental_sound(
            world,
            creature.position,
            emitter,
            kind,
            amp,
            material,
        );
    };

    match action {
        Action::Move(_) | Action::Push(_) | Action::Follow | Action::TransferOrganic => false,
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
                                let transfer = (*voxel.organic * 0.25).min(0.12);
                                *voxel.organic -= transfer;
                                creature.regulatory.energy =
                                    (creature.regulatory.energy + transfer * 2.7).min(1.0);
                                creature.regulatory.integrity =
                                    (creature.regulatory.integrity + 0.012).min(1.0);
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
            if consumed {
                emit(world, ActionSoundKind::ConsumeOrganic, 0.07);
            }
            consumed
        }
        Action::Rest => {
            creature.regulatory.fatigue = (creature.regulatory.fatigue - 0.15).max(0.0);
            creature.regulatory.energy = (creature.regulatory.energy + 0.07).min(1.0);
            creature.regulatory.integrity = (creature.regulatory.integrity + 0.018).min(1.0);
            if creature.sensor.chemical_wet_mineral > 0.15 {
                creature.regulatory.hydration = (creature.regulatory.hydration
                    + creature.sensor.chemical_wet_mineral * 0.03)
                    .min(1.0);
            }
            creature.regulatory.apply_action_cost(0.01, -0.1);
            true
        }
        Action::EmitSound => {
            let energy_scale = (creature.regulatory.energy * 0.6 + 0.2).min(1.0);
            let profile =
                age_adjusted_vocal_profile(&creature.genome.vocal_profile, creature.age);
            world.emit_sound(SoundEvent::from_vocal_profile(
                creature.position,
                creature.id,
                signature_with_age_band(creature.signature, creature.age),
                &profile,
                energy_scale,
            ));
            creature.regulatory.apply_action_cost(EMIT_SOUND_ENERGY_COST, 0.05);
            true
        }
        Action::Dig => {
            let pos = creature.position.floor_i();
            if let Some(voxel) = world.sample_voxel_mut(pos) {
                if *voxel.solid_fraction < 0.15 {
                    return false;
                }
                let solid_remove = (*voxel.solid_fraction * 0.08).min(0.05);
                *voxel.solid_fraction -= solid_remove;
                *voxel.void_fraction = (*voxel.void_fraction + solid_remove).min(1.0);

                let organic_loose = (*voxel.organic * 0.15).min(0.03);
                *voxel.organic -= organic_loose;

                if creature.regulatory.carried_mass < creature.morphology.carry_capacity {
                    let room = creature.morphology.carry_capacity - creature.regulatory.carried_mass;
                    creature.regulatory.carried_mass += organic_loose.min(room);
                }

                creature.regulatory.apply_action_cost(DIG_ENERGY_COST, DIG_FATIGUE_COST);
                emit(world, ActionSoundKind::Dig, 0.12);
                true
            } else {
                false
            }
        }
        Action::Carry => {
            if creature.regulatory.carried_mass >= creature.morphology.carry_capacity {
                return false;
            }
            let pos = creature.position.floor_i();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        let check = Vec3i::new(pos.x + dx, pos.y + dy, pos.z + dz);
                        if let Some(voxel) = world.sample_voxel_mut(check) {
                            if *voxel.organic > 0.05 {
                                let room = creature.morphology.carry_capacity - creature.regulatory.carried_mass;
                                let transfer = (*voxel.organic * 0.25).min(0.05).min(room);
                                *voxel.organic -= transfer;
                                creature.regulatory.carried_mass += transfer;
                                creature.regulatory.apply_action_cost(
                                    CARRY_ENERGY_COST,
                                    CARRY_FATIGUE_COST,
                                );
                                emit(world, ActionSoundKind::Carry, 0.06);
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Action::Drop => {
            if creature.regulatory.carried_mass < 0.01 {
                return false;
            }
            let pos = creature.position.floor_i();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        let check = Vec3i::new(pos.x + dx, pos.y + dy, pos.z + dz);
                        if let Some(voxel) = world.sample_voxel_mut(check) {
                            if *voxel.void_fraction > 0.4 {
                                let deposit = creature.regulatory.carried_mass.min(0.08);
                                *voxel.organic += deposit;
                                creature.regulatory.carried_mass -= deposit;
                                creature.regulatory.apply_action_cost(
                                    DROP_ENERGY_COST,
                                    DROP_FATIGUE_COST,
                                );
                                emit(world, ActionSoundKind::Drop, 0.1);
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Action::PlaceMaterial => {
            if creature.regulatory.carried_mass < 0.02 {
                return false;
            }
            let pos = creature.position.floor_i();
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        if dx == 0 && dy == 0 && dz == 0 {
                            continue;
                        }
                        let check = Vec3i::new(pos.x + dx, pos.y + dy, pos.z + dz);
                        if let Some(voxel) = world.sample_voxel_mut(check) {
                            if *voxel.void_fraction > 0.35 {
                                let deposit = creature.regulatory.carried_mass.min(0.06);
                                let organic_part = deposit * 0.7;
                                let binder_part = deposit * 0.3;
                                *voxel.organic = (*voxel.organic + organic_part).min(1.0);
                                *voxel.binder = (*voxel.binder + binder_part).min(1.0);
                                *voxel.solid_fraction =
                                    (*voxel.solid_fraction + deposit * 0.15).min(1.0);
                                *voxel.void_fraction =
                                    (*voxel.void_fraction - deposit * 0.1).max(0.0);
                                creature.regulatory.carried_mass -= deposit;
                                creature.regulatory.apply_action_cost(
                                    PLACE_MATERIAL_ENERGY_COST,
                                    PLACE_MATERIAL_FATIGUE_COST,
                                );
                                emit(world, ActionSoundKind::PlaceMaterial, 0.09);
                                return true;
                            }
                        }
                    }
                }
            }
            false
        }
        Action::ApplyBinder => {
            let pos = creature.position.floor_i();
            let mut organic_source = None;
            for dx in -1..=1 {
                for dy in -1..=1 {
                    for dz in -1..=1 {
                        let check = Vec3i::new(pos.x + dx, pos.y + dy, pos.z + dz);
                        if let Some(voxel) = world.sample_voxel(check) {
                            if voxel.organic > BINDER_ORGANIC_COST {
                                organic_source = Some(check);
                                break;
                            }
                        }
                    }
                    if organic_source.is_some() {
                        break;
                    }
                }
                if organic_source.is_some() {
                    break;
                }
            }

            let from_carried = organic_source.is_none()
                && creature.regulatory.carried_mass >= BINDER_ORGANIC_COST;

            if organic_source.is_none() && !from_carried {
                return false;
            }

            let target = organic_source.unwrap_or(pos);
            if let Some(voxel) = world.sample_voxel_mut(target) {
                if from_carried {
                    creature.regulatory.carried_mass -= BINDER_ORGANIC_COST;
                } else {
                    *voxel.organic -= BINDER_ORGANIC_COST;
                }
                *voxel.binder = (*voxel.binder + 0.08).min(1.0);
                *voxel.structural_strength =
                    (*voxel.structural_strength + 0.05).min(1.5);
                creature.regulatory.apply_action_cost(
                    APPLY_BINDER_ENERGY_COST,
                    APPLY_BINDER_FATIGUE_COST,
                );
                true
            } else {
                false
            }
        }
    }
}
