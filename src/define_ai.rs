use crate::considerations::ConsiderationType;
use crate::decisions::Decision;
use crate::plugin::{UtilityAISet, UtilityAISettings};
use crate::systems::{ensure_entity_has_ai_meta, handle_ai_marker_removed};
use crate::{AIDefinition, AIDefinitions, FilterDefinition, TargetedInputRequirements};
use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};

use crate::utils::trim_type_name;
use bevy::prelude::{AppTypeRegistry, IntoSystemConfigs};
use bevy::{
    app::App,
    prelude::Component,
    prelude::Resource,
    reflect::TypeRegistration,
    utils::{HashMap, HashSet},
};
use std::any::{type_name, TypeId};
use std::marker::PhantomData;

/// A builder which allows you declaratively specify your AI
/// and returns a bundle that you can add to an entity.
pub struct DefineAI<T: Component> {
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

impl<T: Component> DefineAI<T> {
    pub fn new() -> DefineAI<T> {
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

    pub fn add_decision(mut self, decision: Decision) -> DefineAI<T> {
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

    pub fn use_schedule(mut self, schedule: impl ScheduleLabel) -> DefineAI<T> {
        self.schedule_label = Some(schedule.intern());
        self
    }

    pub fn set_default_intertia(mut self, value: f32) -> DefineAI<T> {
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
                .world
                .remove_resource::<AddedSystemTracker>()
                .unwrap_or_else(|| {
                    panic!("Make sure the plugin is added to the app before calls to DefineAI")
                });

            let schedule_label = self
                .schedule_label
                .unwrap_or(app.world.resource::<UtilityAISettings>().default_schedule);

            app.add_systems(
                schedule_label,
                (
                    ensure_entity_has_ai_meta::<T>,
                    handle_ai_marker_removed::<T>,
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

            app.world.insert_resource(added_systems);
        }

        // Register actions with the AppTypeRegistry
        {
            let registry = app.world.resource_mut::<AppTypeRegistry>();
            let mut registry_write = registry.write();
            self.action_type_registrations
                .into_iter()
                .for_each(|f| registry_write.add_registration(f));
        }

        // Add the AIDefinition to the AIDefinitions resource
        let mut ai_definitions = app.world.resource_mut::<AIDefinitions>();

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

impl<T: Component> Default for DefineAI<T> {
    fn default() -> Self {
        DefineAI::<T>::new()
    }
}

#[derive(Resource, Default)]
pub(crate) struct AddedSystemTracker {
    pub(crate) systems: HashSet<TypeId>,
}
