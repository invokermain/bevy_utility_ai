use crate::logic::ai::actions::ActionHunt;
use crate::logic::ai::hunter::HunterAI;
use crate::logic::ai::prey::PreyAI;
use crate::logic::rest::Energy;
use bevy::prelude::{
    Commands, Entity, Event, EventWriter, Query, Transform, Vec2, With, Without,
};
use bevy_utility_ai::ActionTarget;

#[derive(Event)]
pub struct PreyKilledEvent {
    pub entity: Entity,
    pub position: Vec2,
}

pub fn hunt(
    mut q_hunter: Query<
        (&Transform, &mut Energy, &ActionTarget),
        (With<ActionHunt>, With<HunterAI>),
    >,
    q_prey: Query<&Transform, (Without<HunterAI>, With<PreyAI>)>,
    mut commands: Commands,
    mut ev_prey_killed: EventWriter<PreyKilledEvent>,
) {
    for (hunter_transform, mut energy, target_entity) in q_hunter.iter_mut() {
        let position = &mut hunter_transform.translation.truncate();

        if let Ok(target) = q_prey.get(target_entity.target) {
            let target_position = target.translation.truncate();

            // if we are close enough kill the prey
            if target_position.distance(*position) <= 7.5 {
                commands.entity(target_entity.target).despawn();
                ev_prey_killed.send(PreyKilledEvent {
                    entity: target_entity.target,
                    position: target_position,
                })
            }
            // otherwise move towards our prey
            else {
                let movement_vector = target_position - *position;
                *position += movement_vector.normalize() * 7.5;
            }

            energy.value = (energy.value - 0.5).max(0.0);
        }
    }
}
