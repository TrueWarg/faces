use bevy::{
    prelude::{App, PluginGroup, Startup, Update, IntoSystemConfigs},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};
use playground::CharacterAnimationResource;
use ron::de::from_bytes;

mod playground;
mod resources;
mod startup;
mod states;

fn main() {
    App::new()
        .insert_resource(
            from_bytes::<CharacterAnimationResource>(include_bytes!(
                "../data/character_animations.ron"
            ))
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
        .add_state::<states::GameState>()
        .add_systems(
            Startup,
            (
                playground::setup,
                playground::spawn_player,
                playground::spawn_nps,
            ),
        )
        .add_systems(Update, playground::player_movement)
        .add_systems(Update, playground::set_player_animation_system.after(playground::player_movement))
        .add_systems(Update, playground::animate_character_system.after(playground::set_player_animation_system))
        .run();
}
