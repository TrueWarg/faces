use bevy::prelude::{Resource, Val};
use bevy::utils::HashMap;

use crate::fight::{Enemy, Fight, FightId};
use crate::rpg::{RangedProp, TargetProps};

#[derive(Resource)]
pub struct FightStorage;

impl FightStorage {
    pub fn load(&self, id: &usize) -> Option<Fight> {
        let mut fights = test_fights();
        return fights.remove(id)

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
                    relative_x: 40.0,
                    relative_y: 20.0,
                    scale: 0.3,
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
                    attacks: vec![],
                },
                Enemy {
                    id: 1,
                    name: "".to_string(),
                    asset_path: "npc/red_chad.png".to_string(),
                    relative_x: 20.0,
                    relative_y: 15.0,
                    scale: 0.5,
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
                    attacks: vec![],
                },
                Enemy {
                    id: 2,
                    name: "".to_string(),
                    asset_path: "npc/funny_gopnik.png".to_string(),
                    relative_x: 60.0,
                    relative_y: 15.0,
                    scale: 0.4,
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
                    attacks: vec![],
                },
                Enemy {
                    id: 3,
                    name: "".to_string(),
                    asset_path: "npc/funny_gopnik.png".to_string(),
                    relative_x: 65.0,
                    relative_y: 15.0,
                    scale: 0.4,
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
                    attacks: vec![],
                },
            ],
        },
    );
    test_fights.insert(
        TEST_FIGHT_ID_1.0,
        Fight {
            id: TEST_FIGHT_ID_1,
            arena_bg_path: "background/test_bg2.png".to_string(),
            enemies: vec![
                Enemy {
                    id: 0,
                    name: "".to_string(),
                    asset_path: "".to_string(),
                    relative_x: 50.0,
                    relative_y: 50.0,
                    scale: 0.8,
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
                    attacks: vec![],
                },
            ],
        },
    );

    return test_fights;
}

pub const TEST_FIGHT_ID_0: FightId = FightId(10);
pub const TEST_FIGHT_ID_1: FightId = FightId(20);
