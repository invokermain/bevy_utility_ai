use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::entity_filter::EntityFilterList;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

#[derive(SystemParam)]
pub(crate) struct EntitySelectPanel {}

impl RootWidgetSystem for EntitySelectPanel {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        _state: &mut SystemState<Self>,
        ctx: &mut Context,
        _args: Self::Args,
    ) -> Self::Output {
        egui::SidePanel::left("side_panel")
            .default_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Entities");
                ui.add_system_with::<EntityFilterList>(world, "entity_filter_list", ());
            });
    }
}
