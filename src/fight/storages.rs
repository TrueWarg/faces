use std::collections::hash_map::{IntoIter, IntoValues};
use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::fight::{Enemy, Fight};
use crate::rpg::{RangedProp, TargetProps};

#[derive(Resource)]
pub struct FightStorage;

impl FightStorage {
    pub fn load(&self, id: usize) -> Option<Fight> {
        let mut fights = test_fights();
        return fights.remove(&id)

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
        TEST_FIGHT_ID_0,
        Fight {
            id: TEST_FIGHT_ID_0,
            arena_bg_path: "background/test_bg.png".to_string(),
            enemies: vec![
                Enemy {
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
                    attacks: vec![],
                },
                Enemy {
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
                    attacks: vec![],
                },
                Enemy {
                    id: 2,
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
                    attacks: vec![],
                },
                Enemy {
                    id: 3,
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
                    attacks: vec![],
                },
            ],
        },
    );
    test_fights.insert(
        TEST_FIGHT_ID_1,
        Fight {
            id: TEST_FIGHT_ID_1,
            arena_bg_path: "background/test_bg2.png".to_string(),
            enemies: vec![
                Enemy {
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
                    attacks: vec![],
                },
            ],
        },
    );

    return test_fights;
}

pub const TEST_FIGHT_ID_0: usize = 10;
pub const TEST_FIGHT_ID_1: usize = 20;
