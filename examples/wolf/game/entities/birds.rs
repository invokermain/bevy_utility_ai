use crate::game::systems::hunt::IsPrey;
use crate::game::systems::pathfinding::{
    random_pathable_point, random_point_on_edge_of_map,
};
use crate::level::{Walls, GRID_SIZE, MAP_SIZE};
use crate::utils::animations::{AnimationIndices, AnimationTimer};
use bevy::asset::AssetServer;
use bevy::ecs::system::{Command, Res};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::SpriteSheetBundle;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone)]
pub enum BirdScriptStage {
    FlyInto,
    Wait,
    FlyAway,
}

#[derive(Component, Copy, Clone)]
pub struct BirdScriptedMovement {
    pub start_at: Vec2,
    pub land_at: Vec2,
    pub wait_for: f32,
    pub end_at: Vec2,
    pub stage: BirdScriptStage,
}

#[derive(Resource, Default)]
pub struct BirdAssetHandles {
    image: Handle<Image>,
    flying_texture_layout: Handle<TextureAtlasLayout>,
    flying_animation: (AnimationIndices, AnimationTimer),
    resting_texture_layout: Handle<TextureAtlasLayout>,
}

pub fn load_bird_assets(
    mut r_bird_assets: ResMut<BirdAssetHandles>,
    r_asset_server: Res<AssetServer>,
    mut r_texture_atlas_layout: ResMut<Assets<TextureAtlasLayout>>,
) {
    *r_bird_assets = BirdAssetHandles {
        image: r_asset_server.load("pigeons.png"),
        flying_texture_layout: r_texture_atlas_layout.add(TextureAtlasLayout::from_grid(
            Vec2::new(16.0, 16.0),
            3,
            1,
            None,
            Some(Vec2::new(0.0, 16.0)),
        )),
        flying_animation: (
            AnimationIndices { first: 0, last: 2 },
            AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        ),
        resting_texture_layout: r_texture_atlas_layout.add(
            TextureAtlasLayout::from_grid(
                Vec2::new(16.0, 16.0),
                1,
                1,
                None,
                Some(Vec2::new(64.0, 16.0)),
            ),
        ),
    }
}

pub struct SpawnBird;

impl Command for SpawnBird {
    fn apply(self, world: &mut World) {
        let mut rng = thread_rng();
        let fly_to_point = random_pathable_point(world.resource::<Walls>());
        world.resource_scope(|world: &mut World, asset_handles: Mut<BirdAssetHandles>| {
            let starting_point = random_point_on_edge_of_map();
            let end_point = starting_point
                + (fly_to_point - starting_point).normalize() * MAP_SIZE * 1.1;
            let entity_layer: f32 = 8.0;

            world.spawn((
                SpriteSheetBundle {
                    texture: asset_handles.image.clone(),
                    transform: Transform::from_translation(
                        starting_point.extend(entity_layer),
                    ),
                    atlas: TextureAtlas {
                        layout: asset_handles.flying_texture_layout.clone(),
                        index: 0,
                    },
                    ..default()
                },
                BirdScriptedMovement {
                    start_at: starting_point,
                    land_at: fly_to_point,
                    wait_for: rng.gen_range(3.0..8.0),
                    end_at: end_point,
                    stage: BirdScriptStage::FlyInto,
                },
                asset_handles.flying_animation.clone(),
            ));
        })
    }
}

pub fn spawn_birds_occasionally(
    mut time_till_spawn: Local<f32>,
    mut commands: Commands,
    r_time: Res<Time>,
) {
    if *time_till_spawn <= 0.0 {
        let mut rng = thread_rng();
        commands.add(SpawnBird);
        *time_till_spawn = rng.gen_range(10.0..30.0f32);
    } else {
        *time_till_spawn -= r_time.delta_seconds();
    }
}

pub fn bird_movement(
    mut q_birds: Query<(Entity, &mut Transform, &mut BirdScriptedMovement)>,
    r_time: Res<Time>,
    r_asset_handles: Res<BirdAssetHandles>,
    mut commands: Commands,
) {
    for (entity, mut transform, mut movement_script) in &mut q_birds {
        match movement_script.stage {
            BirdScriptStage::FlyInto => {
                let current_point = transform.translation.xy();
                let target_point = movement_script.land_at;
                let distance_to_target = target_point.distance(current_point);
                if distance_to_target <= 0.1 {
                    commands
                        .entity(entity)
                        .insert((
                            TextureAtlas {
                                layout: r_asset_handles.resting_texture_layout.clone(),
                                index: 0,
                            },
                            IsPrey,
                        ))
                        .remove::<AnimationIndices>()
                        .remove::<AnimationTimer>();
                    movement_script.stage = BirdScriptStage::Wait;
                } else {
                    let direction = (target_point - current_point).normalize();
                    transform.translation += direction.extend(0.0)
                        * (GRID_SIZE * 2.5 * r_time.delta_seconds())
                            .min(distance_to_target);
                }
            }
            BirdScriptStage::Wait => {
                if movement_script.wait_for <= 0.0 {
                    commands
                        .entity(entity)
                        .insert((
                            TextureAtlas {
                                layout: r_asset_handles.flying_texture_layout.clone(),
                                index: 0,
                            },
                            r_asset_handles.flying_animation.clone(),
                        ))
                        .remove::<IsPrey>();
                    movement_script.stage = BirdScriptStage::FlyAway;
                } else {
                    movement_script.wait_for -= r_time.delta_seconds();
                }
            }
            BirdScriptStage::FlyAway => {
                let current_point = transform.translation.xy();
                let target_point = movement_script.end_at;
                let distance_to_target = target_point.distance(current_point);
                if distance_to_target <= 1.0 {
                    commands.entity(entity).despawn();
                } else {
                    let direction = (target_point - current_point).normalize();
                    transform.translation += direction.extend(0.0)
                        * (GRID_SIZE * 2.5 * r_time.delta_seconds())
                            .min(distance_to_target);
                }
            }
        }
    }
}
