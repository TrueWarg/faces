use bevy::{
    DefaultPlugins,
    prelude::{App, PluginGroup, Startup},
    window::{Window, WindowPlugin},
};
use bevy::prelude::AppExtStates;
use bevy::window::WindowMode;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

use player::animations::PlayerAnimations;

use crate::core::states::GameState;
use crate::dev::DevSettingsPlugin;
use crate::dialog::{DialogPlugin, DialogScene};
use crate::fight::{FightingScene, FightPlugin};
use crate::gui::UiPlugin;
use crate::interaction::BaseInteractionPlugin;
use crate::level::LevelNavPlugin;
use crate::menu::MainMenuPlugin;
use crate::party::PartyPlugin;
use crate::player::plugins::PlayerPlugin;

mod core;
mod interaction;
mod level;
mod player;
mod startup;
mod movement;
mod npc;
mod animation;
mod menu;
mod fight;
mod gui;
mod dev;
mod rpg;
mod party;
mod dialog;

fn main() {
    App::new()
        .insert_resource(PlayerAnimations::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Faces".to_string(),
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins((
            MainMenuPlugin,
            DevSettingsPlugin,
            PlayerPlugin,
            LevelNavPlugin,
            FightPlugin,
            PartyPlugin,
            FightingScene,
            UiPlugin,
            DialogPlugin,
            DialogScene,
            BaseInteractionPlugin,
        ))
        .add_systems(Startup, startup::setup)
        .init_state::<GameState>()
        .run();
}
