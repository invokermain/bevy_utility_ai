use crate::dashboard::view::{DashboardState, ViewMode};
use bevy::ecs::system::{Res, SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::plot_consideration_scores::ConsiderationScoresPlot;
use super::plot_decision_scores::DecisionScoresPlot;
use super::plot_input_scores::InputScoresPlot;
use super::view_decision::DecisionView;

#[derive(SystemParam)]
pub(crate) struct ObserverPanel<'w> {
    dashboard_state: Res<'w, DashboardState>,
}

impl<'w> RootWidgetSystem for ObserverPanel<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ctx: &mut Context,
        _args: Self::Args,
    ) -> Self::Output {
        let state = state.get(world);
        let view_mode = state.dashboard_state.view_mode;

        egui::CentralPanel::default().show(ctx, |ui| match view_mode {
            ViewMode::Decisions => {
                ui.add_system_with::<DecisionScoresPlot>(
                    world,
                    "decision_scores_plot",
                    (),
                );
            }
            ViewMode::Considerations => {
                ui.add_system_with::<ConsiderationScoresPlot>(
                    world,
                    "consideration_scores_plot",
                    (),
                );
            }
            ViewMode::Inputs => {
                ui.add_system_with::<InputScoresPlot>(world, "input_scores_plot", ());
            }
            ViewMode::ResponseCurves => {
                ui.add_system_with::<DecisionView>(world, "decision_view", ());
            }
        });
    }
}
