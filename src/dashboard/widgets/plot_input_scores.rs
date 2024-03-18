use crate::dashboard::data::{DashboardData, GRAPH_HISTORY_SIZE};
use crate::dashboard::view::DashboardState;
use bevy::ecs::system::{Res, SystemParam, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui::{Ui, Vec2b};
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};

use super::base::WidgetSystem;

#[derive(SystemParam)]
pub(crate) struct InputScoresPlot<'w> {
    dashboard_data: Res<'w, DashboardData>,
    dashboard_state: Res<'w, DashboardState>,
}

impl<'w> WidgetSystem for InputScoresPlot<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        _args: Self::Args,
    ) -> Self::Output {
        let InputScoresPlot {
            dashboard_data,
            dashboard_state,
        } = state.get(world);
        let input_scores_plot = Plot::new("input_scores")
            .legend(Legend::default().position(Corner::LeftTop))
            .auto_bounds(Vec2b::TRUE)
            .include_x(GRAPH_HISTORY_SIZE as f64)
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false);

        input_scores_plot.show(ui, |plot_ui| {
            for entity in &dashboard_state.selected_entities {
                if let Some(scores) = dashboard_data.entity_input_scores.get(entity) {
                    for ((input, target), scores_vec) in scores {
                        let name = match target {
                            None => input.to_string(),
                            Some(target) => format! {"{} - {:?}", input, target},
                        };
                        plot_ui.line(
                            Line::new(PlotPoints::from_ys_f32(scores_vec.as_slices().0))
                                .name(name),
                        )
                    }
                }
            }
        });
    }
}
