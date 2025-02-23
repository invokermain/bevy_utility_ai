use crate::game::ai::actions::{ActionIdle, ActionRest};
use crate::game::systems::pathfinding::{
    calculate_path_vector, random_pathable_point, Path,
};
use crate::level::{Walls, WolfText, GRID_SIZE};
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy::utils::HashMap;
use rand::{thread_rng, Rng};

use super::pathfinding::PathRequested;

#[derive(Component, Copy, Clone)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}

#[derive(Component, Default, Copy, Clone)]
pub struct Shelter;

#[derive(Component, Default, Copy, Clone)]
pub struct Fatigued;

#[derive(Component, Copy, Clone)]
pub struct IdleBehaviour {
    pub idle_time: f32,
    pub idled_for: f32,
    pub idle_location: Vec2,
}

impl IdleBehaviour {
    pub fn new(idle_location: Vec2) -> Self {
        let mut rng = thread_rng();
        Self {
            idle_time: rng.gen_range(1.0..8.0),
            idled_for: 0.0,
            idle_location,
        }
    }

    pub fn idled_for(&mut self, seconds: f32) {
        self.idled_for += seconds;
    }

    pub fn is_finished_idling(&self) -> bool {
        self.idled_for >= self.idle_time
    }
}

pub fn rest(
    mut q_subject: Query<
        (Entity, &mut Energy, &Transform, &mut Visibility),
        (With<ActionRest>, Without<Path>, Without<WolfText>),
    >,
    q_shelter: Query<&Transform, With<Shelter>>,
    mut q_wolf_text: Query<(&mut Text, &mut Visibility), With<WolfText>>,
    mut ew_path_requested: EventWriter<PathRequested>,
    r_time: Res<Time>,
    mut commands: Commands,
) {
    for (entity, mut energy, transform, mut visibility) in q_subject.iter_mut() {
        if let Ok(shelter_transform) = q_shelter.get_single() {
            let shelter_point = shelter_transform.translation.xy();
            let subject_point = transform.translation.xy();
            let distance_to_shelter = subject_point.distance(shelter_point);
            // if we are near shelter sleep
            if distance_to_shelter <= 1.0 {
                *visibility = Visibility::Hidden;

                energy.value += 5.0 * r_time.delta_seconds();

                if energy.value >= energy.max {
                    commands.entity(entity).remove::<Fatigued>();
                    energy.value = energy.max;
                }

                if let Ok((mut text, mut text_visibility)) = q_wolf_text.get_single_mut()
                {
                    *text_visibility = Visibility::Visible;
                    text.sections[0].value = "*zzz*".into();
                };
            }
            // otherwise request path to shelter
            else {
                ew_path_requested.send(PathRequested {
                    entity,
                    target_point: shelter_point,
                    speed: 2.0,
                });
            }
        };
    }
}

pub fn on_rest_removed(
    mut removals: RemovedComponents<ActionRest>,
    mut query: Query<&mut Visibility>,
) {
    for entity in removals.read() {
        if let Ok(mut visibility) = query.get_mut(entity) {
            *visibility = Visibility::Visible;
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

        let path_vector = calculate_path_vector(
            &start_position.translation.xy(),
            &target_point,
            &r_walls,
        );

        if let Some(path_vector) = path_vector {
            commands.entity(entity).insert((
                IdleBehaviour::new(target_point),
                Path::new(path_vector, 1.0),
            ));
        }
    }
}

pub fn on_idle_removed(
    mut removals: RemovedComponents<ActionIdle>,
    mut commands: Commands,
) {
    for entity in removals.read() {
        commands.entity(entity).remove::<IdleBehaviour>();
    }
}

pub fn idle(
    mut q_energy: Query<(Entity, &Transform, &mut IdleBehaviour), With<ActionIdle>>,
    mut commands: Commands,
    r_time: Res<Time>,
) {
    for (entity, transform, mut idle_behaviour) in q_energy.iter_mut() {
        let target_point = idle_behaviour.idle_location;
        let distance_to_target = transform.translation.xy().distance(target_point);

        // Idle at the target idle location
        if distance_to_target <= 1.0 {
            if idle_behaviour.is_finished_idling() {
                commands.entity(entity).remove::<IdleBehaviour>();
            } else {
                idle_behaviour.idled_for(r_time.delta_seconds());
            }
        }
    }
}

pub fn consume_energy(
    mut query: Query<(Entity, &Transform, &mut Energy)>,
    r_time: Res<Time>,
    mut previous_positions: Local<HashMap<Entity, Vec2>>,
    mut commands: Commands,
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

        if energy.value <= 5.0 {
            commands.entity(entity).insert(Fatigued);
        }

        *previous_position = current_position;
    }
}
