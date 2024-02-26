use bevy::prelude::*;

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionEat;
use crate::level::HALF_GRID_SIZE;

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Food {
    pub remaining: f32,
}

impl Food {
    pub fn new(units: f32) -> Self {
        Self { remaining: units }
    }
}

pub fn increase_hunger(mut q_hunger: Query<&mut Hunger>, r_time: Res<Time>) {
    for mut hunger in q_hunger.iter_mut() {
        hunger.value += 0.5 * r_time.delta_seconds();
        if hunger.value >= hunger.max {
            hunger.value = hunger.max;
        }
    }
}

pub fn eat(
    mut q_subject: Query<
        (&mut Hunger, &mut Transform, &ActionTarget),
        (With<ActionEat>, Without<Food>),
    >,
    mut q_food: Query<(&mut Food, &Transform)>,
    r_time: Res<Time>,
) {
    for (mut subject_hunger, mut transform, action_target) in q_subject.iter_mut() {
        if let Ok((mut food, food_transform)) = q_food.get_mut(action_target.target) {
            // if we are near food eat
            if food_transform.translation.distance(transform.translation)
                <= HALF_GRID_SIZE
            {
                let portion_size = food.remaining.min(2.5).min(subject_hunger.value)
                    * r_time.delta_seconds();
                food.remaining -= portion_size;
                subject_hunger.value -= portion_size;
            }
            // otherwise walk to the food
            else {
                let direction =
                    (food_transform.translation - transform.translation).normalize();
                transform.translation += direction * r_time.delta_seconds();
            }
        } else {
            continue;
        };
    }
}
