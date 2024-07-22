use bevy::prelude::Component;

use crate::rpg::{DirectionalAttack, TargetProps};

#[derive(Component)]
pub struct Fight {
    pub id: usize,
    pub arena_bg_path: String,
    pub enemies: Vec<Enemy>,
}

#[derive(Component)]
pub struct Enemy {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<DirectionalAttack>,
}

#[derive(Component)]
pub struct FightId(pub usize);
