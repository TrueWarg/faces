use bevy::app::{Plugin, Update};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Query, Res, Transform};

use crate::core::entities::Description;
use crate::interaction::interactors::change_switcher_state;
use crate::interaction::interactors::detect_active_interaction;
use crate::interaction::interactors::transit_to_next_container_state;
use crate::interaction::interactors::ActiveInteractor;
use crate::interaction::interactors::PassiveInteractor;

pub mod interactors;

pub struct BaseInteractionPlugin;

impl Plugin for BaseInteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                show_lookups,
                transit_to_next_container_state,
                change_switcher_state,
            ),
        );
    }
}

fn show_lookups(
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform, &Description)>,
) {
    if !(keyboard.pressed(KeyCode::KeyF) && keyboard.just_pressed(KeyCode::KeyF)) {
        return;
    }
    for (inteactor, transform, description) in interactors.iter() {
        let is_interacting = detect_active_interaction(&active, (inteactor, transform));
        if is_interacting {
            println!("{:?}", description.text);
        }
    }
}
