use crate::game::systems::food::Food;
use crate::layers::RESOURCE_LAYER;
use bevy::asset::Assets;
use bevy::math::primitives::Cuboid;
use bevy::math::Vec2;
use bevy::prelude::{
    default, Bundle, Color, ColorMaterial, Commands, Component, Entity, Mesh, Query,
    ResMut, Transform, Visibility, Without,
};
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Default)]
pub struct Grass {
    growth: u8,
}

pub fn hide_eaten_grass(
    mut q_grass: Query<(Entity, &mut Grass, &Food, &mut Visibility)>,
    mut commands: Commands,
) {
    for (entity, mut grass, food, mut visibility) in &mut q_grass {
        if food.remaining == 0.0 {
            grass.growth = 0;
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<Food>();
        }
    }
}

pub fn regrow_grass(
    mut q_grass: Query<(Entity, &mut Grass, &mut Visibility), Without<Food>>,
    mut commands: Commands,
) {
    for (entity, mut grass, mut visibility) in &mut q_grass {
        grass.growth += 1;

        if grass.growth >= 100 {
            *visibility = Visibility::Visible;
            commands.entity(entity).insert(Food::new(100.0));
        }
    }
}

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
                mesh: meshes.add(Cuboid::new(25., 25., 0.)).into(),
                material: materials.add(ColorMaterial::from(Color::DARK_GREEN)),
                transform: Transform::from_translation(position.extend(RESOURCE_LAYER)),
                ..default()
            },
            grass: Grass::default(),
            food: Food::new(100.0),
        }
    }
}
