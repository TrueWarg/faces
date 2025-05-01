use crate::rpg::character::{Character, Class, Level};
use bevy::prelude::Resource;

#[derive(Resource)]
pub struct CharacterStorage {
    characters: Vec<Character>,
}

impl Default for CharacterStorage {
    fn default() -> Self {
        CharacterStorage {
            characters: vec![Character::initial_formidable_face()],
        }
    }
}

impl CharacterStorage {
    pub fn get(&self) -> Vec<Character> {
        self.characters.clone()
    }

    pub fn get_class_by_id(&self, id: usize) -> &Class {
        &self.characters[id].class
    }

    pub fn get_level_by_id(&self, id: usize) -> &Level {
        &self.characters[id].level
    }

    pub fn update_by_id(&mut self, id: usize, new_value: Character) {
        self.characters[id] = new_value;
    }

    pub fn update_class_by_id(&mut self, id: usize, class: Class) {
        self.characters[id].class = class;
    }

    pub fn update_level_by_id(&mut self, id: usize, level: Level) {
        self.characters[id].level = level;
    }

    pub fn update_exp_by_id(&mut self, id: usize, exp: i32) {
        self.characters[id].level.current_experience += exp;
    }

    pub fn add(&mut self, value: Character) {
        self.characters.push(value);
    }
}
