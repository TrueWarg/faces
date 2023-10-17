use bevy::{
    prelude::{App, PluginGroup, Startup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};

mod states;
mod startup;
mod resources;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Faces".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<states::GameState>()
        .add_systems(Startup, startup::setup)
        .run();
}
