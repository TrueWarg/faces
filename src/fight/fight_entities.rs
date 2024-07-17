use crate::rpg::{DirectionalAction, TargetProps};

pub struct Fight {
    pub id: usize,
    pub arena_bg_path: String,
    pub enemies: Vec<Enemy>,
}

pub struct Enemy {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<Box<dyn DirectionalAction>>,
}

pub struct Ally {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<Box<dyn DirectionalAction>>,
    pub abilities: Vec<Box<dyn DirectionalAction>>,
}

