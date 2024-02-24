use bevy::asset::Assets;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::ResMut;
use bevy::math::primitives::Circle;
use bevy::math::Vec2;
use bevy::prelude::default;
use bevy::render::color::Color;
use bevy::render::mesh::Mesh;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle};
use bevy::transform::components::Transform;

use crate::game::systems::water::Water;
use crate::layers::RESOURCE_LAYER;

#[derive(Bundle)]
pub struct WaterSourceBundle {
    mesh: MaterialMesh2dBundle<ColorMaterial>,
    water: Water,
}

impl WaterSourceBundle {
    pub fn new(
        position: Vec2,
        meshes: &mut ResMut<Assets<Mesh>>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
    ) -> Self {
        Self {
            mesh: MaterialMesh2dBundle {
                mesh: meshes.add(Circle::new(25.)).into(),
                material: materials.add(ColorMaterial::from(Color::BLUE)),
                transform: Transform::from_translation(position.extend(RESOURCE_LAYER)),
                ..default()
            },
            water: Water {},
        }
    }
}
