use bevy::app::App;
use bevy::prelude::Component;

use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{Linear, Polynomial};

use crate::game::ai::actions::{ActionEat, ActionIdle, ActionRest};
use crate::game::ai::inputs::{distance_to, energy, hunger};
use crate::game::entities::carrion::Carrion;
use crate::game::systems::water::Water;

use super::actions::ActionDrink;
use super::inputs::thirst;

// Define our AI Marker Component, if this is present on an entity it will enable the
// corresponding AI behaviours.
#[derive(Component)]
pub struct HunterAI {}

// Declare our hunter's behaviour
pub(crate) fn construct_hunter_ai(app: &mut App) {
    // use DefineAI to build a set of decisions that will be ran for any entity with
    // the HunterAI marker component added.
    DefineAI::<HunterAI>::new()
        // Add some intertia to our decisions so that we do not oscillate between
        // decisions.
        .set_default_intertia(0.1)
        .add_decision(
            // Rest..
            Decision::simple::<ActionRest>()
                // if we have low energy
                .add_consideration(
                    Consideration::simple(energy).with_response_curve(
                        Polynomial::new(-3.0, 3.0).shifted(0.0, 1.0),
                    ),
                )
                // and we have low hunger
                .add_consideration(
                    Consideration::simple(hunger).with_response_curve(
                        Polynomial::new(-3.0, 3.0).shifted(0.0, 1.0),
                    ),
                )
                // and we have low thirst
                .add_consideration(
                    Consideration::simple(thirst).with_response_curve(
                        Polynomial::new(-3.0, 3.0).shifted(0.0, 1.0),
                    ),
                ),
        )
        // .add_decision(
        //     // Hunt...
        //     Decision::targeted::<ActionHunt>()
        //         // only prey
        //         .target_filter_include::<PreyAI>()
        //         // prefer closer targets
        //         .add_consideration(
        //             Consideration::targeted(distance_to)
        //                 .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
        //         )
        //         // if there is no food in the area
        //         .add_consideration(
        //             Consideration::simple(carcass_availability)
        //                 .with_response_curve(Linear::new(-1.0).shifted(0.0, 1.0)),
        //         )
        //         // if we have energy
        //         .add_consideration(
        //             Consideration::simple(energy)
        //                 .with_response_curve(Polynomial::new(1.0, 0.25)),
        //         )
        //         // if we are hungry
        //         .add_consideration(
        //             Consideration::simple(hunger)
        //                 .with_response_curve(Polynomial::new(1.0, 0.25)),
        //         ),
        // )
        .add_decision(
            // Eat...
            Decision::targeted::<ActionEat>()
                // only carrion
                .target_filter_include::<Carrion>()
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to)
                        .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
                )
                // & prefer if we are hungry
                .add_consideration(
                    Consideration::simple(hunger)
                        .with_response_curve(Polynomial::new(1.05, 0.33)),
                ),
        )
        .add_decision(
            // Drink from...
            Decision::targeted::<ActionDrink>()
                // the water source
                .target_filter_include::<Water>()
                // when we are thirsty
                .add_consideration(
                    Consideration::simple(thirst)
                        .with_response_curve(Polynomial::new(1.05, 0.33)),
                )
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to).with_response_curve(
                        Polynomial::new(-0.001, 1.0).shifted(0.0, 1.0),
                    ),
                ),
        )
        .add_decision(
            // Idle...
            Decision::simple::<ActionIdle>()
                // if we have high energy
                .add_consideration(
                    Consideration::simple(energy)
                        .with_response_curve(Polynomial::new(1.0, 0.5)),
                )
                // and we have low hunger
                .add_consideration(
                    Consideration::simple(hunger).with_response_curve(
                        Polynomial::new(-1.0, 2.0).shifted(0.0, 1.0),
                    ),
                )
                // and we have low thirst
                .add_consideration(
                    Consideration::simple(thirst).with_response_curve(
                        Polynomial::new(-1.0, 2.0).shifted(0.0, 1.0),
                    ),
                ),
        )
        // registering against the app adds the input systems and actions we have used for
        // us. But it does not add any action systems as it has no awareness of them.
        .register(app);
}
