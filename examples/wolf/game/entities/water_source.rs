use bevy::ecs::bundle::Bundle;
use bevy::prelude::{default, Transform};

use crate::game::systems::water::Water;

#[derive(Bundle, Default)]
pub struct WaterSourceBundle {
    water: Water,
    transform: Transform,
}

impl WaterSourceBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            ..default()
        }
    }
}
