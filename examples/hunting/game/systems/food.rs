use crate::game::ai::actions::ActionEat;
use bevy::prelude::*;
use bevy_utility_ai::ActionTarget;

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Food {
    /// How many units of food remain from this Food source
    pub remaining: f32,
    /// The number of animals currently eating this food
    pub current_eaters: u8,
}

impl Food {
    pub fn new(units: f32) -> Self {
        Self {
            remaining: units,
            current_eaters: 0,
        }
    }
}

pub fn increase_hunger(mut q_hunger: Query<&mut Hunger>) {
    for mut hunger in q_hunger.iter_mut() {
        hunger.value += 0.1;
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
) {
    for (mut subject_hunger, mut transform, action_target) in q_subject.iter_mut() {
        if let Ok((mut food, food_transform)) = q_food.get_mut(action_target.target) {
            // if we are near food eat
            if food_transform.translation.distance(transform.translation) < 5.0 {
                let portion_size = food.remaining.min(2.5).min(subject_hunger.value);
                food.remaining -= portion_size;
                subject_hunger.value -= portion_size;
                food.current_eaters = food.current_eaters.saturating_add(1);
            }
            // otherwise walk to the food
            else {
                let direction =
                    (food_transform.translation - transform.translation).normalize();
                transform.translation += direction * 2.5;
            }
        } else {
            continue;
        };
    }
}

pub fn decrement_food_eaters(mut q_food: Query<&mut Food>) {
    for mut food in &mut q_food {
        food.current_eaters = food.current_eaters.saturating_sub(1);
    }
}
