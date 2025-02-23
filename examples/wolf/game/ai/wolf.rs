use bevy::{app::App, prelude::Component};

use bevy_utility_ai::{
    considerations::Consideration,
    decisions::Decision,
    define_ai::DefineUtilityAI,
    response_curves::{Linear, PiecewiseLinear, Polynomial},
};

use crate::game::{
    ai::{
        actions::{ActionEat, ActionHunt, ActionIdle, ActionRest},
        inputs::{distance_to, energy, hunger},
    },
    entities::carrion::Meat,
    systems::{drink::Water, hunt::IsPrey},
};

use super::{
    actions::ActionDrink,
    inputs::{is_asleep, thirst},
};

// Define our AI Marker Component, if this is present on an entity it will enable the
// corresponding AI behaviours.
#[derive(Component)]
pub struct HunterAI {}

// Declare our hunter's behaviour
pub(crate) fn construct_hunter_ai(app: &mut App) {
    // use DefineAI to build a set of decisions that will be run for any entity with
    // the HunterAI marker component added.
    DefineUtilityAI::<HunterAI>::new()
        // Add some intertia to our decisions so that we do not oscillate between
        // decisions.
        .set_default_intertia(0.1)
        .add_decision(
            // Hunt...
            Decision::targeted::<ActionHunt>()
                // only prey
                .target_filter_include::<IsPrey>()
                // prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to)
                        .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
                )
                // if we have energy
                .add_consideration(
                    Consideration::simple(energy)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                )
                // if we are hungry
                .add_consideration(
                    Consideration::simple(hunger)
                        .with_response_curve(Polynomial::new(1.0, 0.25)),
                ),
        )
        .add_decision(
            // Eat...
            Decision::targeted::<ActionEat>()
                // only food
                .target_filter_include::<Meat>()
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
                        .with_response_curve(Polynomial::new(1.05, 0.2)),
                )
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to).with_response_curve(
                        PiecewiseLinear::new([
                            (0.0, 1.0),
                            (32.0, 1.0),
                            (48.0, 0.9),
                            (256.0, 0.75),
                        ]),
                    ),
                ),
        )
        .add_decision(
            // Idle...
            Decision::simple::<ActionIdle>()
                // reduce likelihood to idle slightly so it is more of a 'fallback'
                .set_base_score(0.8)
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
        .add_decision(
            // Rest..
            Decision::simple::<ActionRest>()
                // if we have low energy
                .add_consideration(
                    Consideration::simple(energy).with_response_curve(
                        Polynomial::new(-1.0, 2.0).shifted(0.0, 1.0),
                    ),
                )
                // if we are asleep prefer to stay asleep (to make it 'sticky'),
                // by changing the bounds we are in effect making this a multiplier for
                // when our Wolf is alseep
                .add_consideration(
                    Consideration::simple(is_asleep)
                        .with_response_curve(Linear::new(3.0))
                        .with_bounds(1.0, 3.0),
                ),
        )
        // registering against the app adds the input systems and actions we have used for
        // us. But it does not add any action systems as it has no awareness of them.
        .register(app);
}
