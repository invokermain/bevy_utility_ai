mod camera;
mod game;
mod layers;
mod ui;

use crate::game::ai::hunter::construct_hunter_ai;
use crate::game::systems::food::{decrement_food_eaters, eat, increase_hunger};
use crate::game::systems::hunt::PreyKilledEvent;
use crate::ui::action_text_update_system;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_egui::EguiPlugin;
use bevy_utility_ai::dashboard::UtilityAIDashboardPlugin;
use bevy_utility_ai::plugin::{UtilityAIPlugin, UtilityAISet};
use bevy_utility_ai::systems::make_decisions::EntityActionChangedEvent;
use camera::{mouse_control, scroll_zoom};
use game::ai::prey::construct_prey_ai;
use game::entities::carrion::{despawn_eaten_carrion, spawn_carrion_on_kill};
use game::entities::grass::GrassBundle;
use game::entities::grass::{hide_eaten_grass, regrow_grass};
use game::entities::hunter::HunterBundle;
use game::entities::water_source::WaterSourceBundle;
use game::systems::hunt::hunt;
use game::systems::prey::{flee, herd, remove_flee_to, PreyBundle};
use game::systems::rest::{idle, rest};
use game::systems::water::{drink, increase_thirst};
use rand::Rng;
use std::time::Duration;
use ui::{
    draw_fence, energy_text_update_system, fps_text_update_system,
    hunger_text_update_system, setup_fps_counter, thirst_text_update_system,
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
                update_subscriber: None,
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800., 800.),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin {
                file_path: "examples/hunting/assets".to_string(),
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
            thirst_text_update_system,
            draw_fence,
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
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(0.05)));

    // Add the Utility AI plugins, uncomment the extra plugins to see the dashboard
    app.add_plugins((
        UtilityAIPlugin::new(FixedUpdate),
        EguiPlugin,
        UtilityAIDashboardPlugin,
    ));

    // Add our AI logic
    construct_hunter_ai(&mut app);
    construct_prey_ai(&mut app);

    // Add our game systems
    app.add_systems(
        FixedUpdate,
        (
            (
                hunt,
                rest,
                increase_hunger,
                increase_thirst,
                spawn_carrion_on_kill,
                eat,
                idle,
                flee,
                drink,
                herd,
                despawn_eaten_carrion,
                hide_eaten_grass,
                regrow_grass,
                decrement_food_eaters,
            ),
            (remove_flee_to,),
        )
            .chain(),
    );

    // Register our events
    app.add_event::<PreyKilledEvent>();

    // Spawn some entities
    app.add_systems(Startup, worldgen);

    app.run()
}

fn worldgen(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // Spawn our hunter
    commands.spawn(HunterBundle::new(Vec2::ZERO, asset_server));

    // Spawn some prey
    commands.spawn_batch(
        (0..25)
            .map(|_| PreyBundle::new(&mut meshes, &mut materials))
            .collect::<Vec<_>>(),
    );

    // Spawn two water sources
    commands.spawn_batch(vec![
        WaterSourceBundle::new(Vec2::new(250., 250.), &mut meshes, &mut materials),
        WaterSourceBundle::new(Vec2::new(-250., 250.), &mut meshes, &mut materials),
        WaterSourceBundle::new(Vec2::new(250., -250.), &mut meshes, &mut materials),
        WaterSourceBundle::new(Vec2::new(-250., -250.), &mut meshes, &mut materials),
    ]);

    // Spawn some grass
    let mut rng = rand::thread_rng();
    commands.spawn_batch(
        (0..100)
            .map(|_| {
                Vec2::new(
                    rng.gen_range(-1000.0..=1000.0),
                    rng.gen_range(-1000.0..=1000.0),
                )
            })
            // filter out grass near water
            .filter(|pos| {
                !(Rect::new(-300., -300., -200., -200.).contains(*pos)
                    || Rect::new(200., 200., 300., 300.).contains(*pos)
                    || Rect::new(-200., 200., -300., 300.).contains(*pos)
                    || Rect::new(200., -200., 300., -300.).contains(*pos))
            })
            .map(|pos| GrassBundle::new(pos, &mut meshes, &mut materials))
            .collect::<Vec<_>>(),
    );
}
