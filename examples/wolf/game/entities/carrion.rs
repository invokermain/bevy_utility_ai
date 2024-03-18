use crate::game::systems::food::Food;
use crate::game::systems::hunt::PreyKilledEvent;
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec2;
use bevy::prelude::{
    default, Color, Commands, Component, Entity, EventReader, Query, Res, ResMut, Sprite,
    SpriteSheetBundle, TextureAtlas, TextureAtlasLayout, Transform, With,
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
            SpriteSheetBundle {
                sprite: Sprite {
                    color: Color::rgb(0.9, 0.9, 0.9),
                    ..default()
                },
                texture: r_asset_server.load("pigeons.png"),
                atlas: TextureAtlas {
                    layout: r_texture_atlas_layout.add(TextureAtlasLayout::from_grid(
                        Vec2::new(16.0, 16.0),
                        1,
                        1,
                        None,
                        Some(Vec2::new(0.0, 16.0)),
                    )),
                    index: 0,
                },
                transform,
                ..default()
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
