// Define simple input system, this input is calculated for each entity that has the
// required components.

use crate::logic::food::{Food, Hunger};
use crate::logic::rest::Energy;
use crate::logic::water::Thirst;
use bevy::prelude::{Query, Transform};
use bevy_utility_ai_macros::{input_system, targeted_input_system};

/// How much energy we have on a scale of 0.0 to 1.0
#[input_system]
pub(crate) fn energy(energy: &Energy) -> f32 {
    energy.value / energy.max
}

/// How hungry we are on a scale of 0.0 to 1.0
#[input_system]
pub(crate) fn hunger(hunger: &Hunger) -> f32 {
    hunger.value / hunger.max
}

/// How thirsty we are on a scale of 0.0 to 1.0
#[input_system]
pub(crate) fn thirst(thirst: &Thirst) -> f32 {
    thirst.value / thirst.max
}

/// How much food is in the area relative to our hunger on a scale of 0.0 to 1.0.
#[input_system]
pub(crate) fn food_availability(hunger: &Hunger, q_food: Query<&Food>) -> f32 {
    let total_food: f32 = q_food.iter().map(|food| food.remaining).sum();
    (total_food - hunger.value).clamp(0.0, hunger.max) / hunger.max
}

// Define targeted input systems, these are calculated for every combination of entity and
// target entity that match the required components.

/// Distance to the target
#[targeted_input_system]
pub(crate) fn distance_to(subject: (&Transform,), target: (&Transform,)) -> f32 {
    let subject_pos = subject.0.translation;
    let target_pos = target.0.translation;

    subject_pos.distance(target_pos)
}
