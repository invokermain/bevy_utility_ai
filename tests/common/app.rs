use bevy::app::App;
use bevy::log::LogPlugin;
use bevy::utils::default;

pub fn test_app() -> App {
    let mut app = App::new();
    app.add_plugins(LogPlugin {
        filter: "wgpu=error".into(),
        level: bevy::log::Level::DEBUG,
        ..default()
    });
    app
}
