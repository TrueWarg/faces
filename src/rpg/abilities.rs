use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone)]
pub enum Ability {
    NeckTwist {
        damage: i32,
    }
}
