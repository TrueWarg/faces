use bevy::prelude::Component;
use crate::rpg::{DirectionalAction, TargetProps};

#[derive(Component, PartialEq, Clone)]
pub enum DirectionalAttack {
    InevitableDamage {
        damage: i32,
    },

    Punch {
        damage: i32,
    },
}

impl DirectionalAction for DirectionalAttack {
    fn apply(&self, target: &mut TargetProps) {
        match self {
            DirectionalAttack::InevitableDamage { damage } => {
                target.health.decrease(*damage);
            }
            DirectionalAttack::Punch { damage } => {
                target.health.decrease(*damage);
            }
        }
    }
}
