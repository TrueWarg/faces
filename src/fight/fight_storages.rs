use crate::fight::{Enemy, Fight};
use crate::rpg::{RangedProp, TargetProps};

struct FightStorages;

impl FightStorages {
    fn load(&self, id: usize) -> Fight {
        return Fight {
            id,
            arena_bg_path: "assets/background/test_bg.png".to_string(),
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
        };
    }
}