use crate::rpg::TargetProps;
use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone, Debug)]
pub enum Ability {
    NeckTwist { damage: i32, cost: i32 },
    SuperPunch { damage: i32, cost: i32 },
    WoundsLicking { health: i32, cost: i32 },
    NeckGnawing { damage: i32, cost: i32 },
}

impl Ability {
    pub fn apply(&self, target: &mut TargetProps) {
        match self {
            Ability::NeckTwist { damage, .. } => {
                target.health.decrease(*damage);
            }
            Ability::WoundsLicking { health, .. } => {
                target.health.increase(*health);
            }
            Ability::SuperPunch { damage, .. } => {
                target.health.decrease(*damage);
            }
            Ability::NeckGnawing { damage, .. } => {
                target.health.decrease(*damage);
            }
        }
    }

    pub fn apply_cost(&self, target: &mut TargetProps) {
        match self {
            Ability::NeckTwist { damage, cost } => {
                target.energy.decrease(*cost);
            }
            Ability::WoundsLicking { health, cost } => {
                target.energy.decrease(*cost);
            }
            Ability::SuperPunch { damage, cost } => {
                target.energy.decrease(*cost);
            }
            Ability::NeckGnawing { damage, cost } => {
                target.energy.decrease(*cost);
            }
        }
    }
}
