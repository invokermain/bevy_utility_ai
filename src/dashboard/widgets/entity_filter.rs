use super::base::WidgetSystem;
use crate::dashboard::data::DashboardData;
use crate::dashboard::view::DashboardState;
use bevy::ecs::system::{ResMut, SystemParam, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui;
use bevy_egui::egui::Ui;

#[derive(SystemParam)]
pub(crate) struct EntityFilterList<'w> {
    dashboard_data: ResMut<'w, DashboardData>,
    dashboard_state: ResMut<'w, DashboardState>,
}

impl<'w> WidgetSystem for EntityFilterList<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        _: Self::Args,
    ) -> Self::Output {
        let mut state = state.get_mut(world);
        for entity in &state.dashboard_data.entities {
            if ui
                .add(egui::SelectableLabel::new(
                    state.dashboard_state.selected_entities.contains(entity),
                    format!("{:?}", entity),
                ))
                .clicked()
            {
                state.dashboard_state.selected_entities.insert(*entity);
            }
        }
    }
}
