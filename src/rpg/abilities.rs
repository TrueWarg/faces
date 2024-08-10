use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone)]
pub enum Ability {
    NeckTwist {
        damage: i32,
        cost: i32,
    },
    WoundsLicking {
        health: i32,
        cost: i32,
    }
}
