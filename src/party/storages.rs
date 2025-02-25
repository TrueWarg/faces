use bevy::prelude::Resource;

use crate::party::entities::PartyMember;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack};
use crate::rpg::Dumplings;
use crate::rpg::RangedProp;
use crate::rpg::TargetProps;
use crate::rpg::Venison;

#[derive(Resource)]
pub struct PartyStateStorage;

impl PartyStateStorage {
    pub fn get_fight_party_members(&self) -> Vec<PartyMember> {
        return vec![
            PartyMember {
                id: 0,
                name: "".to_string(),
                asset_path: "".to_string(),
                target: TargetProps {
                    health: RangedProp {
                        min: 0,
                        current: 100,
                        max: 100,
                    },
                    energy: RangedProp {
                        min: 0,
                        current: 20,
                        max: 20,
                    },
                    armor: 10,
                    evasion: 50,
                },
                attacks: vec![
                    DirectionalAttack::InevitableDamage { damage: 40 },
                    DirectionalAttack::Punch { damage: 20 },
                    DirectionalAttack::Punch { damage: 15 },
                ],
                abilities: vec![
                    Ability::WoundsLicking { health: 25, cost: 5 },
                ],
            },
            PartyMember {
                id: 1,
                name: "".to_string(),
                asset_path: "".to_string(),
                target: TargetProps {
                    health: RangedProp {
                        min: 0,
                        current: 120,
                        max: 120,
                    },
                    energy: RangedProp {
                        min: 0,
                        current: 15,
                        max: 15,
                    },
                    armor: 10,
                    evasion: 20,
                },
                attacks: vec![
                    DirectionalAttack::InevitableDamage { damage: 40 },
                    DirectionalAttack::InevitableDamage { damage: 15 },
                    DirectionalAttack::Punch { damage: 20 },
                    DirectionalAttack::Punch { damage: 15 },
                ],
                abilities: vec![
                    Ability::NeckTwist { damage: 50, cost: 10 },
                    Ability::NeckTwist { damage: 50, cost: 10 },
                    Ability::NeckTwist { damage: 50, cost: 10 },
                    Ability::NeckTwist { damage: 50, cost: 10 },
                    Ability::NeckTwist { damage: 50, cost: 10 },
                ],
            },
        ];
    }

    pub fn get_consumables(&self) -> Vec<ConsumableItem> {
        return vec![
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 20, energy: 5 }),
        ];
    }
}
