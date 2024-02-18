use crate::game::systems::food::Food;
use crate::game::systems::hunt::PreyKilledEvent;
use crate::layers::ITEM_LAYER;
use bevy::asset::Assets;
use bevy::prelude::{
    default, shape, Color, ColorMaterial, Commands, Component, Entity, EventReader, Mesh,
    Query, ResMut, Transform, With,
};
use bevy::sprite::MaterialMesh2dBundle;

#[derive(Component, Default)]
pub struct Carrion {}

pub fn spawn_carrion_on_kill(
    mut er_prey_killed: EventReader<PreyKilledEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for prey_killed in er_prey_killed.read() {
        let position = prey_killed.position;
        commands.spawn((
            Food::new(100.0),
            Carrion::default(),
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                transform: Transform::from_translation(position.extend(ITEM_LAYER)),
                ..default()
            },
        ));
    }
}

pub fn despawn_eaten_carrion(
    q_food: Query<(Entity, &Food), With<Carrion>>,
    mut commands: Commands,
) {
    for (entity, food) in q_food.iter() {
        if food.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
