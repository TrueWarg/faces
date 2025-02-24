use bevy::prelude::Component;
use rand::{random, Rng};
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

pub enum AttackResult {
    Hit,
    Miss,
}

impl DirectionalAttack {
    pub fn damage(&self) -> i32 {
        return match self {
            DirectionalAttack::InevitableDamage { damage } => { damage.clone() }
            DirectionalAttack::Punch { damage } => { damage.clone() }
        };
    }

    pub fn apply(&self, target: &mut TargetProps) -> AttackResult {
        match self {
            DirectionalAttack::InevitableDamage { damage } => {
                target.health.decrease(*damage);
                AttackResult::Hit
            }
            DirectionalAttack::Punch { damage } => {
                let reduced_damage = if target.armor == 0 {
                    *damage as f32
                } else {
                    *damage as f32 - ((target.armor as f32) / 100.0) * *damage as f32
                };
                let miss_p = target.evasion;
                let mut rng = rand::thread_rng();
                let rand = rng.gen_range(1..=100);

                if rand <= miss_p {
                    AttackResult::Miss
                } else {
                    target.health.decrease(reduced_damage as i32);
                    AttackResult::Hit
                }
            }
        }
    }
}