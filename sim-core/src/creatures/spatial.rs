use crate::math::{Vec3f, Vec3i};
use crate::world::physics::apply_trail_wear;
use crate::world::{
    emit_incidental_sound, sample_material_acoustics, ActionSoundKind, World,
};

use super::creature::Creature;

const PUSH_ENERGY_COST: f32 = 0.06;
const PUSH_FATIGUE_COST: f32 = 0.12;
const PUSHED_ENERGY_COST: f32 = 0.04;
const PUSHED_FATIGUE_COST: f32 = 0.08;

/// Displacement strength from morphology mass plus carried load.
pub fn push_strength(creature: &Creature) -> f32 {
    creature.morphology.push_strength() + creature.regulatory.carried_mass * 0.6
}

pub fn creature_at_position(creatures: &[Creature], pos: Vec3i, exclude_id: u64) -> Option<usize> {
    creatures
        .iter()
        .position(|c| c.id != exclude_id && c.position.floor_i() == pos)
}

fn voxel_passable(world: &World, pos: Vec3i) -> bool {
    world
        .sample_voxel(pos)
        .map(|v| v.void_fraction > 0.4)
        .unwrap_or(false)
}

fn find_displacement_slot(
    world: &World,
    creatures: &[Creature],
    prefer: Vec3i,
    exclude_ids: &[u64],
) -> Option<Vec3i> {
    if voxel_passable(world, prefer)
        && !creatures.iter().any(|c| {
            !exclude_ids.contains(&c.id) && c.position.floor_i() == prefer
        })
    {
        return Some(prefer);
    }
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if dx == 0 && dy == 0 && dz == 0 {
                    continue;
                }
                let alt = Vec3i::new(prefer.x + dx, prefer.y + dy, prefer.z + dz);
                if voxel_passable(world, alt)
                    && !creatures.iter().any(|c| {
                        !exclude_ids.contains(&c.id) && c.position.floor_i() == alt
                    })
                {
                    return Some(alt);
                }
            }
        }
    }
    None
}

pub const FOLLOW_ENERGY_COST: f32 = 0.04;
pub const FOLLOW_FATIGUE_COST: f32 = 0.09;

/// Step direction toward the adjacent neighbor with strongest creature-trace gradient.
pub fn compute_follow_direction(creature: &Creature, others: &[Creature]) -> Option<Vec3i> {
    let center = creature.position.floor_i();
    let mut best_strength = 0.0f32;
    let mut best_dir = None;

    for other in others {
        if other.id == creature.id {
            continue;
        }
        let other_pos = other.position.floor_i();
        let dx = other_pos.x - center.x;
        let dy = other_pos.y - center.y;
        let dz = other_pos.z - center.z;
        if dx.abs() > 1 || dy.abs() > 1 || dz.abs() > 1 {
            continue;
        }
        if dx == 0 && dy == 0 && dz == 0 {
            continue;
        }
        let dist_sq = (dx * dx + dy * dy + dz * dz) as f32;
        let strength = 0.15 / dist_sq.max(1.0);
        if strength > best_strength {
            best_strength = strength;
            best_dir = Some(Vec3i::new(dx.signum(), dy.signum(), dz.signum()));
        }
    }
    best_dir
}

pub fn try_creature_move_at(
    creatures: &mut [Creature],
    idx: usize,
    delta: Vec3i,
    world: &mut World,
) -> bool {
    let id = creatures[idx].id;
    let current = creatures[idx].position.floor_i();
    let new_pos = Vec3i::new(
        current.x + delta.x,
        current.y + delta.y,
        current.z + delta.z,
    );
    if creature_at_position(creatures, new_pos, id).is_some() {
        return false;
    }
    if !voxel_passable(world, new_pos) {
        return false;
    }
    let speed = creatures[idx].genome.move_speed;
    creatures[idx].position = Vec3f::from_vec3i(new_pos);
    creatures[idx]
        .regulatory
        .apply_action_cost(0.035 * speed, 0.08);
    if let Some(mut surface) = world.sample_voxel_mut(new_pos) {
        apply_trail_wear(&mut surface);
    }
    let emitter = creatures[idx].sound_emitter_context();
    let material = sample_material_acoustics(world, new_pos);
    emit_incidental_sound(
        world,
        creatures[idx].position,
        emitter,
        ActionSoundKind::Move,
        0.08,
        material,
    );
    true
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct PushEvent {
    pub pusher_id: u64,
    pub pushed_id: u64,
    pub from: Vec3i,
    pub to: Vec3i,
}

pub fn try_creature_push_at(
    creatures: &mut [Creature],
    pusher_idx: usize,
    direction: Vec3i,
    world: &mut World,
) -> Option<PushEvent> {
    let pusher_id = creatures[pusher_idx].id;
    let pusher_pos = creatures[pusher_idx].position.floor_i();
    let target_pos = Vec3i::new(
        pusher_pos.x + direction.x,
        pusher_pos.y + direction.y,
        pusher_pos.z + direction.z,
    );

    if !voxel_passable(world, target_pos) {
        return None;
    }

    let pushed_idx = creature_at_position(creatures, target_pos, pusher_id)?;

    let pusher_strength = push_strength(&creatures[pusher_idx]);
    let pushed_strength = push_strength(&creatures[pushed_idx]);
    if pusher_strength <= pushed_strength {
        return None;
    }

    let pushed_id = creatures[pushed_idx].id;
    let exclude = [pusher_id, pushed_id];
    let displacement = find_displacement_slot(world, creatures, pusher_pos, &exclude)?;

    let (pusher, pushed) = if pusher_idx < pushed_idx {
        let (left, right) = creatures.split_at_mut(pushed_idx);
        (&mut left[pusher_idx], &mut right[0])
    } else {
        let (left, right) = creatures.split_at_mut(pusher_idx);
        (&mut right[0], &mut left[pushed_idx])
    };

    pushed.position = Vec3f::from_vec3i(displacement);
    pusher.position = Vec3f::from_vec3i(target_pos);
    pusher
        .regulatory
        .apply_action_cost(PUSH_ENERGY_COST, PUSH_FATIGUE_COST);
    pushed
        .regulatory
        .apply_action_cost(PUSHED_ENERGY_COST, PUSHED_FATIGUE_COST);

    if let Some(mut surface) = world.sample_voxel_mut(target_pos) {
        apply_trail_wear(&mut surface);
    }

    let pusher_emitter = pusher.sound_emitter_context();
    let material = sample_material_acoustics(world, target_pos);
    emit_incidental_sound(
        world,
        pusher.position,
        pusher_emitter,
        ActionSoundKind::Push,
        0.11,
        material,
    );

    Some(PushEvent {
        pusher_id,
        pushed_id,
        from: pusher_pos,
        to: target_pos,
    })
}

/// Separate overlapping creatures after all actions; weaker party yields to a free adjacent voxel.
pub fn resolve_position_overlaps(creatures: &mut [Creature], world: &World) {
    let n = creatures.len();
    for i in 0..n {
        let pos_i = creatures[i].position.floor_i();
        for j in (i + 1)..n {
            if creatures[j].position.floor_i() != pos_i {
                continue;
            }
            let strength_i = push_strength(&creatures[i]);
            let strength_j = push_strength(&creatures[j]);
            let (weak_idx, strong_id) = if strength_i <= strength_j {
                (i, creatures[j].id)
            } else {
                (j, creatures[i].id)
            };
            let weak_id = creatures[weak_idx].id;
            if let Some(slot) =
                find_displacement_slot(world, creatures, pos_i, &[weak_id, strong_id])
            {
                creatures[weak_idx].position = Vec3f::from_vec3i(slot);
            }
        }
    }
}
