use crate::logic::ai::actions::ActionEat;
use crate::logic::hunt::PreyKilledEvent;
use bevy::asset::Assets;
use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy_utility_ai::ActionTarget;

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
pub struct Grass {
    growth: u8
}

#[derive(Component, Default)]
pub struct Carrion {}

pub fn spawn_carrion_on_kill(
    mut er_prey_killed: EventReader<PreyKilledEvent>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for prey_killed in er_prey_killed.read() {
        let position = prey_killed.position;
        commands.spawn((
            Food { remaining: 100. },
            Carrion::default(),
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

pub fn increase_hunger(mut q_hunger: Query<&mut Hunger>) {
    for mut hunger in q_hunger.iter_mut() {
        hunger.value += 0.1;
        if hunger.value >= hunger.max {
            hunger.value = hunger.max;
        }
    }
}

pub fn despawn_eaten_carrion(q_food: Query<(Entity, &Food), With<Carrion>>, mut commands: Commands) {
    for (entity, food) in q_food.iter() {
        if food.remaining == 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn hide_eaten_grass(mut q_grass: Query<(Entity, &mut Grass, &Food, &mut Visibility)>, mut commands: Commands) {
    for (entity, mut grass, food, mut visibility) in &mut q_grass {
        if food.remaining == 0.0 {
            grass.growth = 0;
            *visibility = Visibility::Hidden;
            commands.entity(entity).remove::<Food>();
        }
    }
}

pub fn regrow_grass(mut q_grass: Query<(Entity, &mut Grass, &mut Visibility), Without<Food>>, mut commands: Commands) {
    for (entity, mut grass, mut visibility) in &mut q_grass {
        grass.growth += 1;

        if grass.growth >= 100 {
            *visibility = Visibility::Visible;
            commands.entity(entity).insert(Food { remaining: 50.0 });
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
