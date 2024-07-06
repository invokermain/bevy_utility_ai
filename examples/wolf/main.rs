use std::time::Duration;

use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin, log::LogPlugin, prelude::*,
    window::WindowResolution,
};
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_egui::EguiPlugin;

use bevy_utility_ai::{
    dashboard::UtilityAIDashboardPlugin,
    plugin::{UtilityAIPlugin, UtilityAISet},
    systems::make_decisions::EntityActionChangedEvent,
};
use game::{
    entities::{
        carrion::{despawn_eaten_meat, spawn_meat_on_kill},
        wolf::clear_wolf_text,
    },
    systems::{
        drink::{drink, increase_thirst},
        hunt::hunt,
        pathfinding::{assign_path, follow_path, PathRequested},
        rest::{consume_energy, idle, on_idle_removed, on_rest_removed, rest},
    },
};
use ui::{
    energy_text_update_system, fps_text_update_system, hunger_text_update_system,
    setup_fps_counter, thirst_text_update_system,
};
use utils::animations::animate_sprite;

use crate::{
    game::{
        ai::wolf::construct_hunter_ai,
        entities::birds::{
            bird_movement, load_bird_assets, spawn_birds_occasionally, BirdAssetHandles,
        },
        systems::{
            food::{eat, increase_hunger},
            hunt::PreyKilledEvent,
            rest::insert_idle_behaviour,
        },
    },
    level::WolfSceneSetupPlugin,
    ui::action_text_update_system,
};

mod game;
mod layers;
mod level;
mod ui;
mod utils;

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

fn main() -> AppExit {
    let mut app = App::new();

    // Set up the App
    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                // can change bevy_utility_ai to debug to see what's happening under the hood
                filter: "warn,bevy_utility_ai=info,wolf=debug".into(),
                level: bevy::log::Level::INFO,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resolution: WindowResolution::new(800.0, 800.0),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest())
            .set(AssetPlugin {
                file_path: "examples/wolf/assets".to_string(),
                ..default()
            }),
        LdtkPlugin,
        WolfSceneSetupPlugin,
        FrameTimeDiagnosticsPlugin,
    ));

    // Set up the camera, level, ui and other general stuff
    app.add_systems(Startup, setup_fps_counter);
    app.add_systems(
        Update,
        (
            fps_text_update_system,
            action_text_update_system,
            hunger_text_update_system,
            energy_text_update_system,
            thirst_text_update_system,
            animate_sprite,
        ),
    );

    // Add some logging observability, we use the UpdateActions system set to ensure
    // that is runs after any decisions have been made.
    app.add_systems(
        Update,
        (
            log_ai_updated_action.in_set(UtilityAISet::UpdateActions),
            clear_wolf_text.in_set(UtilityAISet::UpdateActions),
        ),
    );

    // Run the Utility AI systems every 0.25 seconds.
    app.insert_resource(Time::<Fixed>::from_duration(Duration::from_secs_f64(0.25)));

    // Add the Utility AI plugins
    app.add_plugins((
        UtilityAIPlugin::new(FixedUpdate), // Add the plugin with a FixedUpdate schedule
        EguiPlugin,                        // Required for the dasboard
        UtilityAIDashboardPlugin, // A dashboard for visualising what the AI is doing
    ));

    // Add our AI logic, this will add all the required systems for you
    construct_hunter_ai(&mut app);

    // Add our game systems
    app.add_systems(
        Update,
        (
            (
                on_rest_removed,
                on_idle_removed,
                insert_idle_behaviour,
                despawn_eaten_meat,
            ),
            (
                hunt,
                rest,
                consume_energy,
                increase_hunger,
                increase_thirst,
                eat,
                idle,
                drink,
                bird_movement,
                follow_path,
            ),
            (spawn_meat_on_kill, assign_path),
        )
            .chain(),
    );
    app.init_resource::<BirdAssetHandles>();
    app.add_systems(Startup, load_bird_assets);
    app.add_systems(FixedUpdate, spawn_birds_occasionally);

    // Register our events
    app.add_event::<PreyKilledEvent>()
        .add_event::<PathRequested>();

    app.run()
}
