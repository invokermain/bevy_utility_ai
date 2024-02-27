use crate::level::{Walls, GRID_SIZE, HALF_GRID_SIZE, MAP_SIZE};
use bevy::prelude::{Component, Vec2};
use pathfinding::prelude::astar;
use rand::Rng;

#[derive(Component)]
pub struct Path {
    pub path: Vec<Vec2>,
    pub current_index: usize,
}

impl Path {
    pub fn new(path: Vec<Vec2>) -> Self {
        Self {
            path,
            current_index: 0,
        }
    }

    pub fn current_path_point(&self) -> Vec2 {
        self.path[self.current_index].clone()
    }

    pub fn complete_path_point(&mut self) {
        self.current_index += 1;
    }

    pub fn is_path_complete(&self) -> bool {
        self.current_index + 1 == self.path.len()
    }

    pub fn validate_destination(&self, destination: &Vec2) -> bool {
        let last_path_point = self.path.last().unwrap();
        last_path_point.distance(*destination) <= HALF_GRID_SIZE
    }
}

pub fn random_pathable_point(walls: &Walls) -> Vec2 {
    let mut rng = rand::thread_rng();
    loop {
        let try_point =
            Vec2::new(rng.gen_range(0.0..MAP_SIZE), rng.gen_range(0.0..MAP_SIZE));
        if !walls.in_wall(&try_point) {
            break try_point;
        }
    }
}

pub fn calculate_path(start: &Vec2, end: &Vec2, walls: &Walls) -> Option<Path> {
    // we need to translate to 'grid scale'.
    let start_point = world_to_grid(start);
    let end_point = world_to_grid(end);
    let mut path_grid = walls.grid.clone();
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
            ((p.0.abs_diff(end_point.0) + p.1.abs_diff(end_point.1)) as f32)
                .sqrt()
                .floor() as usize
        },
        |p| *p == end_point,
    );

    return if let Some(path) = path_result {
        Some(Path::new(
            path.0
                .iter()
                .skip(1) // it includes the start point
                .map(grid_to_world)
                .collect(),
        ))
    } else {
        None
    };
}

fn world_to_grid(world_point: &Vec2) -> (usize, usize) {
    (
        ((world_point.x - HALF_GRID_SIZE) / GRID_SIZE).round() as usize,
        ((world_point.y - HALF_GRID_SIZE) / GRID_SIZE).round() as usize,
    )
}

fn grid_to_world(grid_point: &(usize, usize)) -> Vec2 {
    Vec2::new(
        grid_point.0 as f32 * GRID_SIZE + HALF_GRID_SIZE,
        grid_point.1 as f32 * GRID_SIZE + HALF_GRID_SIZE,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_to_grid() {
        assert_eq!(world_to_grid(&Vec2::new(0.0, 0.0)), (0, 0));
        assert_eq!(
            world_to_grid(&Vec2::new(HALF_GRID_SIZE, HALF_GRID_SIZE)),
            (0, 0)
        );
        assert_eq!(world_to_grid(&Vec2::new(GRID_SIZE, GRID_SIZE)), (1, 1));
        assert_eq!(world_to_grid(&Vec2::new(MAP_SIZE, MAP_SIZE)), (16, 16));
    }

    #[test]
    fn test_grid_to_world() {
        assert_eq!(
            grid_to_world(&(0, 0)),
            Vec2::new(HALF_GRID_SIZE, HALF_GRID_SIZE)
        );
        assert_eq!(
            grid_to_world(&(1, 1)),
            Vec2::new(GRID_SIZE + HALF_GRID_SIZE, GRID_SIZE + HALF_GRID_SIZE)
        );
    }
}
