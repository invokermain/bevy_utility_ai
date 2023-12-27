use bevy::prelude::Component;

// Define some components for us to work with
#[derive(Component)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}
