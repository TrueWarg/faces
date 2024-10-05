use bevy::prelude::Resource;
use bevy::utils::HashMap;
use crate::dialog::DialogId;
use crate::dialog::entities::Dialog;
use crate::dialog::graph::{test_dialog_1, test_dialog_2};

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
    let (root_id, sticks) = test_dialog_1();
    result.insert(
        0,
        Dialog::from(
            DialogId(0),
            "Dialog 1".to_string(),
            root_id,
            sticks,
        ),
    );
    let (root_id, sticks) = test_dialog_2();

    result.insert(
        1,
        Dialog::from(
            DialogId(1),
            "Dialog 2".to_string(),
            root_id,
            sticks,
        ),
    );
    return result;
}
