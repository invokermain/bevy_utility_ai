use bevy::prelude::*;

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionDrink;
use crate::game::systems::pathfinding::{Path, PathRequested};
use crate::level::WolfText;

#[derive(Component)]
pub struct Thirst {
    pub value: f32,
    pub max: f32,
}

#[derive(Component, Default, Copy, Clone)]
pub struct Water;

pub fn increase_thirst(mut q_thirst: Query<&mut Thirst>, r_time: Res<Time>) {
    for mut thirst in &mut q_thirst {
        thirst.value += 0.5 * r_time.delta_seconds();
        if thirst.value >= thirst.max {
            thirst.value = thirst.max;
        }
    }
}

pub fn drink(
    mut q_subject: Query<
        (Entity, &mut Thirst, &Transform, &ActionTarget),
        (With<ActionDrink>, Without<Water>, Without<Path>),
    >,
    mut q_water: Query<&Transform, With<Water>>,
    mut q_wolf_text: Query<&mut Text, With<WolfText>>,
    r_time: Res<Time>,
    mut ew_path_requested: EventWriter<PathRequested>,
) {
    for (entity, mut subject_thirst, transform, action_target) in q_subject.iter_mut() {
        if let Ok(water_transform) = q_water.get_mut(action_target.target) {
            let water_point = water_transform.translation.xy();
            let subject_point = transform.translation.xy();
            // if we are near water drink
            if subject_point.distance(water_point) <= 1.0 {
                let portion_size =
                    (20.0f32 * r_time.delta_seconds()).min(subject_thirst.value);
                subject_thirst.value -= portion_size;

                if let Ok(mut text) = q_wolf_text.get_single_mut() {
                    text.sections[0].value = "*slurp*".into();
                };
            }
            // otherwise request path to water
            else {
                ew_path_requested.send(PathRequested {
                    entity,
                    target_point: water_point,
                    speed: 2.0,
                });
            }
        };
    }
}
