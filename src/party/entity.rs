use crate::rpg::{Ability, DirectionalAttack, TargetProps};

pub struct PartyMember {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<DirectionalAttack>,
    pub abilities: Vec<Ability>,
}

