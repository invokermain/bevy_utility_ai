use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::select_ai_definition::SelectAIDefinition;
use crate::dashboard::data::DashboardData;
use crate::dashboard::view::{DashboardState, ViewMode};
use bevy::ecs::system::{Res, SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

#[derive(SystemParam)]
pub(crate) struct HeaderPanel<'w> {
    dashboard_data: Res<'w, DashboardData>,
    dashboard_state: Res<'w, DashboardState>,
}

impl<'w> RootWidgetSystem for HeaderPanel<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ctx: &mut Context,
        _args: Self::Args,
    ) -> Self::Output {
        let HeaderPanel {
            dashboard_data,
            dashboard_state,
        } = state.get(world);

        egui::TopBottomPanel::top("top_panel")
            .min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_system_with::<SelectAIDefinition>(
                        world,
                        "select_ai_definition",
                        (),
                    );
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
    }
}
