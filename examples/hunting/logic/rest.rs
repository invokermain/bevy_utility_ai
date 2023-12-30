use crate::logic::ai::ActionRest;
use crate::logic::components::Energy;
use bevy::math::Vec3;
use bevy::prelude::{Query, With};
use bevy::transform::components::Transform;
use rand::Rng;

use super::ai::ActionIdle;

pub fn rest(mut q_energy: Query<&mut Energy, With<ActionRest>>) {
    for mut energy in q_energy.iter_mut() {
        energy.value += 1.0;
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
