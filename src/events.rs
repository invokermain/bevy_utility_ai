use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub struct InputCalculatedEvent {
    pub entity: Entity,
    pub input: String,
    pub target: Option<Entity>,
    pub score: f32,
}

#[derive(Event)]
pub struct ConsiderationCalculatedEvent {
    pub entity: Entity,
    pub decision: String,
    pub target: Option<Entity>,
    pub consideration_name: String,
    pub score: f32,
}

#[derive(Event)]
pub struct DecisionCalculatedEvent {
    pub entity: Entity,
    pub decision: String,
    pub target: Option<Entity>,
    pub score: f32,
}
