use crate::dashboard::data::{
    record_consideration_scores, record_decision_scores, record_input_scores,
    sync_dashboard_data, DashboardData,
};
use crate::dashboard::view;
use crate::dashboard::view::DashboardState;
use bevy::app::{App, Plugin, PreUpdate, Startup};
use bevy::prelude::{Commands, Component, IVec2, Update, Window, WindowPosition};
use bevy::window::WindowResolution;

pub struct UtilityAIDashboardPlugin;

#[derive(Component)]
pub struct UtilityAIDashboardWindow;

impl Plugin for UtilityAIDashboardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DashboardData>()
            .init_resource::<DashboardState>()
            .add_systems(Startup, create_new_window_system)
            .add_systems(PreUpdate, sync_dashboard_data)
            .add_systems(
                Update,
                (
                    view::layout,
                    record_input_scores,
                    record_consideration_scores,
                    record_decision_scores,
                ),
            );
    }
}

const SCREEN_WIDTH: usize = 2560;
const SCREEN_HEIGHT: usize = 1300;

fn create_new_window_system(mut commands: Commands) {
    commands.spawn((
        Window {
            title: "UtilityAI Dashboard".to_owned(),
            position: WindowPosition::new(IVec2::new(SCREEN_WIDTH as i32 / 2, 0)),
            resolution: WindowResolution::new(
                SCREEN_WIDTH as f32 / 2.0,
                SCREEN_HEIGHT as f32 / 2.0,
            ),
            ..Default::default()
        },
        UtilityAIDashboardWindow,
    ));
}
