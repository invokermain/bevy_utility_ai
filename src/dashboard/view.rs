use crate::dashboard::data::DashboardData;
use crate::dashboard::plugin::UtilityAIDashboardWindow;
use crate::dashboard::view_models::ViewAIDefinition;
use crate::dashboard::widgets::WorldWidgetSystemExt;
use bevy::ecs::world::{Mut, World};
use bevy::prelude::{Entity, Resource, With};
use bevy::utils::HashSet;
use bevy_egui::EguiContext;

use super::widgets::{EntitySelectPanel, HeaderPanel, ObserverPanel};

#[derive(Resource, Default)]
pub(crate) struct DashboardState {
    pub(crate) view_mode: ViewMode,
    pub(crate) selected_ai_definition: Option<ViewAIDefinition>,
    /// The selected entities on the UI
    pub(crate) selected_entities: HashSet<Entity>,
    pub(crate) paused: bool,
}

impl DashboardState {
    pub(crate) fn reset(&mut self) {
        self.selected_entities = HashSet::new();
    }
}

pub(crate) fn layout(
    world: &mut World,
    // mut egui_ctx: Query<&mut EguiContext, With<UtilityAIDashboardWindow>>,
    // dashboard_data: Res<DashboardData>,
    // mut dashboard_state: ResMut<DashboardState>,
) {
    if world
        .query_filtered::<&mut EguiContext, With<UtilityAIDashboardWindow>>()
        .get_single(world)
        .is_err()
    {
        return;
    }

    // autoselect values if we have none
    world.resource_scope(|world, mut dashboard_state: Mut<DashboardState>| {
        let dashboard_data = world.resource::<DashboardData>();
        if dashboard_state.selected_ai_definition.is_none() {
            if let Some(ai_definition) = &dashboard_data.ai_definitions.first() {
                dashboard_state.selected_ai_definition = Some(ViewAIDefinition {
                    id: ai_definition.id,
                    name: ai_definition.name.clone(),
                });
            }
        }
        if dashboard_state.selected_entities.is_empty() {
            if let Some(entity) = dashboard_data.entities.first() {
                dashboard_state.selected_entities.insert(*entity);
            }
        }
    });

    world.root_widget_with::<HeaderPanel>("header_panel", ());
    world.root_widget_with::<EntitySelectPanel>("entity_select_panel", ());
    world.root_widget_with::<ObserverPanel>("observer_panel", ());
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Copy)]
pub(crate) enum ViewMode {
    #[default]
    Decisions,
    Considerations,
    Inputs,
    ResponseCurves,
}
