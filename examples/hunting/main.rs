mod ai;
mod components;
mod systems;

use crate::ai::{construct_hunter_ai, ActionHunt, ActionRest, HunterAI};
use crate::components::Energy;
use crate::systems::{hunt, rest};
use bevy::app::ScheduleRunnerPlugin;
use bevy::core_pipeline::clear_color::ClearColorConfig;
use bevy::gizmos::GizmoPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::render::camera::{ScalingMode, Viewport};
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy_utility_ai::plugin::UtilityAIPlugin;
use std::time::Duration;

fn main() {
    let mut app = App::new();

    // Setup the App
    app.add_plugins((
        ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1.0)),
        DefaultPlugins
            .set(LogPlugin {
                // can change bevy_utility_ai to debug to see whats happening under the hood
                filter: "warn,bevy_utility_ai=info".into(),
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
    ));

    app.add_systems(Startup, setup);

    // Add our plugin
    app.add_plugins(UtilityAIPlugin::new(Update));

    // Register our actions
    app.register_type::<ActionHunt>();
    app.register_type::<ActionRest>();

    // Add our AI logic
    construct_hunter_ai(&mut app);

    // Add our Action systems
    app.add_systems(Update, (hunt, rest));

    // Spawn our hunter
    app.world.spawn((
        HunterAI {},
        Energy {
            value: 100.0,
            max: 100.0,
        },
    ));

    app.run()
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gizmos: Gizmos,
) {
    // Camera
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            clear_color: ClearColorConfig::Custom(Color::LIME_GREEN),
        },
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::Fixed { width: 255., height: 255. },
            ..default()
        },
        transform: Transform::from_xyz(127.5, 127.5, 0.),
        ..default()
    });

    println!("{:?}", Camera2dBundle::default().projection.area);

    gizmos.line_2d(Vec2::ZERO, Vec2::new(255., 255.), Color::BLUE);

    // Spawn some prey
    commands.spawn_batch(vec![MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
        ..default()
    }]);
}
