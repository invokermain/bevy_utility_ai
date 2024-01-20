use crate::dashboard::data::DashboardData;
use crate::dashboard::plugin::UtilityAIDashboardWindow;
use crate::dashboard::view_models::ViewAIDefinition;
use crate::dashboard::widgets::{
    entity_filter, plot_consideration_scores, plot_decision_scores, plot_input_scores,
    select_ai_definition,
};
use bevy::prelude::{Entity, Query, Res, ResMut, Resource, With};
use bevy::utils::HashSet;
use bevy_egui::{egui, EguiContext};

#[derive(Resource, Default)]
pub(crate) struct DashboardState {
    pub(crate) view_mode: ViewMode,
    pub(crate) selected_ai_definition: Option<ViewAIDefinition>,
    /// The selected entities on the UI
    pub(crate) selected_entities: HashSet<Entity>,
    pub(crate) paused: bool,
}

impl DashboardState {
    pub(crate) fn reset(&mut self, _dashboard_data: &DashboardData) {
        self.selected_entities = HashSet::new();
    }
}

pub(crate) fn layout(
    mut egui_ctx: Query<&mut EguiContext, With<UtilityAIDashboardWindow>>,
    dashboard_data: Res<DashboardData>,
    mut dashboard_state: ResMut<DashboardState>,
) {
    let Ok(mut ctx) = egui_ctx.get_single_mut() else {
        return;
    };

    // autoselect values if we have none
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

    egui::TopBottomPanel::top("top_panel")
        .min_height(32.0)
        .show(ctx.get_mut(), |ui| {
            ui.horizontal(|ui| {
                select_ai_definition(ui, &dashboard_data, &mut dashboard_state);
                ui.horizontal(|ui| {
                    ui.selectable_value(
                        &mut dashboard_state.view_mode,
                        ViewMode::Decisions,
                        "Decisions",
                    );
                    ui.selectable_value(
                        &mut dashboard_state.view_mode,
                        ViewMode::Considerations,
                        "Considerations",
                    );
                    ui.selectable_value(
                        &mut dashboard_state.view_mode,
                        ViewMode::Inputs,
                        "Inputs",
                    );
                });
            });
            let pause_button = ui.button(match dashboard_state.paused {
                true => "Resume",
                false => "Pause",
            });
            if pause_button.clicked() {
                dashboard_state.paused = !dashboard_state.paused;
            };
        });

    egui::SidePanel::left("side_panel")
        .default_width(200.0)
        .show(ctx.get_mut(), |ui| {
            ui.heading("Entities");
            entity_filter(ui, &dashboard_data, &mut dashboard_state)
        });

    egui::CentralPanel::default().show(ctx.get_mut(), |ui| {
        match dashboard_state.view_mode {
            ViewMode::Decisions => {
                plot_decision_scores(ui, &dashboard_data, &dashboard_state)
            }
            ViewMode::Considerations => {
                plot_consideration_scores(ui, &dashboard_data, &dashboard_state)
            }
            ViewMode::Inputs => plot_input_scores(ui, &dashboard_data, &dashboard_state),
        }
    });
}

#[derive(Eq, PartialEq, Debug, Default)]
pub(crate) enum ViewMode {
    #[default]
    Decisions,
    Considerations,
    Inputs,
}
