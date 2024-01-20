use bevy::ecs::reflect::ReflectComponent;
use bevy::prelude::{Component, Reflect};
use bevy::reflect::std_traits::ReflectDefault;

// Define our Actions, when a decision is made these components will be added to the
// entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionHunt {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionRest {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionEat {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionDrink {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionIdle {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionFlee {}

#[derive(Component, Reflect, Default)]
#[reflect(Component, Default)]
pub struct ActionHerd {}
