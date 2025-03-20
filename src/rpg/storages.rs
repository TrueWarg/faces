use bevy::prelude::Resource;
use crate::rpg::character::{Character, Class, Level};

#[derive(Resource)]
pub struct CharacterStorage;

impl CharacterStorage {
    pub fn get_characters(&self) -> Vec<Character> {
        return vec![
            Character {
                name: "Грозный Личик".to_string(),
                class: Class::FormidableFace {
                    strength: 1,
                    agility: 1,
                    stamina: 1,
                    fortitude: 1,
                    charisma: 1,
                },
                level: Level {
                    current: 1,
                    current_experience: 0,
                    experience_for_the_next: 2000,
                    available_points: 15,
                    characteristic_max_value: 5,
                },
            },
            Character {
                name: "Грозный Пёс".to_string(),
                class: Class::FormidableDog {
                    strength: 4,
                    agility: 4,
                    stamina: 4,
                    fortitude: 4,
                },
                level: Level {
                    current: 2,
                    current_experience: 3214,
                    experience_for_the_next: 4000,
                    available_points: 10,
                    characteristic_max_value: 6,
                },
            },
        ];
    }
}