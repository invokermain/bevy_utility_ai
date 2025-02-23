use bevy::prelude::{Entity, Event};
use uuid::Uuid;

#[derive(Event)]
pub struct InputCalculatedEvent {
    /// The Entity this calculation is for
    pub entity: Entity,
    /// The name of this input, also acts as an Id
    pub input: String,
    /// The Target Entity if it is a targeted input
    pub target: Option<Entity>,
    /// The calculated score
    pub score: f32,
}

#[derive(Event)]
pub struct ConsiderationCalculatedEvent {
    /// The Entity this calculation is for
    pub entity: Entity,
    /// The Consideration Id
    pub consideration: Uuid,
    /// The Decision Id that this Consideration is for
    pub decision: Uuid,
    /// The Target Entity if it is a targeted consideration
    pub target: Option<Entity>,
    /// The calculated score
    pub score: f32,
}

#[derive(Event)]
pub struct DecisionCalculatedEvent {
    /// The Entity this calculation is for
    pub entity: Entity,
    /// The Decision Id
    pub decision: Uuid,
    /// The Target Entity if it is a targeted decision
    pub target: Option<Entity>,
    /// The calculated score
    pub score: f32,
}
