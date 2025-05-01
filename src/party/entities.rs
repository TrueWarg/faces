use crate::rpg::{Ability, DirectionalAttack, RangedProp, TargetProps};
use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone)]
pub struct PartyMember {
    pub id: usize,
    pub name: String,
    pub asset_path: String,
    pub target: TargetProps,
    pub base_attack: i32,
    pub attacks: Vec<DirectionalAttack>,
    pub abilities: Vec<Ability>,
}

impl PartyMember {
    pub fn initial_formidable_face() -> Self {
        PartyMember {
            id: 0,
            name: "Грозный Личик".to_string(),
            base_attack: 15,
            asset_path: "".to_string(),
            target: TargetProps {
                health: RangedProp {
                    min: 0,
                    current: 113,
                    max: 113,
                },
                energy: RangedProp {
                    min: 0,
                    current: 24,
                    max: 24,
                },
                armor: 4,
                evasion: 4,
            },
            attacks: vec![DirectionalAttack::Punch { damage: 15 }],
            abilities: vec![Ability::NeckTwist {
                damage: 30,
                cost: 10,
            }],
        }
    }

    pub fn initial_formidable_dog() -> Self {
        PartyMember {
            id: 1,
            name: "Грозный Пёс".to_string(),
            base_attack: 15,
            asset_path: "".to_string(),
            target: TargetProps {
                health: RangedProp {
                    min: 0,
                    current: 113,
                    max: 113,
                },
                energy: RangedProp {
                    min: 0,
                    current: 24,
                    max: 24,
                },
                armor: 4,
                evasion: 4,
            },
            attacks: vec![DirectionalAttack::Bite { damage: 17 }],
            abilities: vec![Ability::WoundsLicking {
                health: 20,
                cost: 15,
            }],
        }
    }
}
