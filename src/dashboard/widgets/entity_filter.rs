use crate::dashboard::data::DashboardData;
use crate::dashboard::view::DashboardState;
use bevy_egui::egui;
use bevy_egui::egui::Ui;

pub(crate) fn entity_filter(
    ui: &mut Ui,
    dashboard_data: &DashboardData,
    dashboard_state: &mut DashboardState,
) {
    for entity in &dashboard_data.entities {
        if ui
            .add(egui::SelectableLabel::new(
                dashboard_state.selected_entities.contains(entity),
                format!("{:?}", entity),
            ))
            .clicked()
        {
            dashboard_state.selected_entities.insert(*entity);
        }
    }
}
