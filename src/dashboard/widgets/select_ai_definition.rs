use crate::dashboard::data::DashboardData;

use crate::dashboard::view::DashboardState;
use crate::dashboard::view_models::ViewAIDefinition;
use bevy_egui::egui;
use bevy_egui::egui::Ui;

pub(crate) fn select(
    ui: &mut Ui,
    dashboard_data: &DashboardData,
    dashboard_state: &mut DashboardState,
) {
    egui::ComboBox::from_label("AI Definition")
        .selected_text(match &dashboard_state.selected_ai_definition {
            None => "None",
            Some(def) => &def.name,
        })
        .show_ui(ui, |ui| {
            let selected_id = dashboard_state
                .selected_ai_definition
                .as_ref()
                .map(|f| f.id);

            for ViewAIDefinition {
                id: option_id,
                name: option_name,
            } in &dashboard_data.ai_definitions
            {
                if ui
                    .add(egui::SelectableLabel::new(
                        Some(*option_id) == selected_id,
                        option_name,
                    ))
                    .clicked()
                {
                    dashboard_state.selected_ai_definition = Some(ViewAIDefinition {
                        id: *option_id,
                        name: option_name.clone(),
                    });
                }
            }
        });
}
