use bevy::prelude::*;

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionEat;
use crate::level::{WolfText, GRID_SIZE};

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
    mut q_wolf_text: Query<&mut Text, With<WolfText>>,
    r_time: Res<Time>,
) {
    for (mut subject_hunger, mut transform, action_target) in q_subject.iter_mut() {
        if let Ok((mut food, food_transform)) = q_food.get_mut(action_target.target) {
            let food_point = food_transform.translation.xy();
            let subject_point = transform.translation.xy();
            // if we are near food eat
            if food_point.distance(subject_point) <= 1.0 {
                let portion_size = food.remaining.min(5.0).min(subject_hunger.value)
                    * r_time.delta_seconds();
                food.remaining -= portion_size;
                subject_hunger.value -= portion_size;

                if let Ok(mut text) = q_wolf_text.get_single_mut() {
                    text.sections[0].value = "*crunch*".into();
                };
            }
            // otherwise walk to the food
            else {
                let direction = (food_point - subject_point).normalize();
                transform.translation +=
                    direction.extend(0.0) * r_time.delta_seconds() * GRID_SIZE;
            }
        } else {
            continue;
        };
    }
}
