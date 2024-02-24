use crate::game::ai::actions::{ActionFlee, ActionHerd};
use crate::game::ai::hunter::HunterAI;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::query::{With, Without};
use bevy::ecs::removal_detection::RemovedComponents;
use bevy::ecs::system::{Commands, ParamSet, Query};
use bevy::math::{Vec2, Vec3, Vec3Swizzles};
use bevy::transform::components::Transform;
use bevy_utility_ai::ActionTarget;
use rand::Rng;

#[derive(Component)]
pub struct FleeTo {
    point: Vec2,
}

pub fn flee(
    mut q_subject: Query<
        (Entity, &mut Transform, &ActionTarget, Option<&FleeTo>),
        With<ActionFlee>,
    >,
    q_hunter: Query<&Transform, (With<HunterAI>, Without<ActionFlee>)>,
    mut commands: Commands,
) {
    for (entity, mut transform, action_target, flee_to) in &mut q_subject {
        if let Some(flee_to) = flee_to {
            let flee_direction = (flee_to.point - transform.translation.xy()).normalize();

            transform.translation += flee_direction.extend(2.0) * 3.0;

            if transform.translation.xy().distance(flee_to.point) <= 10.0 {
                commands.entity(entity).remove::<FleeTo>();
            } else {
                continue;
            }
        }

        // pick a random point
        let mut rng = rand::thread_rng();
        let target_transform = q_hunter.get(action_target.target).unwrap();
        let target_positon = target_transform.translation.xy();
        let flee_direction = (transform.translation.xy() - target_positon).normalize();
        let mut flee_to_point = transform.translation.xy() + flee_direction * 1000.0;
        flee_to_point +=
            Vec2::new(rng.gen_range(-250.0..250.0), rng.gen_range(-250.0..250.0));
        flee_to_point =
            flee_to_point.clamp(Vec2::new(-1000.0, 1000.0), Vec2::new(-1000.0, 1000.0));
        commands.entity(entity).insert(FleeTo {
            point: flee_to_point,
        });
    }
}

pub fn remove_flee_to(
    mut er_remove_components: RemovedComponents<ActionFlee>,
    mut commands: Commands,
) {
    for entity in er_remove_components.read() {
        commands.entity(entity).remove::<FleeTo>();
    }
}

pub fn herd(
    mut paramset: ParamSet<(
        Query<(&mut Transform, &ActionTarget), With<ActionHerd>>,
        Query<&Transform>,
    )>,
) {
    let targets: Vec<Entity> = paramset
        .p0()
        .iter()
        .map(|(_, action_target)| action_target.target)
        .collect();

    let target_positions: Vec<Option<Vec3>> = targets
        .iter()
        .map(|target| {
            paramset
                .p1()
                .get(*target)
                .ok()
                .map(|transform| transform.translation)
        })
        .collect();

    let mut rng = rand::thread_rng();

    for ((mut transform, _), target_position) in
        &mut paramset.p0().iter_mut().zip(target_positions)
    {
        if let Some(target_position) = target_position {
            let vector = target_position - transform.translation;
            let distance = vector.length();
            let direction = vector.normalize();

            if distance > 15.0 {
                transform.translation += direction * 3.0;
            } else {
                transform.translation +=
                    Vec3::new(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0), 0.0);
            }
        }
    }
}
