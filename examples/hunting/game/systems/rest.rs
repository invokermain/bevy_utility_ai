use crate::game::ai::actions::{ActionIdle, ActionRest};
use bevy::math::Vec3;
use bevy::prelude::{Component, Query, With};
use bevy::transform::components::Transform;
use rand::Rng;

#[derive(Component)]
pub struct Energy {
    pub value: f32,
    pub max: f32,
}

pub fn rest(mut q_energy: Query<&mut Energy, With<ActionRest>>) {
    for mut energy in q_energy.iter_mut() {
        energy.value += 0.50;
        if energy.value >= energy.max {
            energy.value = energy.max
        }
    }
}

pub fn idle(mut q_energy: Query<(&mut Energy, &mut Transform), With<ActionIdle>>) {
    let mut rng = rand::thread_rng();

    for (mut energy, mut transform) in q_energy.iter_mut() {
        energy.value += 0.1;
        if energy.value >= energy.max {
            energy.value = energy.max
        }

        transform.translation +=
            Vec3::new(rng.gen_range(-2.5..2.5), rng.gen_range(-2.5..2.5), 0.0);
    }
}
