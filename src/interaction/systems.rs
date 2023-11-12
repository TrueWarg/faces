use bevy::prelude::{Input, KeyCode, Plugin, Query, Res, Transform, Update};

use crate::core::components::Description;

use super::component::{ActiveInteractor, PassiveInteractor};

pub struct BaseInteractionPlugin;

impl Plugin for BaseInteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, show_lookups);
    }
}

pub fn detect_active_interaction(
    active: &Query<(&ActiveInteractor, &Transform)>,
    passive: (&PassiveInteractor, &Transform),
) -> bool {
    let (active_interactor, active_transform) = active
        .get_single()
        .expect("One active interactor is expected");

    let active_translation = active_transform.translation;
    let active_area = &active_interactor
        .area
        .to_box(active_translation.x, active_translation.y);

    let delta: f32 = 0.0000001;
    let (interactor, passive_transform) = passive;
    let translation = passive_transform.translation;
    let area = interactor.area.to_box(translation.x, translation.y);
    let intersection = active_area.intersection_with(&area);
    return active_translation.z - translation.z >= delta && intersection > 0;
}

fn show_lookups(
    keyboard: Res<Input<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform, &Description)>,
) {
    if !keyboard.pressed(KeyCode::F) {
        return;
    }
    for (inteactor, transform, description) in interactors.iter() {
        let is_interacting = detect_active_interaction(&active, (inteactor, transform));
        if is_interacting {
            println!("{:?}", description.text);
        }
    }
}

fn switch_object_one_off_state() {}
