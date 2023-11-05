use bevy::{
    prelude::{App, PluginGroup, Startup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use player::resources::MoveAnimationResource;
use ron::de::from_bytes;

mod core;
mod level;
mod player;
mod resources;
mod startup;

fn main() {
    App::new()
        .insert_resource(
            from_bytes::<MoveAnimationResource>(include_bytes!("../data/character_animations.ron"))
                .unwrap(),
        )
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Faces".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((player::systems::PlayerPlugin, level::house::HousePlugin))
        .add_systems(Startup, startup::setup)
        .add_state::<core::states::GameState>()
        .run();
}
