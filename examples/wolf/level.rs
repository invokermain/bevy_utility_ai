use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Assets};
use bevy::math::Vec3;
use bevy::prelude::{
    default, Added, Camera2dBundle, Commands, Entity, OrthographicProjection, Query, Res,
    ResMut, Resource, Startup, TextureAtlasLayout, Transform, Update, Vec2, Vec3Swizzles,
};
use bevy::render::camera::ScalingMode;
use bevy_ecs_ldtk::{
    EntityInstance, IntGridCell, IntGridRendering, LdtkSettings, LdtkWorldBundle,
    LevelSelection,
};
use pathfinding::prelude::Grid;

use crate::game::entities::water_source::WaterSourceBundle;
use crate::game::entities::wolf::WolfBundle;

pub const GRID_SIZE: f32 = 16.0;
pub const HALF_GRID_SIZE: f32 = 8.0;
pub const MAP_SIZE: f32 = 256.0;
pub const MAP_TILE_WIDTH: usize = 16;
pub const MAP_TILE_HEIGHT: usize = 16;

#[derive(Copy, Clone, Debug, Default)]
pub struct WolfSceneSetupPlugin;

impl Plugin for WolfSceneSetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(LevelSelection::index(0));
        app.insert_resource(Walls {
            grid: Grid::new(MAP_TILE_WIDTH, MAP_TILE_HEIGHT),
        });
        app.insert_resource(LdtkSettings {
            int_grid_rendering: IntGridRendering::Invisible,
            ..default()
        });
        app.add_systems(Startup, setup_level);
        app.add_systems(Update, (spawn_game_entities, collect_int_gridcells));
    }
}

fn setup_level(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(
            MAP_SIZE / 2.0,
            MAP_SIZE / 2.0,
            100.0,
        )),
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: MAP_SIZE,
                min_height: MAP_SIZE,
            },
            ..default()
        },
        ..default()
    });
    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("game.ldtk"),
        ..default()
    });
}

fn spawn_game_entities(
    mut commands: Commands,
    new_entity_instances: Query<
        (Entity, &EntityInstance, &Transform),
        Added<EntityInstance>,
    >,
    assets: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    for (entity, entity_instance, transform) in new_entity_instances.iter() {
        if entity_instance.identifier == *"Wolf" {
            commands.entity(entity).insert(WolfBundle::new(
                *transform,
                &assets,
                &mut texture_atlas_layouts,
            ));
        } else if entity_instance.identifier == *"WaterSource" {
            commands
                .entity(entity)
                .insert(WaterSourceBundle::new(*transform));
        }
    }
}

#[derive(Resource, Clone, Debug)]
pub struct Walls {
    pub grid: Grid,
}

impl Walls {
    pub fn in_wall(&self, point: &Vec2) -> bool {
        let vertex = (
            (point.x / GRID_SIZE).round() as usize,
            (point.y / GRID_SIZE).round() as usize,
        );
        vertex.0 >= MAP_TILE_WIDTH
            || vertex.1 >= MAP_TILE_HEIGHT
            || self.grid.has_vertex(vertex)
    }
}

fn collect_int_gridcells(
    new_grid_cells: Query<(&IntGridCell, &Transform), Added<IntGridCell>>,
    mut r_walls: ResMut<Walls>,
) {
    if new_grid_cells.is_empty() {
        return;
    }

    let wall_locations: Vec<(usize, usize)> = new_grid_cells
        .iter()
        .filter(|(cell, _)| cell.value == 1)
        .map(|(_, transform)| transform.translation.xy() / GRID_SIZE)
        .map(|pos| (pos.x.round() as usize, pos.y.round() as usize))
        .collect();

    wall_locations.iter().for_each(|pos| {
        r_walls.grid.add_vertex(*pos);
    });
}
