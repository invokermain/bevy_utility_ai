use crate::components::Energy;
use bevy::app::App;
use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault, Transform};
use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{Linear, Polynomial};
use bevy_utility_ai_macros::{input_system, targeted_input_system};

// Define a simple input system.
#[input_system]
fn current_energy(energy: &Energy) -> f32 {
    energy.value / energy.max
}

// Define a targeted input system.
#[targeted_input_system]
fn distance(subject: (&Transform,), target: (&Transform,)) -> f32 {
    let subject_pos = subject.0.translation;
    let target_pos = target.0.translation;

    subject_pos.distance(target_pos)
}

// Define our AI Marker Component
#[derive(Component)]
pub struct HunterAI {}

// Define our Actions
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionHunt {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionRest {}

// Piece it all together
pub(crate) fn construct_hunter_ai(app: &mut App) {
    DefineAI::<HunterAI>::new()
        .add_decision(
            // When our energy is low rest
            Decision::simple::<ActionRest>().add_consideration(
                Consideration::simple(current_energy)
                    .with_response_curve(Polynomial::new(-1.0, 3.0).shifted(1.0, 0.0)),
            ),
        )
        .add_decision(
            Decision::targeted::<ActionHunt>()
                .add_consideration(
                    Consideration::targeted(distance)
                        .with_response_curve(Linear::new(-0.01).shifted(0.0, 1.0)),
                )
                .add_consideration(
                    Consideration::simple(current_energy)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                ),
        )
        .register(app);
}
