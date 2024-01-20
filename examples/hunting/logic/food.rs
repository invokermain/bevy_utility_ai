use crate::bundles::GrassBundle;
use crate::logic::ai::actions::ActionEat;
use crate::logic::hunt::PreyKilledEvent;
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_utility_ai::ActionTarget;
use rand::Rng;

#[derive(Component)]
pub struct Hunger {
    pub value: f32,
    pub max: f32,
}

#[derive(Component)]
pub struct Food {
    pub remaining: f32,
}

#[derive(Component, Default)]
pub struct Grass {}

pub fn spawn_food_on_kill(
    mut er_prey_killed: EventReader<PreyKilledEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for prey_killed in er_prey_killed.read() {
        let position = prey_killed.position;
        commands.spawn((
            Food { remaining: 100. },
            MaterialMesh2dBundle {
                material: materials.add(ColorMaterial::from(Color::BLACK)),
                mesh: meshes.add(shape::Circle::new(5.).into()).into(),
                transform: Transform::from_translation(Vec3::new(
                    position.x, position.y, 1.0,
                )),
                ..default()
            },
        ));
    }
}

pub fn spawn_new_grass_on_grass_despawn(
    mut removed: RemovedComponents<Grass>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    for _ in removed.read() {
        let mut position: Vec2;
        loop {
            position = Vec2::new(
                rng.gen_range(-1000.0..=1000.0),
                rng.gen_range(-1000.0..=1000.0),
            );
            if !((-300f32..-200f32).contains(&position.x)
                && (-300f32..-200f32).contains(&position.y))
                || ((300f32..200f32).contains(&position.x)
                    && (300f32..200f32).contains(&position.y))
            {
                break;
            }
        }

        commands.spawn(GrassBundle::new(position, &mut meshes, &mut materials));
    }
}

pub fn increase_hunger(mut q_hunger: Query<&mut Hunger>) {
    for mut hunger in q_hunger.iter_mut() {
        hunger.value += 0.1;
        if hunger.value >= hunger.max {
            hunger.value = hunger.max;
        }
    }
}

pub fn despawn_empty_food(q_food: Query<(Entity, &Food)>, mut commands: Commands) {
    for (entity, food) in q_food.iter() {
        if food.remaining == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn eat(
    mut q_subject: Query<
        (&mut Hunger, &mut Transform, &ActionTarget),
        (With<ActionEat>, Without<Food>),
    >,
    mut q_food: Query<(&mut Food, &Transform)>,
) {
    for (mut subject_hunger, mut transform, action_target) in q_subject.iter_mut() {
        if let Ok((mut food, food_transform)) = q_food.get_mut(action_target.target) {
            // if we are near food eat
            if food_transform.translation.distance(transform.translation) < 5.0 {
                let portion_size = food.remaining.min(2.5).min(subject_hunger.value);
                food.remaining -= portion_size;
                subject_hunger.value -= portion_size;
            }
            // otherwise walk to the food
            else {
                let direction =
                    (food_transform.translation - transform.translation).normalize();
                transform.translation += direction * 2.5;
            }
        } else {
            continue;
        };
    }
}
