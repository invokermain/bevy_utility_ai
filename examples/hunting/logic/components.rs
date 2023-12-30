use bevy::prelude::Component;

#[derive(Component)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}
