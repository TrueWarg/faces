use bevy::prelude::Component;
use crate::rpg::{DirectionalAction, TargetProps};

#[derive(Component)]
pub enum ConsumableItem {
    Dumplings(Dumplings),
    Venison(Venison),
}

#[derive(Component)]
pub struct Dumplings {
    pub health: i32,
    pub energy: i32,
}

#[derive(Component)]
pub struct Venison {
    pub health: i32,
    pub energy: i32,
}
