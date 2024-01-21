use crate::dashboard::data::DashboardData;
use crate::dashboard::view::{DashboardState, ViewMode};
use bevy::ecs::system::{Res, ResMut, SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::plot_consideration_scores::ConsiderationScoresPlot;

#[derive(SystemParam)]
pub(crate) struct ObserverPanel<'w> {
    dashboard_data: Res<'w, DashboardData>,
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

        let selected_entities = &state.dashboard_state.selected_entities;
        egui::CentralPanel::default().show(ctx, |ui| {
            match state.dashboard_state.view_mode {
                ViewMode::Decisions => {
                    ui.add_system_with::<ConsiderationScoresPlot>(
                        world,
                        "consideration_scores_plot",
                        selected_entities,
                    );
                }
                ViewMode::Considerations => {
                    ui.add_system_with::<ConsiderationScoresPlot>(
                        world,
                        "consideration_scores_plot",
                        &selected_entities,
                    );
                }
                ViewMode::Inputs => {
                    ui.add_system_with::<ConsiderationScoresPlot>(
                        world,
                        "consideration_scores_plot",
                        &selected_entities,
                    );
                }
            }
        });
    }
}
