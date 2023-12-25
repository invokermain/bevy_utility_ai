use bevy::app::App;
use bevy::prelude::Update;
use bevy_utility_ai::dashboard::UtilityAIDashboardPlugin;
use bevy_utility_ai::plugin::UtilityAIPlugin;

#[test]
fn test_dashboard() {
    let mut app = App::new();

    app.add_plugins((UtilityAIDashboardPlugin, UtilityAIPlugin::new(Update)));

    app.update();
}
