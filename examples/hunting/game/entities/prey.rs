use bevy::asset::{AssetServer, Assets};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{
    default, Bundle, Component, Res, ResMut, SpriteSheetBundle, TextureAtlas,
    TextureAtlasLayout, Timer, TimerMode, Transform,
};
use rand::Rng;

use crate::game::ai::prey::PreyAI;
use crate::game::systems::food::Hunger;
use crate::game::systems::water::Thirst;
use crate::layers::ACTOR_LAYER;
use crate::utils::animations::{AnimationIndices, AnimationTimer};

#[derive(Bundle)]
pub struct PreyBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    prey_ai: PreyAI, // this component enables the PreyAI's behaviour
    thirst: Thirst,
    hunger: Hunger,
    personality: PreyPersonality,
}

impl PreyBundle {
    pub fn new(
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture = asset_server.load("goose.png");
        let layout =
            TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        let mut rng = rand::thread_rng();
        Self {
            sprite: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_translation(Vec3::new(
                    rng.gen_range(-1000.0..=1000.0),
                    rng.gen_range(-1000.0..=1000.0),
                    ACTOR_LAYER,
                )),
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            )),
            prey_ai: PreyAI {},
            thirst: Thirst {
                value: rng.gen_range(0.0..=100.0),
                max: 100.,
            },
            hunger: Hunger {
                value: rng.gen_range(0.0..=100.0),
                max: 100.,
            },
            personality: PreyPersonality {
                confidence: rng.gen_range(0.5..=1.0),
            },
        }
    }
}

#[derive(Component)]
pub struct PreyPersonality {
    pub confidence: f32, // between 0 and 1
}
