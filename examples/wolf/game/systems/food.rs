use bevy::prelude::*;

use bevy_utility_ai::ActionTarget;

use crate::game::ai::actions::ActionEat;
use crate::level::WolfText;

use super::pathfinding::PathRequested;

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
        (Entity, &mut Hunger, &Transform, &ActionTarget),
        (With<ActionEat>, Without<Food>),
    >,
    mut q_food: Query<(&mut Food, &Transform)>,
    mut q_wolf_text: Query<&mut Text, With<WolfText>>,
    r_time: Res<Time>,
    mut ew_path_requested: EventWriter<PathRequested>,
) {
    for (entity, mut subject_hunger, transform, action_target) in q_subject.iter_mut() {
        if let Ok((mut food, food_transform)) = q_food.get_mut(action_target.target) {
            let food_point = food_transform.translation.xy();
            let subject_point = transform.translation.xy();
            // if we are near food eat
            if food_point.distance(subject_point) <= 1.0 {
                let portion_size = (5.0 * r_time.delta_seconds())
                    .min(food.remaining)
                    .min(subject_hunger.value);
                food.remaining -= portion_size;
                subject_hunger.value -= portion_size;

                if let Ok(mut text) = q_wolf_text.get_single_mut() {
                    text.sections[0].value = "*crunch*".into();
                };
            }
            // otherwise request path to the food
            else {
                debug!(
                    "distance to food: {:.2}, wolf: {:?}, food: {:?}",
                    food_point.distance(subject_point),
                    subject_point,
                    food_point
                );
                ew_path_requested.send(PathRequested {
                    entity,
                    target_point: food_point,
                    speed: 2.0,
                });
            }
        } else {
            continue;
        };
    }
}
