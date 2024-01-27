use bevy::{
    prelude::{Commands, Entity, Input, KeyCode, Plugin, Query, Res, Transform, Update, With},
    time::Time,
};

use crate::core::{
    components::Description,
    state_machines::{CycleLinearTransition, FiniteLinearTransition},
};

use super::component::{
    ActiveInteractor, Container, LimitedInteractor, PassiveInteractor, Switcher,
};

pub struct BaseInteractionPlugin;

impl Plugin for BaseInteractionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(
            Update,
            (
                show_lookups,
                transite_to_next_container_state,
                change_switcher_state,
            ),
        );
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
    let intersection = active_area.round_intersection_with(&area);
    return active_translation.z - translation.z >= delta && intersection > 0;
}

fn show_lookups(
    keyboard: Res<Input<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform, &Description)>,
) {
    if !(keyboard.pressed(KeyCode::F) && keyboard.just_pressed(KeyCode::F)) {
        return;
    }
    for (inteactor, transform, description) in interactors.iter() {
        let is_interacting = detect_active_interaction(&active, (inteactor, transform));
        if is_interacting {
            println!("{:?}", description.text);
        }
    }
}

pub fn transite_to_next_container_state(
    mut commands: Commands,
    keyboard: Res<Input<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    mut interactors: Query<
        (Entity, &PassiveInteractor, &Transform, &mut Container),
        With<LimitedInteractor>,
    >,
) {
    if !(keyboard.pressed(KeyCode::E) && keyboard.just_pressed(KeyCode::E)) {
        return;
    }
    for (entity, inteactor, transform, mut container) in interactors.iter_mut() {
        let is_interacting = detect_active_interaction(&active, (inteactor, transform));
        if is_interacting {
            container.state = container.state.transite();
            if container.state.is_finished() {
                commands.entity(entity).remove::<LimitedInteractor>();
            }
        }
    }
}

pub fn change_switcher_state(
    time: Res<Time>,
    keyboard: Res<Input<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    mut interactors: Query<(&PassiveInteractor, &Transform, &mut Switcher)>,
) {
    for (inteactor, transform, mut switcher) in interactors.iter_mut() {
        if switcher.state.is_in_transition() {
            switcher.timer.tick(time.delta());
            if switcher.timer.finished() {
                switcher.state = switcher.state.transite();
            }
        } else {
            let is_pressed = keyboard.pressed(KeyCode::E) && keyboard.just_pressed(KeyCode::E);
            if is_pressed && detect_active_interaction(&active, (inteactor, transform)){
                switcher.timer.reset();
                switcher.state = switcher.state.transite();
            }
        }
    }
}
