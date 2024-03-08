use bevy::prelude::{Component, Res, Time};
use bevy::{
    math::Vec3Swizzles,
    prelude::{Entity, Event, EventWriter, Query, Transform, Vec2, With, Without},
};

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionHunt;
use crate::game::ai::wolf::HunterAI;
use crate::game::systems::rest::Energy;
use crate::level::GRID_SIZE;

#[derive(Event)]
pub struct PreyKilledEvent {
    pub entity: Entity,
    pub position: Vec2,
}

#[derive(Component, Copy, Clone)]
pub struct IsPrey;

pub fn hunt(
    mut q_hunter: Query<
        (&mut Transform, &mut Energy, &ActionTarget),
        (With<ActionHunt>, With<HunterAI>),
    >,
    q_prey: Query<&Transform, Without<HunterAI>>,
    r_time: Res<Time>,
    mut ev_prey_killed: EventWriter<PreyKilledEvent>,
) {
    for (mut hunter_transform, mut energy, target_entity) in q_hunter.iter_mut() {
        let position = &mut hunter_transform.translation.xy();

        if let Ok(target) = q_prey.get(target_entity.target) {
            let target_position = target.translation.xy();

            // if we are close enough kill the prey
            if target_position.distance(*position) <= 1.0 {
                ev_prey_killed.send(PreyKilledEvent {
                    entity: target_entity.target,
                    position: target_position,
                });
            }
            // otherwise move towards our prey
            else {
                let movement_vector = target_position - *position;
                hunter_transform.translation += movement_vector.normalize().extend(0.0)
                    * 3.
                    * GRID_SIZE
                    * r_time.delta_seconds();
            }

            energy.value = (energy.value - 5.0 * r_time.delta_seconds()).max(0.0);
        }
    }
}
