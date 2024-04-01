use std::collections::VecDeque;

use crate::dashboard::data::DashboardData;
use crate::dashboard::view::DashboardState;
use crate::response_curves::InputTransform;
use crate::AIDefinitions;
use bevy::ecs::system::{Local, Res, SystemParam, SystemState};
use bevy::ecs::world::World;
use bevy_egui::egui::{self, Ui};
use egui_plot::{Bar, BarChart, Line, Plot, PlotBounds, PlotPoints};

use super::base::WidgetSystem;

#[derive(SystemParam)]
pub(crate) struct DecisionView<'w, 's> {
    ai_definitions: Res<'w, AIDefinitions>,
    dashboard_state: Res<'w, DashboardState>,
    dashboard_data: Res<'w, DashboardData>,
    decision_idx: Local<'s, usize>,
    consideration_idx: Local<'s, usize>,
}

impl<'w, 's> WidgetSystem for DecisionView<'w, 's> {
    type Args = ();
    type Output = ();

    fn system(
        world: &mut World,
        state: &mut SystemState<Self>,
        ui: &mut Ui,
        _args: Self::Args,
    ) -> Self::Output {
        let DecisionView {
            ai_definitions,
            dashboard_state,
            dashboard_data,
            mut decision_idx,
            mut consideration_idx,
        } = state.get_mut(world);

        if dashboard_state.selected_ai_definition.is_none() {
            return;
        }

        let selected_ai_definition =
            dashboard_state.selected_ai_definition.as_ref().unwrap().id;

        let decisions = &ai_definitions
            .map
            .get(&selected_ai_definition)
            .unwrap()
            .decisions;

        if *decision_idx > decisions.len() - 1 {
            *decision_idx = 0;
        }

        ui.horizontal(|ui| {
            for (idx, decision) in decisions.iter().enumerate() {
                if ui
                    .add(egui::SelectableLabel::new(
                        idx == *decision_idx,
                        decision.name.clone(),
                    ))
                    .clicked()
                {
                    *consideration_idx = 0;
                    *decision_idx = idx;
                }
            }
        });

        let decision = &decisions[*decision_idx];

        ui.label(format!("base score: {}", decision.base_score));

        if decision.considerations.is_empty() {
            return;
        }

        ui.horizontal(|ui| {
            for (idx, consideration) in decision.considerations.iter().enumerate() {
                if ui
                    .add(egui::SelectableLabel::new(
                        idx == *consideration_idx,
                        consideration.name.clone(),
                    ))
                    .clicked()
                {
                    *consideration_idx = idx;
                }
            }
        });

        let consideration = &decision.considerations[*consideration_idx];

        let plot = Plot::new(format!(
            "curve_plot-{}-{}",
            decision.name, consideration.name
        ))
        .allow_drag(false)
        .allow_scroll(false)
        .allow_zoom(false);

        let response_curve = consideration.response_curve.clone();

        let mut input_values: Vec<f32> = Vec::from_iter(
            dashboard_data
                .input_scores
                .get(&consideration.input_name)
                .unwrap_or(&VecDeque::new())
                .clone(),
        );

        if input_values.is_empty() {
            return;
        }

        input_values.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let mut x_l = input_values[input_values.len() / 10] as f64;
        let mut x_u = input_values[(input_values.len() * 9) / 10] as f64;

        let base_unit = 5.0 * 10.0f64.powf(((x_u + x_l) / 2.0).log10().floor());

        x_l = (x_l / base_unit).floor() * base_unit;
        x_u = (x_u / base_unit).ceil() * base_unit;

        let points = PlotPoints::from_explicit_callback(
            move |x| (response_curve.transform(x as f32) as f64).clamp(0.0, 1.0),
            x_l..=x_u,
            50,
        );

        // calculate histogram
        let histogram = generate_histogram(&input_values);

        plot.show(ui, |plot_ui| {
            plot_ui.set_plot_bounds(PlotBounds::from_min_max([x_l, 0.0], [x_u, 1.01]));
            plot_ui.line(Line::new(points));
            if let Some((histogram, bar_size)) = histogram {
                plot_ui.bar_chart(BarChart::new(Vec::from_iter(
                    histogram
                        .iter()
                        .map(|h| Bar::new(h.0, h.1).width(bar_size.into())),
                )));
            }
        });
    }
}

/// Generates a histogram, assumes that the values are sorted in ascending order
pub(super) fn generate_histogram(values: &Vec<f32>) -> Option<(Vec<(f64, f64)>, f32)> {
    // take 5th and 95th percentile to exclude anomalous values
    let min = values[(values.len() * 5) / 100];
    let max = values[(values.len() * 95) / 100];
    let step_size = (max - min) / 10.0;

    let mut buckets: Vec<(f32, u16)> =
        Vec::from_iter((1..=10).map(|step| (min + step as f32 * step_size, 0)));

    for input in values {
        let bucket_idx = ((*input - min) / step_size).min(9.0) as usize;
        buckets[bucket_idx].1 += 1;
    }

    Some((
        Vec::from_iter(buckets.iter().map(|b| {
            (
                (b.0 - step_size / 2.0) as f64,
                b.1 as f64 / values.len() as f64,
            )
        })),
        step_size,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_histogram() {
        let hist = generate_histogram(&Vec::from_iter((0..100).map(|x| x as f32 / 99.0)));
        let (bins, _bin_width) = hist.unwrap();

        for bin in &bins {
            assert!((bin.1 - 0.10).abs() < 0.01);
        }
    }
}
