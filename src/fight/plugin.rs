use bevy::app::{App, Plugin, Startup};
use bevy::prelude::Commands;

use crate::fight::FightStorage;

pub struct FightPlugin;

impl Plugin for FightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, init_storages);
    }
}

fn init_storages(
    mut commands: Commands,
) {
    commands.insert_resource(FightStorage);
}
