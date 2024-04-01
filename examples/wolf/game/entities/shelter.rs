use bevy::ecs::bundle::Bundle;
use bevy::prelude::{default, Transform};

use crate::game::systems::rest::Shelter;

#[derive(Bundle, Default)]
pub struct ShelterBundle {
    shelter: Shelter,
    transform: Transform,
}

impl ShelterBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            transform,
            ..default()
        }
    }
}
