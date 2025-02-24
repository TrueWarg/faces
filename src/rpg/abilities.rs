use bevy::prelude::Component;
use crate::rpg::TargetProps;

#[derive(Component, PartialEq, Clone, Debug)]
pub enum Ability {
    NeckTwist {
        damage: i32,
        cost: i32,
    },
    WoundsLicking {
        health: i32,
        cost: i32,
    },
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
        }
    }
}