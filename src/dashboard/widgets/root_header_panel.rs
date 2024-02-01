use super::base::{RootWidgetSystem, UiWidgetSystemExt};
use super::select_ai_definition::SelectAIDefinition;
use super::select_view_mode::SelectViewMode;
use bevy::ecs::system::{SystemParam, SystemState};
use bevy::prelude::World;
use bevy_egui::egui;
use bevy_egui::egui::Context;

#[derive(SystemParam)]
pub(crate) struct HeaderPanel {}

impl RootWidgetSystem for HeaderPanel {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        _state: &mut SystemState<Self>,
        ctx: &mut Context,
        _args: Self::Args,
    ) -> Self::Output {
        egui::TopBottomPanel::top("top_panel")
            .min_height(32.0)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.add_system_with::<SelectAIDefinition>(
                        world,
                        "select_ai_definition",
                        (),
                    );
                    ui.add_system_with::<SelectViewMode>(world, "select_view_mode", ());
                });
            });
    }
}
