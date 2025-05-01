use bevy::prelude::Component;

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Character {
    pub name: String, // it is always unique id
    pub class: Class,
    pub level: Level,
}

impl Character {
    pub fn initial_formidable_face() -> Self {
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
                current: 0,
                current_experience: 0,
                experience_for_the_next: 500,
                available_points: 10,
                characteristic_max_value: 5,
            },
        }
    }

    pub fn initial_formidable_dog() -> Self {
        Character {
            name: "Грозный Пёс".to_string(),
            class: Class::FormidableDog {
                strength: 1,
                agility: 1,
                stamina: 1,
                fortitude: 1,
            },
            level: Level {
                current: 0,
                current_experience: 0,
                experience_for_the_next: 500,
                available_points: 10,
                characteristic_max_value: 5,
            },
        }
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub enum Class {
    FormidableFace {
        strength: i32,
        agility: i32,
        stamina: i32,
        fortitude: i32,
        charisma: i32,
    },

    FormidableDog {
        strength: i32,
        agility: i32,
        stamina: i32,
        fortitude: i32,
    },
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Level {
    pub current: i32,
    pub current_experience: i32,
    pub experience_for_the_next: i32,
    pub available_points: i32,
    pub characteristic_max_value: i32,
}

impl Level {
    pub fn up_level(&self) -> Level {
        Level {
            current: self.current + 1,
            current_experience: 0,
            experience_for_the_next: self.experience_for_the_next * 2,
            available_points: 0,
            characteristic_max_value: self.characteristic_max_value + 1,
        }
    }
}
