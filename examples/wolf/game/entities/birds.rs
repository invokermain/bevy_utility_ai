use crate::level::{Walls, GRID_SIZE, MAP_SIZE};
use crate::utils::animations::{AnimationIndices, AnimationTimer};
use crate::utils::pathfinding::random_pathable_point;
use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::{Command, Res};
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::SpriteSheetBundle;
use rand::{thread_rng, Rng};

#[derive(Copy, Clone, Debug, Component)]
pub enum BirdState {
    Resting,
    Walking,
    Flying,
}

#[derive(Event)]
pub struct BirdStateChanged {
    entity: Entity,
    previous: BirdState,
    new: BirdState,
}

#[derive(Component)]
pub struct FlyTo {
    pub point: Vec2,
}

#[derive(Bundle)]
pub struct BirdBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    state: BirdState,
    fly_to: FlyTo,
}

pub struct SpawnBird {
    fly_to_point: Vec2,
}

impl Command for SpawnBird {
    fn apply(self, world: &mut World) {
        let texture = world.resource_scope(|_, asset_server: Mut<AssetServer>| {
            asset_server.load("pigeons.png")
        });
        let layout = TextureAtlasLayout::from_grid(
            Vec2::new(16.0, 16.0),
            3,
            1,
            None,
            Some(Vec2::new(0.0, 16.0)),
        );
        let texture_atlas_layout = world.resource_scope(
            |_, mut texture_atlas_layouts: Mut<Assets<TextureAtlasLayout>>| {
                texture_atlas_layouts.add(layout)
            },
        );
        let animation_indices = AnimationIndices { first: 0, last: 2 };

        let mut rng = thread_rng();
        let starting_point = Vec3::new(
            rng.gen_range(0.0..MAP_SIZE),
            rng.gen_range(0.0..MAP_SIZE),
            100.0,
        );

        world.spawn(BirdBundle {
            sprite: SpriteSheetBundle {
                texture,
                transform: Transform::from_translation(starting_point),
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            )),
            state: BirdState::Flying,
            fly_to: FlyTo {
                point: self.fly_to_point,
            },
        });
    }
}

pub fn spawn_birds_occasionally(
    mut time_till_spawn: Local<f32>,
    mut commands: Commands,
    r_time: Res<Time>,
    r_walls: Res<Walls>,
) {
    if *time_till_spawn <= 0.0 {
        let mut rng = thread_rng();
        let random = rng.gen_range(0.0..1.032);
        let bird_num = if random <= 0.5 {
            1u8
        } else if random <= 0.8 {
            2u8
        } else {
            3u8
        };
        for _ in 0..bird_num {
            let fly_to_point = random_pathable_point(&r_walls);
            commands.add(SpawnBird { fly_to_point });
        }
        *time_till_spawn = rng.gen_range(10.0..30.0f32);
    } else {
        *time_till_spawn -= r_time.delta_seconds();
    }
}

pub fn fly_to_point(
    mut q_birds: Query<(Entity, &mut Transform, &FlyTo)>,
    r_time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut transform, fly_to) in &mut q_birds {
        let current_point = transform.translation.xy();
        let target_point = fly_to.point;
        let distance_to_target = target_point.distance(current_point);
        if distance_to_target <= 1.0 {
            commands
                .entity(entity)
                .insert(BirdState::Walking)
                .remove::<FlyTo>();
        } else {
            let direction = (target_point - current_point).normalize();
            transform.translation += direction.extend(0.0)
                * (GRID_SIZE * 2.5 * r_time.delta_seconds()).min(distance_to_target);
        }
    }
}

pub fn update_bird_on_state_change(
    q_bird: Query<(Entity, &BirdState), Changed<BirdState>>,
    mut commands: Commands,
) {
    for (entity, new_state) in &q_bird {
        match new_state {
            BirdState::Resting => {}
            BirdState::Walking => {}
            BirdState::Flying => {}
        }
    }
}
