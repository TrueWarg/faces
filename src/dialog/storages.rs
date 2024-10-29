use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::dialog::entities::Dialog;
use crate::level::{COURIER_DIALOG, courier_dialog};

#[derive(Resource)]
pub struct DialogsStorage;

impl DialogsStorage {
    pub fn get_by_id(&self, id: &usize) -> Option<Dialog> {
        return test_dialogs().remove(id);
    }

    pub fn get_all(&self) -> HashMap<usize, Dialog> {
        return test_dialogs();
    }
}

fn test_dialogs() -> HashMap<usize, Dialog> {
    let mut result = HashMap::default();
    result.insert(COURIER_DIALOG, courier_dialog());
    return result;
}
