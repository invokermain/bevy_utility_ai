use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::entity_filter::EntityFilterList;
use crate::dashboard::data::DashboardData;
use crate::dashboard::view::DashboardState;
use bevy::ecs::system::{Res, SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

#[derive(SystemParam)]
pub(crate) struct EntitySelectPanel<'w> {
    dashboard_data: Res<'w, DashboardData>,
    dashboard_state: Res<'w, DashboardState>,
}

impl<'w> RootWidgetSystem for EntitySelectPanel<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ctx: &mut Context,
        _args: Self::Args,
    ) -> Self::Output {
        let state = state.get(world);

        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Entities");
                ui.add_system_with::<EntityFilterList>(world, "entity_filter_list", ());
            });
    }
}
