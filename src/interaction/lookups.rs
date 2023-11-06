use bevy::prelude::{Input, KeyCode, Plugin, Query, Res, Transform, Update};

use crate::core::components::Description;

use super::component::{ActiveInteractor, PassiveInteractor};
pub struct LookupsPlugin;

impl Plugin for LookupsPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Update, detect_and_show_lookups);
    }
}

fn detect_and_show_lookups(
    keyboard: Res<Input<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    interactors: Query<(&PassiveInteractor, &Transform, &Description)>,
) {
    let (active_interactor, active_transform) = active
        .get_single()
        .expect("One active interactor is expected");

    let active_translation = active_transform.translation;
    let active_area = &active_interactor
        .area
        .to_box(active_translation.x, active_translation.y);
    let delta: f32 = 0.0000001;
    for (inteactor, transform, description) in interactors.iter() {
        let translation = transform.translation;
        let area = &inteactor.area.to_box(translation.x, translation.y);
        let intersection = active_area.intersection_with(area);
        if keyboard.pressed(KeyCode::F)
            && active_translation.z - translation.z >= delta
            && intersection > 0
        {
            println!("{:?}", description.text);
        }
    }
}
