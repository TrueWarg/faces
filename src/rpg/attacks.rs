use crate::rpg::TargetProps;
use bevy::prelude::Component;
use rand::Rng;

#[derive(Component, PartialEq, Clone, Debug)]
pub enum DirectionalAttack {
    InevitableDamage { damage: i32 },

    Punch { damage: i32 },

    Kick { damage: i32 },

    Bite { damage: i32 },

    PawStrike { damage: i32 },
}

pub enum AttackResult {
    Hit,
    Miss,
}

impl DirectionalAttack {
    pub fn damage(&self) -> i32 {
        match self {
            DirectionalAttack::InevitableDamage { damage } => damage.clone(),
            DirectionalAttack::Punch { damage } => damage.clone(),
            DirectionalAttack::Kick { damage } => damage.clone(),
            DirectionalAttack::Bite { damage } => damage.clone(),
            DirectionalAttack::PawStrike { damage } => damage.clone(),
        }
    }

    pub fn apply(&self, target: &mut TargetProps) -> AttackResult {
        match self {
            DirectionalAttack::InevitableDamage { damage } => {
                target.health.decrease(*damage);
                AttackResult::Hit
            }
            DirectionalAttack::Punch { damage } => {
                DirectionalAttack::default_damage_apply(*damage, target)
            }
            DirectionalAttack::Kick { damage } => {
                DirectionalAttack::default_damage_apply(*damage, target)
            }
            DirectionalAttack::Bite { damage } => {
                DirectionalAttack::default_damage_apply(*damage, target)
            }
            DirectionalAttack::PawStrike { damage } => {
                DirectionalAttack::default_damage_apply(*damage, target)
            }
        }
    }

    fn default_damage_apply(damage: i32, target: &mut TargetProps) -> AttackResult {
        let reduced_damage = if target.armor == 0 {
            damage as f32
        } else {
            damage as f32 - ((target.armor as f32) / 100.0) * damage as f32
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
