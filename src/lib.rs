use std::{any::TypeId, fmt::Debug};

use bevy::{
    prelude::{Component, Entity, Resource},
    utils::{HashMap, HashSet},
};

pub use bevy_utility_ai_macros::{input_system, targeted_input_system};

pub use crate::ai_meta::AIMeta;
use crate::decisions::{Decision, Filter};

pub mod ai_meta;
pub mod considerations;
pub mod decisions;
pub mod define_ai;
pub mod plugin;
pub mod response_curves;
pub mod systems;
pub mod utils;

#[cfg(feature = "dashboard")]
pub mod dashboard;
pub mod events;

#[derive(Debug)]
pub enum FilterDefinition {
    Any,
    Filtered(Vec<Vec<Filter>>),
}

impl FilterDefinition {
    pub fn merge(&mut self, other: &FilterDefinition) -> FilterDefinition {
        match (self, other) {
            (FilterDefinition::Any, FilterDefinition::Any) => FilterDefinition::Any,
            (FilterDefinition::Filtered(_), FilterDefinition::Any) => {
                FilterDefinition::Any
            }
            (FilterDefinition::Any, FilterDefinition::Filtered(_)) => {
                FilterDefinition::Any
            }
            (FilterDefinition::Filtered(x), FilterDefinition::Filtered(y)) => {
                let mut joined = x.clone();
                joined.extend(y.clone());
                FilterDefinition::Filtered(joined)
            }
        }
    }
}

pub struct TargetedInputRequirements {
    pub target_filter: FilterDefinition,
}

pub struct AIDefinition {
    /// The name of this definition
    pub name: String,
    /// The type id of the Marker Component associated with this definition
    pub marker_type: TypeId,
    /// The default value to use for the intertia of a decision if unspecified
    pub default_intertia: f32,
    /// The decisions that make up this AIDefinition
    pub decisions: Vec<Decision>,
    /// The simple inputs used for this AI, passed to AIDefinition on register.
    pub simple_inputs: HashSet<TypeId>,
    /// The targeted inputs used for this AI, passed to AIDefinition on register.
    pub targeted_inputs: HashMap<TypeId, TargetedInputRequirements>,
}

impl AIDefinition {
    pub fn requires_targeted_input(&self, input: &TypeId) -> bool {
        self.targeted_inputs.contains_key(input)
    }

    pub fn requires_simple_input(&self, input: &TypeId) -> bool {
        self.simple_inputs.contains(input)
    }

    pub fn get_targeted_input_requirements(
        &self,
        input: &TypeId,
    ) -> &TargetedInputRequirements {
        &self.targeted_inputs[input]
    }
}

#[derive(Resource, Default)]
pub struct AIDefinitions {
    /// Map of TypeId of the AIDefinition's Marker Component to AIDefinition
    pub map: HashMap<TypeId, AIDefinition>,
}

/// A component to hold the Target entity ID
#[derive(Component)]
pub struct ActionTarget {
    pub target: Entity,
}
