use crate::dialog::graph::DialogStick;
use bevy::prelude::Component;
use std::collections::HashMap;

#[derive(Component)]
pub struct Dialog {
    pub id: DialogId,
    pub label: Option<String>,
    pub bg_path: String,
    pub character_path: String,
    pub root_id: usize,
    sticks: HashMap<usize, DialogStick>,
}

impl Dialog {
    pub fn from(
        id: DialogId,
        label: String,
        bg_path: String,
        character_path: String,
        root_id: usize,
        sticks: HashMap<usize, DialogStick>,
    ) -> Dialog {
        Dialog {
            id,
            label: Some(label),
            bg_path,
            character_path,
            root_id,
            sticks,
        }
    }

    pub fn get_root_stick(&self) -> &DialogStick {
        self.get_stick_at(self.root_id)
    }
    pub fn get_stick_at(&self, stick_id: usize) -> &DialogStick {
        self.sticks
            .get(&stick_id)
            .expect(&format!("No stick with id {stick_id}"))
    }
}

#[derive(Component)]
pub struct DialogId(pub usize);
