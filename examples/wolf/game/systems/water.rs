use bevy::prelude::*;

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionDrink;
use crate::level::{Walls, GRID_SIZE};
use crate::utils::pathfinding::{calculate_path, Path};

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
        (
            Entity,
            &mut Thirst,
            &mut Transform,
            Option<&mut Path>,
            &ActionTarget,
        ),
        (With<ActionDrink>, Without<Water>),
    >,
    mut q_water: Query<&Transform, With<Water>>,
    r_time: Res<Time>,
    r_walls: Res<Walls>,
    mut commands: Commands,
) {
    for (entity, mut subject_thirst, mut transform, maybe_path, action_target) in
        q_subject.iter_mut()
    {
        if let Ok(water_transform) = q_water.get_mut(action_target.target) {
            let water_point = water_transform.translation.xy();
            let subject_point = transform.translation.xy();

            if maybe_path.is_none()
                || !maybe_path
                    .as_ref()
                    .unwrap()
                    .validate_destination(&water_point)
            {
                let path = calculate_path(&subject_point, &water_point, &r_walls);
                if let Some(path) = path {
                    commands.entity(entity).insert(path);
                }
                continue;
            }

            let mut path = maybe_path.unwrap();
            let target_point = path.current_path_point();
            let distance_to_target = subject_point.distance(target_point);
            // if we are near water drink
            if distance_to_target <= 1.0 {
                if path.is_path_complete() {
                    let portion_size = 20.0f32.min(subject_thirst.value);
                    subject_thirst.value -= portion_size * r_time.delta_seconds();
                } else {
                    path.complete_path_point();
                }
            }
            // otherwise walk to the water
            else {
                let direction = (target_point - subject_point).normalize();
                transform.translation += direction.extend(0.0)
                    * (GRID_SIZE * 2.0 * r_time.delta_seconds()).min(distance_to_target);
            }
        } else {
            continue;
        };
    }
}
