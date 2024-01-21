use bevy::app::App;
use bevy::prelude::Component;
use bevy_utility_ai::considerations::Consideration;
use bevy_utility_ai::decisions::Decision;
use bevy_utility_ai::define_ai::DefineAI;
use bevy_utility_ai::response_curves::{Linear, Polynomial};

use crate::logic::food::{Food, Grass};
use crate::logic::water::Water;

use super::actions::{ActionDrink, ActionEat, ActionFlee, ActionHerd, ActionIdle};
use super::hunter::HunterAI;
use super::inputs::{
    distance_to, hunger, percieved_threat_level, prey_confidence, thirst,
};

#[derive(Component)]
pub struct PreyAI {}

// Declare our prey's behaviour
pub(crate) fn construct_prey_ai(app: &mut App) {
    // use DefineAI to build a set of decisions that will be ran for any entity with
    // the PreyAI marker component added.
    DefineAI::<PreyAI>::new()
        .set_default_intertia(0.1)
        .add_decision(
            // Flee from...
            Decision::targeted::<ActionFlee>()
                // the Hunter
                .target_filter_include::<HunterAI>()
                // when it is close
                .add_consideration(
                    Consideration::targeted(distance_to).with_response_curve(
                        Polynomial::new(-1.0 / 500.0f32.powi(2), 2.0).shifted(0.0, 1.0),
                    ),
                )
                // modulate by our personality
                .add_consideration(Consideration::simple(prey_confidence)),
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
                        Polynomial::new(-0.0001, 1.0).shifted(0.0, 1.0),
                    ),
                )
                // & prefer targets that have lower threat level
                .add_consideration(Consideration::targeted(percieved_threat_level))
                // set a base score of 0.95 so that we always prefer fleeing the hunter
                .set_base_score(0.90),
        )
        .add_decision(
            // Eat...
            Decision::targeted::<ActionEat>()
                // only grass
                .target_filter_include::<Grass>()
                // only food
                .target_filter_include::<Food>()
                // & prefer closer targets
                .add_consideration(
                    Consideration::targeted(distance_to)
                        .with_response_curve(Linear::new(-0.001).shifted(0.0, 1.0)),
                )
                // & prefer targets that have lower threat level
                .add_consideration(Consideration::targeted(percieved_threat_level))
                // & prefer if we are hungry
                .add_consideration(
                    Consideration::simple(hunger)
                        .with_response_curve(Polynomial::new(1.05, 0.33)),
                ),
        )
        .add_decision(
            // Herd together with...
            Decision::targeted::<ActionHerd>()
                // other prey
                .target_filter_include::<PreyAI>()
                // when they are nearby
                .add_consideration(
                    Consideration::targeted(distance_to).with_response_curve(
                        Linear::new(-1.0 / 1000.0).shifted(0.0, 1.0),
                    ),
                )
                // set a base score of 0.5 so that herding is only done when no other
                // needs require satisfying
                .set_base_score(0.5),
        )
        .add_decision(Decision::simple::<ActionIdle>().set_base_score(0.10))
        .register(app);
}
