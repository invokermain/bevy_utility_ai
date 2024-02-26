use bevy::prelude::{
    Commands, Component, Entity, Query, Res, Time, Vec2, Vec3Swizzles, With, Without,
};
use bevy::transform::components::Transform;
use pathfinding::prelude::astar;
use rand::Rng;

use crate::game::ai::actions::{ActionIdle, ActionRest};
use crate::level::{Walls, GRID_SIZE, HALF_GRID_SIZE, MAP_SIZE};

#[derive(Component)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct IdlePath {
    pub path: Vec<(usize, usize)>,
    pub current_index: usize,
}

pub fn rest(mut q_energy: Query<&mut Energy, With<ActionRest>>, r_time: Res<Time>) {
    for mut energy in q_energy.iter_mut() {
        energy.value += 1.0 * r_time.delta_seconds();
        if energy.value >= energy.max {
            energy.value = energy.max
        }
    }
}

pub fn calculate_idle_path(
    q_position: Query<(Entity, &mut Transform), (With<ActionIdle>, Without<IdlePath>)>,
    r_walls: Res<Walls>,
    mut commands: Commands,
) {
    let mut rng = rand::thread_rng();
    for (entity, start_position) in &q_position {
        // add a random gate to make it seem like the wolf is 'waiting' sometimes
        if rng.gen_range(0.0..1.0f32) < 0.99 {
            continue;
        }

        let target_point: (usize, usize);
        target_point = loop {
            // pick a random point in the level
            let try_point = (
                (rng.gen_range(0.0..MAP_SIZE) / GRID_SIZE).round() as usize,
                (rng.gen_range(0.0..MAP_SIZE) / GRID_SIZE).round() as usize,
            );

            // check point is not off limits
            if !r_walls.in_wall(&try_point) {
                break try_point;
            }
        };

        let start_point = (
            (start_position.translation.x / GRID_SIZE).round() as usize,
            (start_position.translation.y / GRID_SIZE).round() as usize,
        );
        let mut path_grid = r_walls.grid.clone();
        path_grid.invert();
        let path_result = astar(
            &start_point,
            |point| {
                path_grid
                    .neighbours(*point)
                    .into_iter()
                    .map(|p| (p, 1usize))
            },
            |p| {
                (((p.0.abs_diff(target_point.0) + p.1.abs_diff(target_point.1)) as f32)
                    .sqrt()
                    .floor()) as usize
            },
            |p| *p == target_point,
        );

        if path_result.is_some() {
            commands.entity(entity).insert(IdlePath {
                path: path_result.unwrap().0,
                current_index: 1, // the path includes the start point
            });
        }
    }
}

pub fn idle(
    mut q_energy: Query<
        (Entity, &mut Energy, &mut Transform, &mut IdlePath),
        With<ActionIdle>,
    >,
    mut commands: Commands,
    r_time: Res<Time>,
) {
    for (entity, mut energy, mut transform, mut path) in q_energy.iter_mut() {
        energy.value += 0.5 * r_time.delta_seconds();

        if energy.value >= energy.max {
            energy.value = energy.max
        }

        let next_grid_point = path.path[path.current_index];
        let next_point = Vec2::new(
            next_grid_point.0 as f32 * GRID_SIZE,
            next_grid_point.1 as f32 * GRID_SIZE,
        );

        if transform.translation.xy().distance(next_point) <= HALF_GRID_SIZE {
            if path.current_index + 1 == path.path.len() {
                commands.entity(entity).remove::<IdlePath>();
            } else {
                path.current_index += 1;
            }
        } else {
            let direction = next_point - transform.translation.xy();
            transform.translation +=
                direction.normalize().extend(0.0) * GRID_SIZE * r_time.delta_seconds();
        }
    }
}
