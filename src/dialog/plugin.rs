use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;
use crate::dialog::{DialogsStorage, SelectedVariantsSource};

pub struct DialogPlugin;

impl Plugin for DialogPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_storages);
    }
}

fn init_storages(
    mut commands: Commands,
) {
    commands.insert_resource(DialogsStorage);
    commands.insert_resource(SelectedVariantsSource::default());
}
