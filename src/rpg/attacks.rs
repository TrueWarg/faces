use bevy::prelude::Component;
use crate::rpg::DirectionalAction;
use crate::rpg::TargetProps;

#[derive(Component, PartialEq, Clone, Debug)]
pub enum DirectionalAttack {
    InevitableDamage {
        damage: i32,
    },

    Punch {
        damage: i32,
    },
}

impl DirectionalAttack {
    pub fn damage(&self) -> i32 {
        return match self {
            DirectionalAttack::InevitableDamage { damage } => { damage.clone() }
            DirectionalAttack::Punch { damage } => { damage.clone() }
        };
    }
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