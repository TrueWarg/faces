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
                        current: 0,
                        max: 0,
                    },
                    energy: RangedProp {
                        min: 0,
                        current: 0,
                        max: 0,
                    },
                    armor: 0,
                    evasion: 0,
                },
                attacks: vec![
                    DirectionalAttack::InevitableDamage { damage: 10 },
                    DirectionalAttack::Punch { damage: 2 },
                    DirectionalAttack::Punch { damage: 3 },
                ],
                abilities: vec![
                    Ability::WoundsLicking { health: 4, cost: 3 },
                ],
            },
            PartyMember {
                id: 1,
                name: "".to_string(),
                asset_path: "".to_string(),
                target: TargetProps {
                    health: RangedProp {
                        min: 0,
                        current: 0,
                        max: 0,
                    },
                    energy: RangedProp {
                        min: 0,
                        current: 0,
                        max: 0,
                    },
                    armor: 0,
                    evasion: 0,
                },
                attacks: vec![
                    DirectionalAttack::InevitableDamage { damage: 10 },
                    DirectionalAttack::InevitableDamage { damage: 10 },
                    DirectionalAttack::Punch { damage: 2 },
                    DirectionalAttack::Punch { damage: 3 },
                ],
                abilities: vec![
                    Ability::NeckTwist { damage: 0, cost: 0 },
                    Ability::NeckTwist { damage: 0, cost: 0 },
                    Ability::NeckTwist { damage: 0, cost: 0 },
                    Ability::NeckTwist { damage: 0, cost: 0 },
                    Ability::NeckTwist { damage: 0, cost: 0 },
                ],
            },
        ];
    }

    pub fn get_consumables(&self) -> Vec<ConsumableItem> {
        return vec![
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
            ConsumableItem::Dumplings(Dumplings { health: 10, energy: 10 }),
            ConsumableItem::Venison(Venison { health: 10, energy: 10 }),
        ];
    }
}
