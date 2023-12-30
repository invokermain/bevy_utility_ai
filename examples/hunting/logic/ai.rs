use crate::logic::components::Energy;
use crate::logic::food::{Food, Hunger};
use bevy::app::App;
use bevy::prelude::{Component, Reflect, ReflectComponent, ReflectDefault, Transform};
use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{Linear, Polynomial};
use bevy_utility_ai_macros::{input_system, targeted_input_system};

// Define simple input system, this input is calculated for each entity that has the
// required components.
#[input_system]
fn energy(energy: &Energy) -> f32 {
    energy.value / energy.max
}

#[input_system]
fn hunger(hunger: &Hunger) -> f32 {
    hunger.value / hunger.max
}

// Define targeted input systems, these are calculated for every combination of entity and
// target entity that match the required components.
#[targeted_input_system]
fn distance_to(subject: (&Transform,), target: (&Transform,)) -> f32 {
    let subject_pos = subject.0.translation;
    let target_pos = target.0.translation;

    subject_pos.distance(target_pos)
}

// Define some AI Marker Components, if this is present on an entity it will enable the
// corresponding AI behaviours.
#[derive(Component)]
pub struct HunterAI {}

#[derive(Component)]
pub struct PreyAI {}

// Define our Actions, when a decision is made these components will be added to the
// entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionHunt {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionRest {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionEat {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionIdle {}

// Piece it all together!
pub(crate) fn construct_hunter_ai(app: &mut App) {
    // use DefineAI to build a set of decisions that will be ran for any entity with
    // the HunterAI marker component added.
    DefineAI::<HunterAI>::new()
        .add_decision(
            // Rest..
            Decision::simple::<ActionRest>()
                // if we have low energy
                .add_consideration(
                    Consideration::simple(energy).with_response_curve(
                        Polynomial::new(-0.5, 3.0).shifted(1.0, 0.0),
                    ),
                )
                // and we have low hunger
                .add_consideration(
                    Consideration::simple(hunger).with_response_curve(
                        Polynomial::new(-0.5, 3.0).shifted(1.0, 0.0),
                    ),
                ),
        )
        .add_decision(
            // Hunt...
            Decision::targeted::<ActionHunt>()
                // only prey
                .target_filter_include::<PreyAI>()
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to)
                        .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
                )
                // & prefer if we have energy
                .add_consideration(
                    Consideration::simple(energy)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                )
                // & prefer if we are hungry
                .add_consideration(
                    Consideration::simple(hunger)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                ),
        )
        .add_decision(
            // Eat...
            Decision::targeted::<ActionEat>()
                // only food
                .target_filter_include::<Food>()
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to)
                        .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
                )
                // & prefer if we are hungry
                .add_consideration(
                    Consideration::simple(hunger)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                ),
        )
        .add_decision(
            // Idle...
            Decision::simple::<ActionIdle>().set_base_score(0.2),
        )
        // registering against the app adds the input systems and actions we have used for
        // us. But it does not add any action systems as it has no awareness of them.
        .register(app);
}
