use crate::rpg::storages::CharacterStorage;
use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;

pub struct RpgPlugin;

impl Plugin for RpgPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_storages);
    }
}

fn init_storages(mut commands: Commands) {
    commands.insert_resource(CharacterStorage::default());
}
