use super::base::WidgetSystem;
use crate::dashboard::data::DashboardData;
use crate::dashboard::view::DashboardState;
use crate::dashboard::view_models::ViewAIDefinition;
use bevy::ecs::system::{ResMut, SystemParam, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui;
use bevy_egui::egui::Ui;

#[derive(SystemParam)]
pub(crate) struct SelectAIDefinition<'w> {
    dashboard_data: ResMut<'w, DashboardData>,
    dashboard_state: ResMut<'w, DashboardState>,
}

impl<'w> WidgetSystem for SelectAIDefinition<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        entities: Self::Args,
    ) -> Self::Output {
        let SelectAIDefinition {
            mut dashboard_state,
            dashboard_data,
        } = state.get_mut(world);

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
}
