use bevy::{
    prelude::{App, PluginGroup, Startup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use player::resources::PlayerAnimations;

mod core;
mod interaction;
mod level;
mod player;
mod resources;
mod startup;
mod movement;

fn main() {
    App::new()
        .insert_resource(PlayerAnimations::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Faces".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            player::systems::PlayerPlugin,
            level::house::HousePlugin,
            interaction::systems::BaseInteractionPlugin,
        ))
        .add_systems(Startup, startup::setup)
        .add_state::<core::states::GameState>()
        .run();
}
