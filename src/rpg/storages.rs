use crate::rpg::character::{Character, Class, Level};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CharacterStorage {
    characters: Vec<Character>,
}

impl Default for CharacterStorage {
    fn default() -> Self {
        CharacterStorage {
            characters: vec![Character {
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
            }],
        }
    }
}

impl CharacterStorage {
    pub fn get(&self) -> Vec<Character> {
        self.characters.clone()
    }

    pub fn update_by_id(&mut self, id: usize, new_value: Character) {
        self.characters[id] = new_value;
    }

    pub fn update_exp_by_id(&mut self, id: usize, exp: i32) {
        self.characters[id].level.current_experience += exp;
    }

    pub fn add(&mut self, value: Character) {
        self.characters.push(value);
    }
}
