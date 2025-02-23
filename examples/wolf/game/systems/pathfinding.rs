use crate::level::{Walls, GRID_SIZE, HALF_GRID_SIZE, MAP_SIZE, MAP_TILE_WIDTH};
use bevy::prelude::*;
use pathfinding::prelude::astar;
use rand::{thread_rng, Rng};

#[derive(Event)]
pub struct PathRequested {
    pub entity: Entity,
    pub target_point: Vec2,
    pub speed: f32,
}

pub fn assign_path(
    mut er_path_requested: EventReader<PathRequested>,
    query: Query<(&Transform, Option<&Path>)>,
    mut commands: Commands,
    r_walls: Res<Walls>,
) {
    for event in er_path_requested.read() {
        if let Ok((current_transform, existing_path)) = query.get(event.entity) {
            if let Some(existing_path) = existing_path {
                if event.target_point == existing_path.destination()
                    && event.speed == existing_path.speed
                {
                    // it's the same path they are currently on
                    debug!("ignoring request for new path");
                    continue;
                }
            }
            let start_point = current_transform.translation.xy();
            let path_vector =
                calculate_path_vector(&start_point, &event.target_point, &r_walls);
            if let Some(path_vector) = path_vector {
                let path = Path::new(path_vector, event.speed);
                debug!("assigned path to {:?}", path.destination());
                commands.entity(event.entity).insert(path);
            }
        }
    }
}

pub fn follow_path(
    mut query: Query<(Entity, &mut Transform, &mut Path)>,
    mut commands: Commands,
    r_time: Res<Time>,
) {
    for (entity, mut transform, mut path) in &mut query {
        let subject_point = transform.translation.xy();
        let target_point = path.current_path_point();
        let distance_to_target = subject_point.distance(target_point);
        // if we are near target complete step
        if distance_to_target <= 1.0 {
            if path.is_path_complete() {
                debug!("path completed");
                commands.entity(entity).remove::<Path>();
            } else {
                path.complete_path_point();
            }
        }
        // otherwise walk to the target point
        else {
            let direction = (target_point - subject_point).normalize();
            transform.translation += direction.extend(0.0)
                * (GRID_SIZE * path.speed * r_time.delta_seconds())
                    .min(distance_to_target);
        }
    }
}

#[derive(Component)]
pub struct Path {
    pub path: Vec<Vec2>,
    pub current_index: usize,
    pub speed: f32,
}

impl Path {
    pub fn new(path_vector: Vec<Vec2>, speed: f32) -> Self {
        Self {
            path: path_vector,
            current_index: 0,
            speed,
        }
    }

    pub fn current_path_point(&self) -> Vec2 {
        self.path[self.current_index]
    }

    pub fn destination(&self) -> Vec2 {
        *self.path.last().unwrap()
    }

    pub fn complete_path_point(&mut self) {
        self.current_index += 1;
    }

    pub fn is_path_complete(&self) -> bool {
        self.current_index + 1 == self.path.len()
    }
}

pub fn random_pathable_point(walls: &Walls) -> Vec2 {
    let mut rng = thread_rng();
    loop {
        let try_point = Vec2::new(
            (rng.gen_range(0.0..MAP_TILE_WIDTH as f32).round() * GRID_SIZE)
                + HALF_GRID_SIZE,
            (rng.gen_range(0.0..MAP_TILE_WIDTH as f32).round() * GRID_SIZE)
                + HALF_GRID_SIZE,
        );
        if !walls.in_wall(&try_point) {
            break try_point;
        }
    }
}

pub fn random_point_on_edge_of_map() -> Vec2 {
    let mut rng = thread_rng();
    match rng.gen_range(0..4) {
        0 => Vec2::new(-5.0, rng.gen_range(0.0..MAP_SIZE)),
        1 => Vec2::new(MAP_SIZE + 5.0, rng.gen_range(0.0..MAP_SIZE)),
        2 => Vec2::new(rng.gen_range(0.0..MAP_SIZE), -5.0),
        3 => Vec2::new(rng.gen_range(0.0..MAP_SIZE), MAP_SIZE + 5.0),
        _ => Vec2::default(),
    }
}

pub fn calculate_path_vector(
    start: &Vec2,
    end: &Vec2,
    walls: &Walls,
) -> Option<Vec<Vec2>> {
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
        if path.0.len() == 1 {
            return None;
        }
        Some(
            path.0
                .iter()
                .skip(1) // it includes the start point
                .map(grid_to_world)
                .collect(),
        )
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
    use pathfinding::grid::Grid;

    use super::*;

    #[test]
    fn test_world_to_grid() {
        assert_eq!(world_to_grid(&Vec2::new(0.0, 0.0)), (0, 0));
        assert_eq!(
            world_to_grid(&Vec2::new(HALF_GRID_SIZE, HALF_GRID_SIZE)),
            (0, 0)
        );
        assert_eq!(
            world_to_grid(&Vec2::new(
                GRID_SIZE + HALF_GRID_SIZE,
                GRID_SIZE + HALF_GRID_SIZE
            )),
            (1, 1)
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

    #[test]
    fn test_random_point_is_pathable() {
        let walls = Walls {
            grid: Grid::new(GRID_SIZE as usize, GRID_SIZE as usize),
        };
        let point = random_pathable_point(&walls);
        let path = calculate_path_vector(&Vec2::splat(0.0), &point, &walls);
        let path_destination = path.unwrap().iter().last().unwrap().clone();
        assert_eq!(path_destination, point);
    }
}
