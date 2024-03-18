use crate::game::ai::actions::{ActionIdle, ActionRest};
use crate::game::systems::pathfinding::{
    calculate_path_vector, random_pathable_point, Path,
};
use crate::level::{Walls, GRID_SIZE};
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy::utils::HashMap;
use rand::{thread_rng, Rng};

#[derive(Component)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct IdleBehaviour {
    pub idle_time: f32,
    pub idled_for: f32,
}

impl IdleBehaviour {
    pub fn new() -> Self {
        let mut rng = thread_rng();
        Self {
            idle_time: rng.gen_range(1.0..8.0),
            idled_for: 0.0,
        }
    }

    pub fn idled_for(&mut self, seconds: f32) {
        self.idled_for += seconds;
    }

    pub fn is_finished_idling(&self) -> bool {
        self.idled_for >= self.idle_time
    }
}

pub fn rest(mut q_energy: Query<&mut Energy, With<ActionRest>>, r_time: Res<Time>) {
    for (entity, mut subject_thirst, transform, action_target) in q_subject.iter_mut() {
        if let Ok(water_transform) = q_water.get_mut(action_target.target) {
            let water_point = water_transform.translation.xy();
            let subject_point = transform.translation.xy();
            let distance_to_water = subject_point.distance(water_point);
            // if we are near water drink
            if distance_to_water <= 1.0 {
                let portion_size = 20.0f32.min(subject_thirst.value);
                subject_thirst.value -= portion_size * r_time.delta_seconds();

                if let Ok(mut text) = q_wolf_text.get_single_mut() {
                    text.sections[0].value = "*slurp*".into();
                };
            }
            // otherwise request path to water
            else {
                ew_path_requested.send(PathRequested {
                    entity,
                    target_point: water_point,
                    speed: 2.0,
                });
            }
        } else {
            continue;
        };
    }
}

pub fn insert_idle_behaviour(
    q_position: Query<
        (Entity, &mut Transform),
        (With<ActionIdle>, Without<IdleBehaviour>),
    >,
    r_walls: Res<Walls>,
    mut commands: Commands,
) {
    for (entity, start_position) in &q_position {
        let target_point = random_pathable_point(&r_walls);

        let path_vector = calculate_path_vector(
            &start_position.translation.xy(),
            &target_point,
            &r_walls,
        );

        if let Some(path_vector) = path_vector {
            commands
                .entity(entity)
                .insert((IdleBehaviour::new(), Path::new(path_vector, 1.0)));
        }
    }
}

pub fn idle(
    mut q_energy: Query<
        (
            Entity,
            &mut Energy,
            &mut Transform,
            &mut IdleBehaviour,
            &mut Path,
        ),
        With<ActionIdle>,
    >,
    mut commands: Commands,
    r_time: Res<Time>,
) {
    for (entity, mut energy, mut transform, mut idle_behaviour, mut path) in
        q_energy.iter_mut()
    {
        energy.value += 0.5 * r_time.delta_seconds();

        if energy.value >= energy.max {
            energy.value = energy.max
        }

        let target_point = path.current_path_point();
        let distance_to_target = transform.translation.xy().distance(target_point);

        if distance_to_target <= 1.0 {
            if path.is_path_complete() {
                // Idle when we get to the destination
                if idle_behaviour.is_finished_idling() {
                    commands.entity(entity).remove::<IdleBehaviour>();
                } else {
                    idle_behaviour.idled_for(r_time.delta_seconds());
                }
            } else {
                path.complete_path_point();
            }
        } else {
            let direction = (target_point - transform.translation.xy()).normalize();
            transform.translation += direction.extend(0.0)
                * (GRID_SIZE * 1.25 * r_time.delta_seconds()).min(distance_to_target);
        }
    }
}

pub fn consume_energy(
    mut query: Query<(Entity, &Transform, &mut Energy)>,
    r_time: Res<Time>,
    mut previous_positions: Local<HashMap<Entity, Vec2>>,
) {
    for (entity, transform, mut energy) in query.iter_mut() {
        let current_position = transform.translation.xy();
        let previous_position =
            previous_positions.entry(entity).or_insert(current_position);
        let distance_travelled = current_position.distance(*previous_position);
        let velocity = distance_travelled / (r_time.delta_seconds() * GRID_SIZE);
        let energy_used =
            (velocity - 1.0).max(0.0).powf(2.0) * r_time.delta_seconds() * 0.25;
        energy.value = (energy.value - energy_used).max(0.0);

        *previous_position = current_position.clone();
    }
}
