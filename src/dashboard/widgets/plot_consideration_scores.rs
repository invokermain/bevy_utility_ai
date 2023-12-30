use crate::dashboard::data::{DashboardData, GRAPH_HISTORY_SIZE};
use crate::dashboard::view::DashboardState;
use bevy_egui::egui::Ui;
use egui_plot::{Corner, Legend, Line, Plot, PlotBounds, PlotPoints};

pub(crate) fn plot(
    ui: &mut Ui,
    dashboard_data: &DashboardData,
    dashboard_state: &DashboardState,
) {
    let plot = Plot::new("consideration_scores")
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
            if let Some(scores) = dashboard_data.consideration_scores.get(entity) {
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
