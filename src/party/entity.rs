use crate::rpg::{DirectionalAction, TargetProps};

pub struct PartyMember {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<Box<dyn DirectionalAction>>,
    pub abilities: Vec<Box<dyn DirectionalAction>>,
}

