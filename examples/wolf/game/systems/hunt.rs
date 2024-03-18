use bevy::{math::Vec3Swizzles, prelude::*};

use bevy_utility_ai::ActionTarget;

use crate::game::ai::wolf::HunterAI;
use crate::{game::ai::actions::ActionHunt, level::HALF_GRID_SIZE};

use super::pathfinding::{Path, PathRequested};

#[derive(Event)]
pub struct PreyKilledEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component, Copy, Clone)]
pub struct IsPrey;

pub fn hunt(
    mut q_hunter: Query<
        (Entity, &Transform, &ActionTarget),
        (With<ActionHunt>, With<HunterAI>, Without<Path>),
    >,
    q_prey: Query<&Transform, Without<HunterAI>>,
    mut ev_prey_killed: EventWriter<PreyKilledEvent>,
    mut ew_path_requested: EventWriter<PathRequested>,
) {
    for (entity, hunter_transform, target_entity) in q_hunter.iter_mut() {
        let position = &hunter_transform.translation.xy();

        if let Ok(target) = q_prey.get(target_entity.target) {
            let target_point = target.translation.xy();

            // if we are close enough kill the prey
            if target_point.distance(*position) <= HALF_GRID_SIZE {
                ev_prey_killed.send(PreyKilledEvent {
                    entity: target_entity.target,
                    position: target_point,
                });
            }
            // otherwise request path to prey
            else {
                ew_path_requested.send(PathRequested {
                    entity,
                    target_point,
                    speed: 5.0,
                });
            }
        }
    }
}
