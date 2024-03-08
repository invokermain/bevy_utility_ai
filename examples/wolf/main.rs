use std::time::Duration;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::window::WindowResolution;
use bevy_ecs_ldtk::LdtkPlugin;
use bevy_egui::EguiPlugin;

use bevy_utility_ai::dashboard::UtilityAIDashboardPlugin;
use bevy_utility_ai::plugin::{UtilityAIPlugin, UtilityAISet};
use bevy_utility_ai::systems::make_decisions::EntityActionChangedEvent;
use game::entities::carrion::{despawn_eaten_meat, spawn_meat_on_kill};
use game::systems::drink::{drink, increase_thirst};
use game::systems::hunt::hunt;
use game::systems::rest::{idle, rest};
use ui::{
    energy_text_update_system, fps_text_update_system, hunger_text_update_system,
    setup_fps_counter, thirst_text_update_system,
};
use utils::animations::animate_sprite;

use crate::game::ai::wolf::construct_hunter_ai;
use crate::game::entities::birds::{
    bird_movement, load_bird_assets, spawn_birds_occasionally, BirdAssetHandles,
};
use crate::game::systems::food::{eat, increase_hunger};
use crate::game::systems::hunt::PreyKilledEvent;
use crate::game::systems::rest::insert_idle_behaviour;
use crate::level::WolfSceneSetupPlugin;
use crate::ui::action_text_update_system;

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

fn main() {
    let mut app = App::new();

    // Set up the App
    app.add_plugins((
        DefaultPlugins
            .set(LogPlugin {
                // can change bevy_utility_ai to debug to see what's happening under the hood
                filter: "warn,bevy_utility_ai=info,wolf=info".into(),
                level: bevy::log::Level::INFO,
                update_subscriber: None,
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
        log_ai_updated_action.in_set(UtilityAISet::UpdateActions),
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
                hunt,
                rest,
                increase_hunger,
                increase_thirst,
                eat,
                insert_idle_behaviour,
                idle,
                drink,
                despawn_eaten_meat,
                bird_movement,
            ),
            spawn_meat_on_kill,
        )
            .chain(),
    );
    app.init_resource::<BirdAssetHandles>();
    app.add_systems(Startup, load_bird_assets);
    app.add_systems(FixedUpdate, spawn_birds_occasionally);

    // Register our events
    app.add_event::<PreyKilledEvent>();

    app.run()
}
