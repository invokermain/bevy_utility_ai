// Define simple input system, this input is calculated for each entity that has the
// required components.

use crate::logic::food::{Carrion, Food, Hunger};
use crate::logic::prey::PreyPersonality;
use crate::logic::rest::Energy;
use crate::logic::water::Thirst;
use bevy::ecs::query::With;
use bevy::math::{Rect, Vec3Swizzles};
use bevy::prelude::{Query, Transform};
use bevy_utility_ai_macros::{input_system, targeted_input_system};

use super::hunter::HunterAI;

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
pub(crate) fn carcass_availability(
    hunger: &Hunger,
    q_food: Query<&Food, With<Carrion>>,
) -> f32 {
    let total_food: f32 = q_food.iter().map(|food| food.remaining).sum();
    (total_food - hunger.value).clamp(0.0, hunger.max) / hunger.max
}

/// Our Confidence (for Prey)
#[input_system]
pub(crate) fn prey_confidence(personality: &PreyPersonality) -> f32 {
    personality.confidence
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

/// Threat Level at Target modulated by Prey's confidence.
#[targeted_input_system]
pub(crate) fn perceived_threat_level(
    subject: (&PreyPersonality,),
    target: (&Transform,),
    q_hunter: Query<&Transform, With<HunterAI>>,
) -> f32 {
    let target_pos = target.0.translation.xy();
    let hunter_pos = q_hunter.single().translation.xy();
    (1.0 - target_pos.distance(hunter_pos) / 500.0).max(0.0) / subject.0.confidence
}

/// Is path blocked by the Hunter, returns 1.0 if it is else 0.0
#[targeted_input_system]
pub(crate) fn is_path_blocked(
    subject: (&Transform,),
    target: (&Transform,),
    q_hunter: Query<&Transform, With<HunterAI>>,
) -> f32 {
    let subject_pos = subject.0.translation.xy();
    let target_pos = target.0.translation.xy();
    let hunter_pos = q_hunter.single().translation.xy();
    let cross_axis = (target_pos - subject_pos).perp().normalize();

    let bounding_box = Rect::from_corners(
        target_pos + cross_axis * 50.0,
        subject_pos - cross_axis * 50.0,
    );

    if bounding_box.contains(hunter_pos) {
        1.0
    } else {
        0.0
    }
}
