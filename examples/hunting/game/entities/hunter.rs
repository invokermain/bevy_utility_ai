use bevy::asset::AssetServer;
use bevy::ecs::bundle::Bundle;
use bevy::ecs::system::Res;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::sprite::{SpriteBundle, SpriteSheetBundle};
use bevy::transform::components::Transform;

use crate::game::ai::hunter::HunterAI;
use crate::game::systems::food::Hunger;
use crate::game::systems::rest::Energy;
use crate::game::systems::water::Thirst;
use crate::layers::ACTOR_LAYER;

#[derive(Bundle)]
pub struct HunterBundle {
    sprite: SpriteBundle,
    hunger: Hunger,
    thirst: Thirst,
    energy: Energy,
    ai: HunterAI,
}

impl HunterBundle {
    pub fn new(position: Vec2, asset_server: Res<AssetServer>) -> Self {
        let texture = asset_server.load("wolf.png");
        let layout =
            TextureAtlasLayout::from_grid(Vec2::new(24.0, 24.0), 7, 1, None, None);
        let texture_atlas_layout = texture_atlas_layouts.add(layout);
        Self {
            sprite: SpriteSheetBundle {
                texture: texture,
                transform: Transform::from_translation(position.extend(ACTOR_LAYER)),
                ..default()
            },
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
