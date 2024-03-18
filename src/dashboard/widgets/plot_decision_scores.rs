use crate::dashboard::data::{DashboardData, GRAPH_HISTORY_SIZE};
use crate::dashboard::view::DashboardState;
use bevy::ecs::system::{Res, SystemParam, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui::Ui;
use egui_plot::{Corner, Legend, Line, Plot, PlotBounds, PlotPoints};

use super::base::WidgetSystem;

#[derive(SystemParam)]
pub(crate) struct DecisionScoresPlot<'w> {
    dashboard_data: Res<'w, DashboardData>,
    dashboard_state: Res<'w, DashboardState>,
}

impl<'w> WidgetSystem for DecisionScoresPlot<'w> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        _args: Self::Args,
    ) -> Self::Output {
        let DecisionScoresPlot {
            dashboard_data,
            dashboard_state,
        } = state.get(world);
        let plot = Plot::new("decision_scores")
            .legend(Legend::default().position(Corner::LeftTop))
            .allow_drag(false)
            .allow_scroll(false)
            .allow_zoom(false);

        plot.show(ui, |plot_ui| {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max(
                [0.0, 0.0],
                [GRAPH_HISTORY_SIZE as f64, 1.01],
            ));
            for entity in &dashboard_state.selected_entities {
                if let Some(scores) = dashboard_data.decision_scores.get(entity) {
                    for ((decision, target), scores_vec) in scores {
                        let name = match target {
                            None => decision.to_string(),
                            Some(target) => format! {"{} - {:?}", decision, target},
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
