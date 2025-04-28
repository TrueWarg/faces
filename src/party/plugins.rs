use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;

use crate::party::storages::PartyStateStorage;

pub struct PartyPlugin;

impl Plugin for PartyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_storages);
    }
}

fn init_storages(
    mut commands: Commands,
) {
    commands.insert_resource(PartyStateStorage::default());
}
