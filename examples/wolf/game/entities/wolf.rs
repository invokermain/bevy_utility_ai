use bevy::{
    asset::AssetServer,
    ecs::{bundle::Bundle, system::Res},
    prelude::*,
};
use bevy_utility_ai::systems::make_decisions::EntityActionChangedEvent;
use rand::{thread_rng, Rng};

use crate::{
    game::{
        ai::wolf::HunterAI,
        systems::{drink::Thirst, food::Hunger, rest::Energy},
    },
    level::WolfText,
    utils::animations::{AnimationIndices, AnimationTimer},
};

#[derive(Bundle)]
pub struct WolfBundle {
    sprite: SpriteBundle,
    texture_atlas: TextureAtlas,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    hunger: Hunger,
    thirst: Thirst,
    energy: Energy,
    ai: HunterAI,
}

impl WolfBundle {
    pub fn new(
        transform: Transform,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let mut rng = thread_rng();
        let texture = asset_server.load("wolf.png");
        let layout = TextureAtlasLayout::from_grid(UVec2::new(16, 16), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        Self {
            sprite: SpriteBundle {
                texture,
                transform,
                ..default()
            },
            texture_atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: animation_indices.first,
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            )),
            ai: HunterAI {}, // this component enables the HunterAI behaviour
            hunger: Hunger {
                value: rng.gen_range(0.0..60.0),
                max: 100.0,
            },
            thirst: Thirst {
                value: rng.gen_range(0.0..60.0),
                max: 100.,
            },
            energy: Energy {
                value: rng.gen_range(5.0..100.0),
                max: 100.0,
            },
        }
    }
}

pub fn clear_wolf_text(
    mut e_action_changed: EventReader<EntityActionChangedEvent>,
    mut q_wolf_text: Query<&mut Text, With<WolfText>>,
) {
    for _ in e_action_changed.read() {
        if let Ok(mut text) = q_wolf_text.get_single_mut() {
            text.sections[0].value = "".to_string();
        }
    }
}
