mod camera;
mod logic;
mod ui;

use crate::logic::ai::{construct_hunter_ai, HunterAI, PreyAI};
use crate::logic::components::Energy;
use crate::logic::food::{despawn_empty_food, eat, increase_hunger, spawn_food, Hunger};
use crate::logic::hunt::PreyKilledEvent;
use crate::ui::action_text_update_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;
use bevy_utility_ai::dashboard::UtilityAIDashboardPlugin;
use bevy_utility_ai::plugin::{UtilityAIPlugin, UtilityAISet};
use bevy_utility_ai::systems::make_decisions::EntityActionChangedEvent;
use camera::{mouse_control, scroll_zoom};
use logic::hunt::hunt;
use logic::rest::{idle, rest};
use rand::Rng;
use std::time::Duration;
use ui::{
    energy_text_update_system, fps_text_update_system, hunger_text_update_system,
    setup_fps_counter,
};

// This system listens to EntityActionChangedEvent events and logs them to give us some
// visibility.
fn log_ai_updated_action(mut e_update_action: EventReader<EntityActionChangedEvent>) {
    for event in e_update_action.read() {
        info!(
            "entity {:?} | decided {} | target {:?} | score {:.2}",
            event.entity_id, event.new_action, event.new_target, event.new_score,
        )
    }
}

fn main() {
    let mut app = App::new();

    // Setup the App
    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                // can change bevy_utility_ai to debug to see whats happening under the hood
                filter: "warn,bevy_utility_ai=info,hunting=info".into(),
                level: bevy::log::Level::INFO,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    fit_canvas_to_parent: true,
                    resolution: WindowResolution::new(800., 800.),
                    resizable: false,
                    ..default()
                }),
                ..default()
            }),
        FrameTimeDiagnosticsPlugin,
    ));

    // Setup the camera, ui and other game systems
    app.add_systems(Startup, (camera::setup_camera, setup_fps_counter));
    app.add_systems(
        Update,
        (
            mouse_control,
            scroll_zoom,
            fps_text_update_system,
            action_text_update_system,
            hunger_text_update_system,
            energy_text_update_system,
        ),
    );

    // Add some logging observability, we use the UpdateActions system set to ensure
    // that is runs after any decisions have been made.
    app.add_systems(
        Update,
        log_ai_updated_action.in_set(UtilityAISet::UpdateActions),
    );

    // So we don't have to deal with time deltas in this example use a fixed timestep for
    // our systems. In general there is no need for the Utility AI systems to run every
    // tick.
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(0.1)));

    // Add the Utility AI plugins
    app.add_plugins((
        UtilityAIPlugin::new(FixedUpdate),
        EguiPlugin,
        UtilityAIDashboardPlugin,
    ));

    // Add our AI logic
    construct_hunter_ai(&mut app);

    // Add our game systems
    app.add_systems(
        FixedUpdate,
        (
            hunt,
            rest,
            increase_hunger,
            spawn_food,
            despawn_empty_food,
            eat,
            idle,
        ),
    );

    // Spawn some entities
    app.add_systems(Startup, worldgen);

    // Register our events
    app.add_event::<PreyKilledEvent>();

    app.run()
}

fn worldgen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Spawn some prey
    let prey_material = materials.add(ColorMaterial::from(Color::PURPLE));
    let pixel_mesh = meshes.add(shape::Box::new(5., 5., 0.).into());

    let mut rng = rand::thread_rng();

    commands.spawn_batch(
        (0..10)
            .map(|_| {
                (
                    MaterialMesh2dBundle {
                        mesh: pixel_mesh.clone().into(),
                        material: prey_material.clone(),
                        transform: Transform::from_translation(Vec3::new(
                            rng.gen_range(-1000.0..=1000.0),
                            rng.gen_range(-1000.0..=1000.0),
                            0.,
                        )),
                        ..default()
                    },
                    Energy {
                        value: 100.,
                        max: 100.,
                    },
                    PreyAI {},
                )
            })
            .collect::<Vec<_>>(),
    );

    // Spawn our hunter
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: pixel_mesh.clone().into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::Z),
            ..default()
        },
        Energy {
            value: 100.0,
            max: 100.0,
        },
        HunterAI {}, // this component enables the HunterAI behaviour
        Hunger {
            value: 50.0,
            max: 100.0,
        },
    ));
}
