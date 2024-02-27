use crate::game::ai::actions::{ActionIdle, ActionRest};
use crate::level::{Walls, GRID_SIZE};
use crate::utils::pathfinding::{calculate_path, random_pathable_point, Path};
use bevy::prelude::{
    Commands, Component, Entity, Query, Res, Time, Vec3Swizzles, With, Without,
};
use bevy::transform::components::Transform;
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
    for mut energy in q_energy.iter_mut() {
        energy.value += 1.0 * r_time.delta_seconds();
        if energy.value >= energy.max {
            energy.value = energy.max
        }
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

        let path =
            calculate_path(&start_position.translation.xy(), &target_point, &r_walls);

        if path.is_some() {
            commands
                .entity(entity)
                .insert((IdleBehaviour::new(), path.unwrap()));
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
