use crate::dashboard::data::{DashboardData, GRAPH_HISTORY_SIZE};
use crate::dashboard::view::DashboardState;
use bevy_egui::egui::Ui;
use egui_plot::{Corner, Legend, Line, Plot, PlotPoints};

pub(crate) fn plot(
    ui: &mut Ui,
    dashboard_data: &DashboardData,
    dashboard_state: &DashboardState,
) {
    let input_scores_plot = Plot::new("input_scores")
        .legend(Legend::default().position(Corner::LeftTop))
        .auto_bounds_x()
        .auto_bounds_y()
        .include_x(GRAPH_HISTORY_SIZE as f64)
        .auto_bounds_y()
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false);

    input_scores_plot.show(ui, |plot_ui| {
        for entity in &dashboard_state.selected_entities {
            if let Some(scores) = dashboard_data.input_scores.get(entity) {
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
