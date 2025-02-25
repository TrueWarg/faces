use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::fight::{Enemy, Fight, FightId};
use crate::rpg::{DirectionalAttack, RangedProp, TargetProps};

#[derive(Resource)]
pub struct FightStorage;

impl FightStorage {
    pub fn load(&self, id: &usize) -> Option<Fight> {
        let mut fights = test_fights();
        return fights.remove(id);
    }

    pub fn get_all(&self) -> Vec<Fight> {
        let mut fights = test_fights();
        let mut items = vec![];
        for (_, item) in fights {
            items.push(item)
        }
        return items;
    }
}

pub fn test_fights() -> HashMap<usize, Fight> {
    let mut test_fights = HashMap::new();
    test_fights.insert(
        TEST_FIGHT_ID_0.0,
        Fight {
            id: TEST_FIGHT_ID_0,
            arena_bg_path: "background/fight/gopniks_1.png".to_string(),
            enemies: vec![
                Enemy {
                    id: 0,
                    name: "".to_string(),
                    asset_path: "npc/gopnik.png".to_string(),
                    relative_x: 50.0,
                    relative_y: 55.0,
                    relative_height: 30.0,
                    target: TargetProps {
                        health: RangedProp {
                            min: 0,
                            current: 100,
                            max: 100,
                        },
                        energy: RangedProp {
                            min: 0,
                            current: 0,
                            max: 0,
                        },
                        armor: 10,
                        evasion: 25,
                    },
                    attacks: vec![
                        DirectionalAttack::Punch {
                            damage: 20,
                        },
                        DirectionalAttack::Punch {
                            damage: 30,
                        },
                    ],
                },
                Enemy {
                    id: 1,
                    name: "".to_string(),
                    asset_path: "npc/red_chad.png".to_string(),
                    relative_x: 25.0,
                    relative_y: 38.0,
                    relative_height: 55.0,
                    target: TargetProps {
                        health: RangedProp {
                            min: 0,
                            current: 200,
                            max: 200,
                        },
                        energy: RangedProp {
                            min: 0,
                            current: 0,
                            max: 0,
                        },
                        armor: 30,
                        evasion: 0,
                    },
                    attacks: vec![
                        DirectionalAttack::Punch {
                            damage: 20,
                        },
                        DirectionalAttack::InevitableDamage {
                            damage: 5,
                        },
                    ],
                },
                Enemy {
                    id: 2,
                    name: "".to_string(),
                    asset_path: "npc/funny_gopnik.png".to_string(),
                    relative_x: 65.0,
                    relative_y: 50.0,
                    relative_height: 30.0,
                    target: TargetProps {
                        health: RangedProp {
                            min: 0,
                            current: 40,
                            max: 40,
                        },
                        energy: RangedProp {
                            min: 0,
                            current: 0,
                            max: 0,
                        },
                        armor: 0,
                        evasion: 70,
                    },
                    attacks: vec![
                        DirectionalAttack::InevitableDamage {
                            damage: 5,
                        },
                    ],
                },
                Enemy {
                    id: 3,
                    name: "".to_string(),
                    asset_path: "npc/funny_gopnik.png".to_string(),
                    relative_x: 73.0,
                    relative_y: 50.0,
                    relative_height: 30.0,
                    target: TargetProps {
                        health: RangedProp {
                            min: 0,
                            current: 40,
                            max: 40,
                        },
                        energy: RangedProp {
                            min: 0,
                            current: 0,
                            max: 0,
                        },
                        armor: 0,
                        evasion: 70,
                    },
                    attacks: vec![
                        DirectionalAttack::Punch {
                            damage: 15,
                        },
                    ],
                },
            ],
        },
    );
    return test_fights;
}

pub const TEST_FIGHT_ID_0: FightId = FightId(10);
