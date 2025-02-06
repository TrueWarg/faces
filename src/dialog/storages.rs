use bevy::prelude::Resource;
use bevy::utils::HashMap;

use crate::dialog::entities::Dialog;
use crate::level::{BLOND_FIRST_DIALOG, CRAZY_MAN_DIALOG, crazy_man_dialog, HALL_GUARDIAN_FIRST_DIALOG, hall_guardian_first_dialog, HALL_GUARDIAN_SECOND_DIALOG, hall_guardian_second_dialog, TABLE_1_DIALOG, table_1_dialog, TABLE_2_DIALOG, table_2_dialog, table_3_dialog, TABLE_3_DIALOG};
use crate::level::BLOND_GIVE_DUMPLINGS_DIALOG;
use crate::level::blond_give_dumplings_dialog;
use crate::level::BLOND_TAKE_DUMPLINGS_DIALOG;
use crate::level::blond_take_dumplings_dialog;
use crate::level::GUARDIAN_FIRST_DIALOG;
use crate::level::guardian_first_dialog;
use crate::level::GUARDIAN_SECOND_DIALOG;
use crate::level::guardian_second_dialog;
use crate::level::guardian_third_dialog;
use crate::level::GUARDIAN_THIRD_DIALOG;
use crate::level::blond_first_dialog;
use crate::level::COURIER_DIALOG;
use crate::level::DREVNIRA_DIALOG;
use crate::level::gopniks_dialog;
use crate::level::GOPNIKS_DIALOG;
use crate::level::old_woman_drevnira_dialog;
use crate::level::courier_dialog;
use crate::level::SLEEPING_FORMIDABLE_DOG_DIALOG;
use crate::level::sleeping_formidable_dog_dialog;

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
    result.insert(SLEEPING_FORMIDABLE_DOG_DIALOG, sleeping_formidable_dog_dialog());
    result.insert(DREVNIRA_DIALOG, old_woman_drevnira_dialog());
    result.insert(BLOND_FIRST_DIALOG, blond_first_dialog());
    result.insert(GOPNIKS_DIALOG, gopniks_dialog());
    result.insert(BLOND_GIVE_DUMPLINGS_DIALOG, blond_give_dumplings_dialog());
    result.insert(BLOND_TAKE_DUMPLINGS_DIALOG, blond_take_dumplings_dialog());
    result.insert(GUARDIAN_FIRST_DIALOG, guardian_first_dialog());
    result.insert(GUARDIAN_SECOND_DIALOG, guardian_second_dialog());
    result.insert(GUARDIAN_THIRD_DIALOG, guardian_third_dialog());
    result.insert(HALL_GUARDIAN_FIRST_DIALOG, hall_guardian_first_dialog());
    result.insert(HALL_GUARDIAN_SECOND_DIALOG, hall_guardian_second_dialog());
    result.insert(TABLE_1_DIALOG, table_1_dialog());
    result.insert(TABLE_2_DIALOG, table_2_dialog());
    result.insert(TABLE_3_DIALOG, table_3_dialog());
    result.insert(CRAZY_MAN_DIALOG, crazy_man_dialog());

    return result;
}

#[derive(Resource, Default)]
pub struct SelectedVariantsSource(HashMap<usize, Vec<usize>>);

impl SelectedVariantsSource {
    pub fn produce(&mut self, dialog_id: usize, variant_id: usize) {
        let mut variants = self.0.get_mut(&dialog_id);
        match variants {
            None => {
                self.0.insert(dialog_id, vec![variant_id]);
            }
            Some(items) => {
                items.push(variant_id);
            }
        }
    }

    pub fn consume(&mut self, dialog_id: &usize) -> Option<Vec<usize>> {
        return self.0.remove(dialog_id);
    }
}

