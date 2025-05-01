use bevy::prelude::Resource;

use crate::party::entities::PartyMember;
use crate::rpg::Dumplings;
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
            members: vec![PartyMember::initial_formidable_face()],
        }
    }
}

impl PartyStateStorage {
    pub fn get_party_members(&self) -> Vec<PartyMember> {
        self.members.clone()
    }

    pub fn add_party_member(&mut self, member: PartyMember) {
        self.members.push(member);
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
