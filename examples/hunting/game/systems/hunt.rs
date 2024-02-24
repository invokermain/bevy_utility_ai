use crate::game::ai::hunter::HunterAI;
use crate::game::ai::prey::PreyAI;
use crate::game::systems::rest::Energy;
use crate::{game::ai::actions::ActionHunt, layers::ACTOR_LAYER};
use bevy::{
    math::Vec3Swizzles,
    prelude::{Entity, Event, EventWriter, Query, Transform, Vec2, With, Without},
};
use bevy_utility_ai::ActionTarget;

#[derive(Event)]
pub struct PreyKilledEvent {
    pub entity: Entity,
    pub position: Vec2,
}

pub fn hunt(
    mut q_hunter: Query<
        (&mut Transform, &mut Energy, &ActionTarget),
        (With<ActionHunt>, With<HunterAI>),
    >,
    q_prey: Query<&Transform, (Without<HunterAI>, With<PreyAI>)>,
    mut ev_prey_killed: EventWriter<PreyKilledEvent>,
) {
    for (mut hunter_transform, mut energy, target_entity) in q_hunter.iter_mut() {
        let position = &mut hunter_transform.translation.xy();

        if let Ok(target) = q_prey.get(target_entity.target) {
            let target_position = target.translation.xy();

            // if we are close enough kill the prey
            if target_position.distance(*position) <= 7.5 {
                ev_prey_killed.send(PreyKilledEvent {
                    entity: target_entity.target,
                    position: target_position,
                });
            }
            // otherwise move towards our prey
            else {
                let movement_vector = target_position - *position;
                hunter_transform.translation +=
                    (movement_vector.normalize() * 7.5).extend(ACTOR_LAYER);
            }

            energy.value = (energy.value - 0.5).max(0.0);
        }
    }
}
