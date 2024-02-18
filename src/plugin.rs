use crate::define_ai::AddedSystemTracker;
use crate::events::{
    ConsiderationCalculatedEvent, DecisionCalculatedEvent, InputCalculatedEvent,
};
use crate::systems::make_decisions::{make_decisions_sys, EntityActionChangedEvent};
use crate::systems::update_action::{
    update_actions_sys, UpdateEntityActionInternalEvent,
};
use crate::AIDefinitions;
use bevy::app::Update;
use bevy::ecs::schedule::{InternedScheduleLabel, ScheduleLabel};
use bevy::prelude::{
    IntoSystemConfigs, IntoSystemSetConfigs, Plugin, Resource, SystemSet,
};

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum UtilityAISet {
    Prepare,
    CalculateInputs,
    MakeDecisions,
    UpdateActions,
    Tidyup,
}

#[derive(Resource)]
pub(crate) struct UtilityAISettings {
    pub(crate) default_schedule: InternedScheduleLabel,
}

pub struct UtilityAIPlugin {
    schedule: InternedScheduleLabel,
}

impl UtilityAIPlugin {
    pub fn new(schedule: impl ScheduleLabel) -> Self {
        UtilityAIPlugin {
            schedule: schedule.intern(),
        }
    }
}

impl Default for UtilityAIPlugin {
    fn default() -> Self {
        UtilityAIPlugin {
            schedule: Update.intern(),
        }
    }
}

impl Plugin for UtilityAIPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_event::<UpdateEntityActionInternalEvent>()
            .add_event::<EntityActionChangedEvent>()
            .insert_resource(UtilityAISettings {
                default_schedule: self.schedule,
            })
            .init_resource::<AIDefinitions>()
            .init_resource::<AddedSystemTracker>()
            .add_systems(
                self.schedule,
                (
                    make_decisions_sys.in_set(UtilityAISet::MakeDecisions),
                    update_actions_sys.in_set(UtilityAISet::UpdateActions),
                ),
            )
            .configure_sets(
                self.schedule,
                (
                    UtilityAISet::CalculateInputs.before(UtilityAISet::MakeDecisions),
                    UtilityAISet::MakeDecisions.before(UtilityAISet::UpdateActions),
                ),
            );

        if cfg!(debug_assertions) {
            app.add_event::<InputCalculatedEvent>()
                .add_event::<ConsiderationCalculatedEvent>()
                .add_event::<DecisionCalculatedEvent>();
        }
    }
}
