use crate::{
    considerations::ConsiderationType,
    decisions::Decision,
    plugin::{UtilityAISet, UtilityAISettings},
    systems::{ensure_entity_has_ai_meta, handle_ai_marker_removed},
    AIDefinition, AIDefinitions, FilterDefinition, TargetedInputRequirements,
};
use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};

use crate::utils::trim_type_name;
use bevy::{
    app::App,
    prelude::{AppTypeRegistry, Component, IntoSystemConfigs, Resource},
    reflect::TypeRegistration,
    utils::{HashMap, HashSet},
};
use std::{
    any::{type_name, TypeId},
    marker::PhantomData,
};

/// A builder which allows you declaratively build your UtilityAI, where the AI is defined as a
/// set of Decisions. and returns a bundle that you can add to an entity.
pub struct DefineUtilityAI<T: Component> {
    /// The AI Set's name.
    name: String,
    /// The decisions that make up this AI's logic, passed to AIDefinition on register.
    decisions: Vec<Decision>,
    /// The simple inputs used for this AI, passed to AIDefinition on register.
    simple_inputs: HashSet<TypeId>,
    /// The targeted inputs used for this AI, passed to AIDefinition on register.
    targeted_inputs: HashMap<TypeId, TargetedInputRequirements>,
    /// A vec of all actions defined as part of this AI, will be registered to the App.
    action_type_registrations: Vec<TypeRegistration>,
    default_intertia: f32,
    marker_phantom: PhantomData<T>,
    schedule_label: Option<InternedScheduleLabel>,
}

impl<T: Component> DefineUtilityAI<T> {
    pub fn new() -> DefineUtilityAI<T> {
        Self {
            name: trim_type_name(type_name::<T>()).into(),
            marker_phantom: PhantomData,
            decisions: Vec::new(),
            simple_inputs: HashSet::new(),
            targeted_inputs: HashMap::new(),
            action_type_registrations: Vec::new(),
            schedule_label: None,
            default_intertia: 0.0,
        }
    }

    pub fn add_decision(mut self, decision: Decision) -> DefineUtilityAI<T> {
        for consideration in &decision.considerations {
            match consideration.consideration_type {
                ConsiderationType::Simple => {
                    self.simple_inputs.insert(consideration.input);
                }
                ConsiderationType::Targeted => {
                    let filter_definition = match &decision.target_filters.is_empty() {
                        true => FilterDefinition::Any,
                        false => FilterDefinition::Filtered(vec![decision
                            .target_filters
                            .clone()]),
                    };
                    if let Some(req) = self.targeted_inputs.get_mut(&consideration.input)
                    {
                        req.target_filter = req.target_filter.merge(&filter_definition)
                    } else {
                        self.targeted_inputs.insert(
                            consideration.input,
                            TargetedInputRequirements {
                                target_filter: filter_definition,
                            },
                        );
                    }
                }
            };
        }

        self.action_type_registrations
            .push(decision.type_registration.clone());
        self.decisions.push(decision);

        self
    }

    pub fn use_schedule(mut self, schedule: impl ScheduleLabel) -> DefineUtilityAI<T> {
        self.schedule_label = Some(schedule.intern());
        self
    }

    pub fn set_default_intertia(mut self, value: f32) -> DefineUtilityAI<T> {
        if !(0.0..1.0).contains(&value) {
            panic!("value must be between =0.0 and 1.0");
        }
        self.default_intertia = value;
        self
    }

    /// Registers the defined AI against the bevy App, this should be called as the last step of
    /// the defineAI process.
    pub fn register(mut self, app: &mut App) {
        // note all these actions are idempotent except app.add_system, so we maintain a resource on
        // the app to track systems that are already added.
        {
            let mut added_systems = app
                .world_mut()
                .remove_resource::<AddedSystemTracker>()
                .unwrap_or_else(|| {
                    panic!("Make sure the plugin is added to the app before calls to DefineAI")
                });

            let schedule_label = self
                .schedule_label
                .unwrap_or(app.world().resource::<UtilityAISettings>().default_schedule);

            app.add_systems(
                schedule_label,
                (
                    ensure_entity_has_ai_meta::<T>.in_set(UtilityAISet::Prepare),
                    handle_ai_marker_removed::<T>.in_set(UtilityAISet::Tidyup),
                ),
            );

            // Add utility systems
            for decision in &mut self.decisions {
                decision.considerations.iter_mut().for_each(|c| {
                    let system_app_config = c.system_app_config.take().unwrap();
                    if !added_systems.systems.contains(&c.input) {
                        app.add_systems(
                            schedule_label,
                            system_app_config.in_set(UtilityAISet::CalculateInputs),
                        );
                        added_systems.systems.insert(c.input);
                    }
                });
            }

            app.world_mut().insert_resource(added_systems);
        }

        // Register actions with the AppTypeRegistry
        {
            let registry = app.world_mut().resource_mut::<AppTypeRegistry>();
            let mut registry_write = registry.write();
            self.action_type_registrations.into_iter().for_each(|f| {
                registry_write.add_registration(f);
            });
        }

        // Add the AIDefinition to the AIDefinitions resource
        let mut ai_definitions = app.world_mut().resource_mut::<AIDefinitions>();

        if !ai_definitions.map.contains_key(&TypeId::of::<T>()) {
            let ai_definition = AIDefinition {
                name: self.name,
                marker_type: TypeId::of::<T>(),
                decisions: self.decisions,
                simple_inputs: self.simple_inputs,
                targeted_inputs: self.targeted_inputs,
                default_intertia: self.default_intertia,
            };
            ai_definitions
                .map
                .insert(ai_definition.marker_type, ai_definition);
        } else {
            panic!("AI is already defined for this marker component!")
        }
    }
}

impl<T: Component> Default for DefineUtilityAI<T> {
    fn default() -> Self {
        DefineUtilityAI::<T>::new()
    }
}

#[derive(Resource, Default)]
pub(crate) struct AddedSystemTracker {
    pub(crate) systems: HashSet<TypeId>,
}
