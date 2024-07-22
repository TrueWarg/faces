use crate::party::entity::PartyMember;
use crate::rpg::DirectionalAction;
use crate::rpg::Dumplings;
use crate::rpg::InevitableDamage;
use crate::rpg::NeckTwist;
use crate::rpg::Punch;
use crate::rpg::RangedProp;
use crate::rpg::TargetProps;
use crate::rpg::Venison;

struct PartyStateStorage;

impl PartyStateStorage {
    fn get_fight_party_members(&self) -> Vec<PartyMember> {
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
                    Box::new(Punch { damage: 10 }),
                    Box::new(Punch { damage: 10 }),
                    Box::new(InevitableDamage { damage: 10 }),
                    Box::new(Punch { damage: 10 }),
                    Box::new(Punch { damage: 10 }),
                ],
                abilities: vec![
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
                    Box::new(NeckTwist { damage: 10 }),
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
                    Box::new(Punch { damage: 10 }),
                    Box::new(Punch { damage: 10 }),
                    Box::new(InevitableDamage { damage: 10 }),
                ],
                abilities: vec![],
            },
        ];
    }

    fn get_items(&self) -> Vec<Box<dyn DirectionalAction>> {
        return vec![
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Dumplings { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
            Box::new(Venison { health: 0, energy: 0 }),
        ];
    }
}
