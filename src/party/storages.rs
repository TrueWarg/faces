use bevy::prelude::Resource;

use crate::party::entities::PartyMember;
use crate::rpg::Dumplings;
use crate::rpg::RangedProp;
use crate::rpg::TargetProps;
use crate::rpg::Venison;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack};

#[derive(Resource)]
pub struct PartyStateStorage {
    members: Vec<PartyMember>,
}

impl Default for PartyStateStorage {
    fn default() -> Self {
        PartyStateStorage {
            members: vec![PartyMember {
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
            }],
        }
    }
}

impl PartyStateStorage {
    pub fn get_party_members(&self) -> Vec<PartyMember> {
        self.members.clone()
    }

    pub fn update_base_attack_by_id(&mut self, id: usize, base_attack: i32) {
        self.members[id].base_attack = base_attack;
    }

    pub fn update_target_props_by_id(&mut self, id: usize, target_props: TargetProps) {
        self.members[id].target = target_props;
    }

    pub fn update_attacks_by_id(&mut self, id: usize, attacks: Vec<DirectionalAttack>) {
        self.members[id].attacks = attacks;
    }

    pub fn update_abilities_by_id(&mut self, id: usize, abilities: Vec<Ability>) {
        self.members[id].abilities = abilities;
    }

    pub fn get_consumables(&self) -> Vec<ConsumableItem> {
        vec![
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
            ConsumableItem::Dumplings(Dumplings {
                health: 10,
                energy: 10,
            }),
            ConsumableItem::Venison(Venison {
                health: 20,
                energy: 5,
            }),
        ]
    }
}
