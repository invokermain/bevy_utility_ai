use crate::dashboard::view::DashboardState;
use crate::dashboard::view_models::ViewAIDefinition;
use crate::events::{
    ConsiderationCalculatedEvent, DecisionCalculatedEvent, InputCalculatedEvent,
};
use crate::AIDefinitions;
use bevy::ecs::archetype::Archetypes;
use bevy::ecs::component::Components;
use bevy::prelude::{Entity, EventReader, Local, Res, ResMut, Resource};
use bevy::utils::HashMap;
use std::any::TypeId;
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub(crate) struct DashboardData {
    pub(crate) ai_definitions: Vec<ViewAIDefinition>,
    /// The entities that have the select AI Definition applied to them.
    pub(crate) entities: Vec<Entity>,
    /// Input scores for the selected Entities
    pub(crate) input_scores:
        HashMap<Entity, HashMap<(String, Option<Entity>), VecDeque<f32>>>,
    /// Considerations scores for the selected Entities
    pub(crate) consideration_scores:
        HashMap<Entity, HashMap<(String, Option<Entity>), VecDeque<f32>>>,
    /// Decision scores for the selected Entities
    pub(crate) decision_scores:
        HashMap<Entity, HashMap<(String, Option<Entity>), VecDeque<f32>>>,
}

impl DashboardData {
    fn clear(&mut self) {
        self.entities.clear();
        self.input_scores.clear();
        self.consideration_scores.clear();
        self.decision_scores.clear();
    }
}

pub(crate) const GRAPH_HISTORY_SIZE: usize = 256;

#[derive(Default)]
pub(crate) struct TypeIdState(Option<TypeId>);

pub(crate) fn sync_dashboard_data(
    mut dashboard_data: ResMut<DashboardData>,
    mut dashboard_state: ResMut<DashboardState>,
    mut previous_ai_definition: Local<TypeIdState>,
    r_ai_definitions: Res<AIDefinitions>,
    archetypes: &Archetypes,
    components: &Components,
) {
    if dashboard_state.paused {
        return;
    };

    dashboard_data.ai_definitions = r_ai_definitions
        .map
        .values()
        .map(ViewAIDefinition::from_ai_definition)
        .collect();

    let selected_ai_definition_changed = dashboard_state
        .selected_ai_definition
        .as_ref()
        .map(|def| def.id)
        != previous_ai_definition.0;

    if selected_ai_definition_changed {
        dashboard_data.clear();
    }

    // Find entities that have the selected AI Definition's marker component
    match &dashboard_state.selected_ai_definition {
        None => dashboard_data.entities.clear(),
        Some(selected_ai_definition) => {
            let marker_component = selected_ai_definition.id;
            let select_ai_definition_component_id =
                components.get_id(marker_component).unwrap();

            dashboard_data.entities = archetypes
                .iter()
                .filter(|archetype| archetype.contains(select_ai_definition_component_id))
                .flat_map(|archetype| archetype.entities())
                .map(|archetype_entity| archetype_entity.entity())
                .collect();
        }
    }

    *previous_ai_definition = TypeIdState(
        dashboard_state
            .selected_ai_definition
            .as_ref()
            .map(|f| f.id),
    );

    if selected_ai_definition_changed {
        dashboard_state.reset(&dashboard_data);
    }
}

pub(crate) fn record_input_scores(
    mut events: EventReader<InputCalculatedEvent>,
    mut dashboard_data: ResMut<DashboardData>,
    dashboard_state: Res<DashboardState>,
) {
    if dashboard_state.paused {
        return;
    };

    // Remove last tick's data and set all values to default of 0.0, as it is not guaranteed that we
    // will get an event for every input score we are tracking which might lead to desyncs.
    if !events.is_empty() {
        dashboard_data
            .input_scores
            .values_mut()
            .flat_map(|inner_map| inner_map.values_mut())
            .for_each(|scores| {
                scores.pop_front();
                scores.push_back(0.0);
            });
    }

    for event in events.read() {
        if dashboard_state.selected_entities.contains(&event.entity) {
            let entry = dashboard_data.input_scores.entry(event.entity).or_default();

            let scores_vec = entry
                .entry((event.input.clone(), event.target))
                .or_insert(VecDeque::from_iter(vec![0.0; GRAPH_HISTORY_SIZE]));

            scores_vec.pop_back();
            scores_vec.push_back(event.score);
            // as we are plotting slices over this vec we must make it contiguous
            scores_vec.make_contiguous();
        }
    }
}

pub(crate) fn record_consideration_scores(
    mut events: EventReader<ConsiderationCalculatedEvent>,
    mut dashboard_data: ResMut<DashboardData>,
    dashboard_state: Res<DashboardState>,
) {
    if dashboard_state.paused {
        return;
    };

    // Remove last tick's data and set all values to default of 0.0, as it is not guaranteed that we
    // will get an event for every input score we are tracking which might lead to desyncs.
    if !events.is_empty() {
        dashboard_data
            .consideration_scores
            .values_mut()
            .flat_map(|inner_map| inner_map.values_mut())
            .for_each(|scores| {
                scores.pop_front();
                scores.push_back(0.0);
            });
    }
    for event in events.read() {
        if dashboard_state.selected_entities.contains(&event.entity) {
            let entry = dashboard_data
                .consideration_scores
                .entry(event.entity)
                .or_default();

            let scores_vec = entry
                .entry((event.consideration_name.clone(), event.target))
                .or_insert(VecDeque::from_iter(vec![0.0; GRAPH_HISTORY_SIZE]));

            scores_vec.pop_back();
            scores_vec.push_back(event.score);
            // as we are plotting slices over this vec we must make it contiguous
            scores_vec.make_contiguous();
        }
    }
}

pub(crate) fn record_decision_scores(
    mut events: EventReader<DecisionCalculatedEvent>,
    mut dashboard_data: ResMut<DashboardData>,
    dashboard_state: Res<DashboardState>,
) {
    if dashboard_state.paused {
        return;
    };

    // Remove last tick's data and set all values to default of 0.0, as it is not guaranteed that we
    // will get an event for every input score we are tracking which might lead to desyncs.
    if !events.is_empty() {
        dashboard_data
            .decision_scores
            .values_mut()
            .flat_map(|inner_map| inner_map.values_mut())
            .for_each(|scores| {
                scores.pop_front();
                scores.push_back(0.0);
            });
    }
    for event in events.read() {
        if dashboard_state.selected_entities.contains(&event.entity) {
            let entry = dashboard_data
                .decision_scores
                .entry(event.entity)
                .or_default();

            let scores_vec = entry
                .entry((event.decision.clone(), event.target))
                .or_insert(VecDeque::from_iter(vec![0.0; GRAPH_HISTORY_SIZE]));

            scores_vec.pop_back();
            scores_vec.push_back(event.score);
            // as we are plotting slices over this vec we must make it contiguous
            scores_vec.make_contiguous();
        }
    }
}
