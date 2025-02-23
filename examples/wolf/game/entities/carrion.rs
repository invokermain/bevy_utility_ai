use crate::game::systems::{food::Food, hunt::PreyKilledEvent};
use bevy::{
    asset::{AssetServer, Assets},
    prelude::*,
};
use rand::{thread_rng, Rng};
use std::f32::consts::PI;

#[derive(Component, Default)]
pub struct Meat {}

pub fn spawn_meat_on_kill(
    mut er_prey_killed: EventReader<PreyKilledEvent>,
    mut commands: Commands,
    mut r_texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
    r_asset_server: Res<AssetServer>,
) {
    let mut rng = thread_rng();
    for prey_killed in er_prey_killed.read() {
        let mut transform = Transform::from_translation(prey_killed.position.extend(8.0));
        transform.rotate_z(PI * rng.gen_range(0.0..4.0));

        commands.entity(prey_killed.entity).despawn();
        commands.spawn((
            Food::new(25.0),
            Meat::default(),
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.9, 0.9, 0.9),
                    ..default()
                },
                texture: r_asset_server.load("pigeons.png"),
                transform,
                ..default()
            },
            TextureAtlas {
                layout: r_texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                    UVec2::new(16, 16),
                    1,
                    1,
                    None,
                    Some(UVec2::new(0, 16)),
                )),
                index: 0,
            },
        ));
    }
}

pub fn despawn_eaten_meat(
    q_food: Query<(Entity, &Food), With<Meat>>,
    mut commands: Commands,
) {
    for (entity, food) in q_food.iter() {
        if food.remaining <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
