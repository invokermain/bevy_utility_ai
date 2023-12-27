use crate::ai::{ActionHunt, ActionRest, HunterAI};
use crate::components::Energy;
use bevy::prelude::{Query, Transform, With, Without};
use bevy_utility_ai::ActionTarget;

pub fn rest(mut q_energy: Query<&mut Energy, With<ActionRest>>) {
    for mut energy in q_energy.iter_mut() {
        energy.value += 1.0;
        if energy.value >= energy.max {
            energy.value = energy.max
        }
    }
}

pub fn hunt(
    mut q_hunter: Query<
        (&mut Transform, &mut Energy, &ActionTarget),
        (With<ActionHunt>, With<HunterAI>),
    >,
    q_prey: Query<&mut Transform, Without<HunterAI>>,
) {
    if q_hunter.is_empty() {
        return;
    }

    let (hunter_transform, mut energy, target_entity) = q_hunter.single_mut();

    let mut position = hunter_transform.translation;

    // get our target prey's position
    let target_position = q_prey.get(target_entity.target).unwrap().translation;

    // step towards our prey
    if target_position.x > position.x {
        position.x += 1.
    } else if target_position.x < position.x {
        position.x -= 1.
    }

    if target_position.y > position.y {
        position.y += 1.
    } else if target_position.y < position.y {
        position.y -= 1.
    }

    energy.value -= 1.0;
}
