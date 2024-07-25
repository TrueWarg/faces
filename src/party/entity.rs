use bevy::prelude::Component;
use crate::rpg::{Ability, DirectionalAttack, TargetProps};

#[derive(Component, PartialEq, Clone)]
pub struct PartyMember {
    pub id: String,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub attacks: Vec<DirectionalAttack>,
    pub abilities: Vec<Ability>,
}

