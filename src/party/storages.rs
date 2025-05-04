use bevy::prelude::Resource;

use crate::party::entities::PartyMember;
use crate::rpg::TargetProps;
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack};

#[derive(Resource)]
pub struct PartyStateStorage {
    members: Vec<PartyMember>,
    consumable: Vec<ConsumableItem>,
}

impl Default for PartyStateStorage {
    fn default() -> Self {
        PartyStateStorage {
            members: vec![PartyMember::initial_formidable_face()],
            consumable: vec![ConsumableItem::default_dumplings()],
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
        self.consumable.clone()
    }

    pub fn add_consumable(&mut self, consumable: ConsumableItem) {
        self.consumable.push(consumable);
    }

    pub fn add_consumables(&mut self, consumables: Vec<ConsumableItem>) {
        for consumable in consumables {
            self.consumable.push(consumable);
        }
    }

    pub fn remove_consumable_by_id(&mut self, id: usize) {
        self.consumable.remove(id);
    }
}
