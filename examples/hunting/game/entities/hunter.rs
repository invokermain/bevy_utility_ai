use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Res;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::SpriteSheetBundle;
use bevy::transform::components::Transform;

use crate::game::ai::hunter::HunterAI;
use crate::game::systems::food::Hunger;
use crate::game::systems::rest::Energy;
use crate::game::systems::water::Thirst;
use crate::layers::ACTOR_LAYER;
use crate::utils::animations::{AnimationIndices, AnimationTimer};

#[derive(Bundle)]
pub struct HunterBundle {
    sprite: SpriteSheetBundle,
    animation_indices: AnimationIndices,
    animation_timer: AnimationTimer,
    hunger: Hunger,
    thirst: Thirst,
    energy: Energy,
    ai: HunterAI,
}

impl HunterBundle {
    pub fn new(
        position: Vec2,
        asset_server: &Res<AssetServer>,
        texture_atlas_layouts: &mut ResMut<Assets<TextureAtlasLayout>>,
    ) -> Self {
        let texture = asset_server.load("wolf.png");
        let layout =
            TextureAtlasLayout::from_grid(Vec2::new(16.0, 16.0), 4, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        Self {
            sprite: SpriteSheetBundle {
                texture,
                atlas: TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                transform: Transform::from_translation(position.extend(ACTOR_LAYER))
                    .with_scale(Vec3::splat(2.0)),
                ..default()
            },
            animation_indices,
            animation_timer: AnimationTimer(Timer::from_seconds(
                0.25,
                TimerMode::Repeating,
            )),
            ai: HunterAI {}, // this component enables the HunterAI behaviour
            hunger: Hunger {
                value: 0.0,
                max: 100.0,
            },
            thirst: Thirst {
                value: 0.,
                max: 100.,
            },
            energy: Energy {
                value: 100.0,
                max: 100.0,
            },
        }
    }
}
