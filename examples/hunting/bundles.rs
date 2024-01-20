use bevy::asset::Assets;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::ResMut;
use bevy::math::Vec2;
use bevy::prelude::default;
use bevy::render::color::Color;
use bevy::render::mesh::{shape, Mesh};
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy::transform::components::Transform;

use crate::logic::food::{Food, Grass};

#[derive(Bundle)]
pub struct GrassBundle {
    grass: Grass,
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    food: Food,
}

impl GrassBundle {
    pub fn new(
        position: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> GrassBundle {
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(shape::Box::new(25., 25., 0.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
                transform: Transform::from_translation(position.extend(2.)),
                ..default()
            },
            grass: Grass::default(),
            food: Food { remaining: 100.0 },
        }
    }
}
