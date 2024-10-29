use bevy::app::{App, Plugin};
use bevy::prelude::AppExtStates;

use crate::world_state::quests::EscapeFromHouse;

pub struct WorldStatePlugin;

impl Plugin for WorldStatePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<EscapeFromHouse>();
    }
}


