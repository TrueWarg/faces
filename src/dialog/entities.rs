use std::collections::HashMap;
use bevy::prelude::Component;
use crate::dialog::graph::DialogStick;

#[derive(Component)]
pub struct Dialog {
    pub id: DialogId,
    pub label: Option<String>,
    pub root_id: usize,
    sticks: HashMap<usize, DialogStick>,
}

impl Dialog {
    pub fn from(
        id: DialogId,
        label: String,
        root_id: usize,
        sticks: HashMap<usize, DialogStick>,
    ) -> Dialog {
        return Dialog {
            id,
            label: Some(label),
            root_id,
            sticks,
        };
    }

    pub fn get_root_stick(&self) -> &DialogStick {
        return self.get_stick_at(self.root_id);
    }
    pub fn get_stick_at(&self, stick_id: usize) -> &DialogStick {
        return self.sticks.get(&stick_id).expect(&format!("No stick with id {stick_id}"));
    }
}

#[derive(Component)]
pub struct DialogId(pub usize);