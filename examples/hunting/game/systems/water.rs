use crate::game::ai::actions::ActionDrink;
use bevy::prelude::*;
use bevy_utility_ai::ActionTarget;

#[derive(Component)]
pub struct Thirst {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Water {}

pub fn increase_thirst(mut q_thirst: Query<&mut Thirst>) {
    for mut thirst in &mut q_thirst {
        thirst.value += 0.2;
        if thirst.value >= thirst.max {
            thirst.value = thirst.max;
        }
    }
}

pub fn drink(
    mut q_subject: Query<
        (&mut Thirst, &mut Transform, &ActionTarget),
        (With<ActionDrink>, Without<Water>),
    >,
    mut q_water: Query<&Transform, With<Water>>,
) {
    for (mut subject_thirst, mut transform, action_target) in q_subject.iter_mut() {
        if let Ok(water_transform) = q_water.get_mut(action_target.target) {
            // if we are near water drink
            if water_transform.translation.distance(transform.translation) < 25.0 {
                let portion_size = 5.0f32.min(subject_thirst.value);
                subject_thirst.value -= portion_size;
            }
            // otherwise walk to the water
            else {
                let direction =
                    (water_transform.translation - transform.translation).normalize();
                transform.translation += direction * 2.5;
            }
        } else {
            continue;
        };
    }
}
