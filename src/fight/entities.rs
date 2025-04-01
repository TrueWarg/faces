use bevy::prelude::Component;

use crate::rpg::{DirectionalAttack, TargetProps};

#[derive(Component)]
pub struct Fight {
    pub id: FightId,
    pub arena_bg_path: String,
    pub enemies: Vec<Enemy>,
}

#[derive(Component)]
pub struct Enemy {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub relative_x: f32,
    pub relative_y: f32,
    pub relative_height: f32,
    pub target: TargetProps,
    pub attacks: Vec<DirectionalAttack>,
}

#[derive(Component)]
pub struct FightId(pub usize);

pub enum ActionTarget {
    Enemy,
    Ally,
}

pub trait GetActionTarget {
    fn action_target(&self) -> ActionTarget;
}
