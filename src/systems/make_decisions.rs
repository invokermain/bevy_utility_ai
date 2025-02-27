use crate::considerations::ConsiderationType;
use crate::decisions::Filter;
#[cfg(debug_assertions)]
use crate::events::{ConsiderationCalculatedEvent, DecisionCalculatedEvent};
use crate::systems::update_action::UpdateEntityActionInternalEvent;
use crate::{AIDefinitions, AIMeta, Decision};
use bevy::ecs::archetype::{Archetype, Archetypes};
use bevy::ecs::component::Components;
use bevy::ecs::entity::Entities;
use bevy::log::{debug, debug_span, warn};
use bevy::prelude::{Entity, Event, EventWriter, Query, Res};
use bevy::utils::HashMap;

pub(crate) fn make_decisions_sys(
    mut query: Query<(Entity, &mut AIMeta)>,
    mut ew_update_entity_action: EventWriter<UpdateEntityActionInternalEvent>,
    mut ew_entity_action_changed: EventWriter<EntityActionChangedEvent>,
    #[cfg(debug_assertions)] mut ew_consideration_calculated: EventWriter<
        ConsiderationCalculatedEvent,
    >,
    #[cfg(debug_assertions)] mut ew_decision_calculated: EventWriter<
        DecisionCalculatedEvent,
    >,
    ai_definitions: Res<AIDefinitions>,
    archetypes: &Archetypes,
    entities: &Entities,
    components: &Components,
) {
    let _span = debug_span!("Making Decisions").entered();

    for (entity_id, mut ai_meta) in query.iter_mut() {
        let ai_definition = &ai_definitions.map[&ai_meta.ai_definition];

        let entity_archetype = archetypes
            .get(entities.get(entity_id).unwrap().archetype_id)
            .unwrap();

        let _span = debug_span!("", entity = entity_id.index()).entered();
        let mut evaluated_decisions = Vec::new();

        for (idx, decision) in ai_definition.decisions.iter().enumerate() {
            let _span = debug_span!("evaluating", name = decision.name).entered();

            let matches_filter = decision.subject_filters.iter().all(|filter| {
                entity_matches_component_filter(filter, entity_archetype, components)
            });

            if !matches_filter {
                debug!("Skipped as entity does not match subject_filter");
                continue;
            }

            let mut decision_score = decision.base_score;

            // consider simple considerations first
            for consideration in decision
                .considerations
                .iter()
                .filter(|c| c.consideration_type == ConsiderationType::Simple)
            {
                let consideration_input_score = *ai_meta
                    .input_scores
                    .get(&consideration.input)
                    .unwrap_or(&f32::NEG_INFINITY);
                if consideration_input_score == f32::NEG_INFINITY {
                    debug!(
                        "It looks like input system for '{}' hasn't run, an entity might \
                        have components missing?",
                        consideration.name
                    );
                } else {
                    let mut consideration_score =
                        consideration.calculate_score(consideration_input_score);
                    if consideration_score.is_nan() {
                        warn!(
                            "consideration {} response curve returned NaN for input {:.2}",
                            consideration.name,
                            consideration_input_score
                        );
                        consideration_score = 0.0;
                    }
                    debug!(
                        "Consideration '{}' scored: {:.2} (raw {:.2})",
                        consideration.name,
                        consideration_score,
                        consideration_input_score
                    );

                    #[cfg(debug_assertions)]
                    ew_consideration_calculated.send(ConsiderationCalculatedEvent {
                        entity: entity_id,
                        decision: decision.id,
                        consideration: consideration.id,
                        target: None,
                        score: consideration_score,
                    });

                    decision_score *= consideration_score;
                }
            }

            if !decision.is_targeted {
                evaluated_decisions.push((idx, None, decision_score));
                debug!("Decision score: {:.2}", decision_score);

                #[cfg(debug_assertions)]
                ew_decision_calculated.send(DecisionCalculatedEvent {
                    entity: entity_id,
                    decision: decision.id,
                    target: None,
                    score: decision_score,
                });
                continue;
            }

            let mut targeted_scores = HashMap::new();

            // consider targeted considerations
            for consideration in decision
                .considerations
                .iter()
                .filter(|c| c.consideration_type == ConsiderationType::Targeted)
            {
                let score_map = ai_meta.targeted_input_scores.get(&consideration.input);
                if score_map.is_none() {
                    debug!(
                        "No scores calculated yet for targeted input system {}, skipping",
                        consideration.input_name
                    );
                    continue;
                };
                let score_map = score_map.unwrap();
                let mut defunct_entities = Vec::new();
                debug!("{:?}", score_map);
                for (&target_entity_id, &consideration_input_score) in score_map {
                    let _span = debug_span!("", target_entity = target_entity_id.index())
                        .entered();

                    let target_entity = entities.get(target_entity_id);
                    if target_entity.is_none() {
                        defunct_entities.push(target_entity_id);
                        continue;
                    }
                    let target_entity_archetype =
                        archetypes.get(target_entity.unwrap().archetype_id).unwrap();
                    let matches_filter = decision.target_filters.iter().all(|filter| {
                        entity_matches_component_filter(
                            filter,
                            target_entity_archetype,
                            components,
                        )
                    });
                    if !matches_filter {
                        debug!("Skipped as target entity does not match target_filter");
                        continue;
                    }
                    let consideration_score =
                        consideration.calculate_score(consideration_input_score);
                    debug!(
                        "Consideration '{}' for entity {:?} scored: {:.2} (raw {:.2})",
                        consideration.name,
                        target_entity_id,
                        consideration_score,
                        consideration_input_score
                    );

                    #[cfg(debug_assertions)]
                    ew_consideration_calculated.send(ConsiderationCalculatedEvent {
                        entity: entity_id,
                        decision: decision.id,
                        consideration: consideration.id,
                        target: Some(target_entity_id),
                        score: consideration_score,
                    });

                    *targeted_scores
                        .entry(target_entity_id)
                        .or_insert(decision_score) *= consideration_score;
                }

                // Tidy up despawned entities
                for defunct_entity in &defunct_entities {
                    let score_map = ai_meta
                        .targeted_input_scores
                        .get_mut(&consideration.input)
                        .unwrap();
                    score_map.remove(defunct_entity);
                }
            }

            for (entity, targeted_decision_score) in targeted_scores {
                evaluated_decisions.push((idx, Some(entity), targeted_decision_score));
                debug!(
                    "Decision score for entity {:?}: {:.2}",
                    entity, targeted_decision_score
                );

                #[cfg(debug_assertions)]
                ew_decision_calculated.send(DecisionCalculatedEvent {
                    entity: entity_id,
                    decision: decision.id,
                    target: Some(entity),
                    score: targeted_decision_score,
                });
            }
        }

        if evaluated_decisions.is_empty() {
            debug!("No scorable considerations for decision, skipping");
            continue;
        }

        // add inertia to current active decision
        let current_decision_idx = ai_definition
            .decisions
            .iter()
            .position(|decision| Some(decision.action) == ai_meta.current_action);

        if let Some(current_decision_idx) = current_decision_idx {
            let decision_inertia = ai_definition.decisions[current_decision_idx].intertia;
            let inertia = decision_inertia.unwrap_or(ai_definition.default_intertia);
            if inertia >= 0.0 {
                if let Some(index) = evaluated_decisions.iter_mut().position(
                    |(decision_idx, target, _)| {
                        decision_idx == &current_decision_idx
                            && target == &ai_meta.current_target
                    },
                ) {
                    evaluated_decisions[index].2 += inertia;
                }
            };
        }

        // pick best decision
        evaluated_decisions.sort_by(|a, b| b.2.total_cmp(&a.2));
        let (decision_idx, target, score) = evaluated_decisions.first().unwrap();

        let Decision {
            action_name,
            action,
            ..
        } = &ai_definition.decisions[*decision_idx];

        let keep_current_action = Some(*action) == ai_meta.current_action;
        let keep_current_target = *target == ai_meta.current_target;

        if keep_current_action && keep_current_target {
            ai_meta.current_action_score = *score;
            continue;
        } else {
            // Change our current action, we do this in another system as it will
            // unfortunately require mut World access so isn't parallelisable.
            // TODO: this could be refactored to use EntityCommands at some point
            ew_update_entity_action.send(UpdateEntityActionInternalEvent {
                entity_id,
                old_action: ai_meta.current_action,
                new_action: *action,
                old_target: ai_meta.current_target,
                new_target: *target,
            });

            ew_entity_action_changed.send(EntityActionChangedEvent {
                entity_id,
                prev_action: ai_meta.current_action_name.clone(),
                new_action: action_name.clone(),
                prev_target: ai_meta.current_target,
                new_target: *target,
                prev_score: ai_meta.current_action_score,
                new_score: *score,
            });

            ai_meta.current_action = Some(*action);
            ai_meta.current_action_name = action_name.clone();
            ai_meta.current_action_score = *score;
            ai_meta.current_target = *target;
        }
    }
}

fn entity_matches_component_filter(
    component_filter: &Filter,
    archetype: &Archetype,
    components: &Components,
) -> bool {
    if let Some(component) = components.get_id(component_filter.component_type_id()) {
        match component_filter {
            Filter::Inclusive(_) => archetype.contains(component),
            Filter::Exclusive(_) => !archetype.contains(component),
        }
    } else {
        // Component hasn't even been registered with the app
        match component_filter {
            Filter::Inclusive(_) => false,
            Filter::Exclusive(_) => true,
        }
    }
}

/// This event is for public consumption.
/// Note that action might stay the same but target can change.
#[derive(Event)]
pub struct EntityActionChangedEvent {
    pub entity_id: Entity,
    pub prev_action: String,
    pub new_action: String,
    pub prev_target: Option<Entity>,
    pub new_target: Option<Entity>,
    pub prev_score: f32,
    pub new_score: f32,
}
