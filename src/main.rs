use bevy::{
    prelude::{App, PluginGroup, Startup, Update},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{RapierPhysicsPlugin, NoUserData};

mod states;
mod startup;
mod resources;
mod playground;

fn main() {
    App::new()
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Faces".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<states::GameState>()
        .add_systems(Startup, (playground::setup, playground::spawn_player))
        .add_systems(Update, playground::player_movement)
        .run();
}
