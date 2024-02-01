use super::base::WidgetSystem;
use crate::dashboard::view::{DashboardState, ViewMode};
use bevy::ecs::system::SystemParam;
use bevy::ecs::system::{ResMut, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui::Ui;

#[derive(SystemParam)]
pub(crate) struct SelectViewMode<'w> {
    dashboard_state: ResMut<'w, DashboardState>,
}

impl<'w> WidgetSystem for SelectViewMode<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        _args: Self::Args,
    ) -> Self::Output {
        let SelectViewMode {
            mut dashboard_state,
            ..
        } = state.get_mut(world);

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
            ui.selectable_value(
                &mut dashboard_state.view_mode,
                ViewMode::ResponseCurves,
                "ResponseCurves",
            );
        });
        let pause_button = ui.button(match dashboard_state.paused {
            true => "Resume",
            false => "Pause",
        });
        if pause_button.clicked() {
            dashboard_state.paused = !dashboard_state.paused;
        };
    }
}
