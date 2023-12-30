pub mod make_decisions;
pub mod update_action;

use bevy::prelude::{Added, Commands, Component, Entity, Query, RemovedComponents};

use crate::ai_meta::AIMeta;

pub(crate) fn ensure_entity_has_ai_meta<T: Component>(
    mut commmands: Commands,
    query: Query<(Entity, Option<&AIMeta>), Added<T>>,
) {
    for (entity, ai_meta) in &query {
        if ai_meta.is_none() {
            commmands.entity(entity).insert(AIMeta::new::<T>());
        }
    }
}

pub(crate) fn handle_ai_marker_removed<T: Component>(
    mut commmands: Commands,
    mut removals: RemovedComponents<T>,
) {
    for entity in removals.read() {
        commmands.entity(entity).remove::<AIMeta>();
    }
}
