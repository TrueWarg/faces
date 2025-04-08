use bevy::prelude::AppExtStates;
use bevy::render::settings::{Backends, WgpuSettings};
use bevy::window::WindowMode;
use bevy::{
    prelude::{App, PluginGroup, Startup},
    window::{Window, WindowPlugin},
    DefaultPlugins,
};
use bevy::render::RenderPlugin;
use bevy_rapier2d::prelude::{NoUserData, RapierPhysicsPlugin};

use player::animations::PlayerAnimations;

use crate::core::states::GameState;
use crate::dev::DevSettingsPlugin;
use crate::dialog::{DialogPlugin, DialogScene};
use crate::fight::{FightPlugin, FightingScene};
use crate::gui::UiPlugin;
use crate::interaction::BaseInteractionPlugin;
use crate::level::LevelNavPlugin;
use crate::menu::MainMenuPlugin;
use crate::npc::NpcPlugin;
use crate::party::PartyPlugin;
use crate::player::plugins::PlayerPlugin;
use crate::rpg::CharacterScreenPlugin;
use crate::rpg::InventoryAndAbilityScreenPlugin;
use crate::rpg::RpgPlugin;
use crate::world_state::WorldStatePlugin;

mod animation;
mod core;
mod dev;
mod dialog;
mod fight;
mod gui;
mod interaction;
mod level;
mod menu;
mod movement;
mod npc;
mod party;
mod player;
mod rpg;
mod sound;
mod startup;
mod world_state;

fn main() {
    let wgpu_settings = WgpuSettings {
        backends: Some(Backends::VULKAN),
        ..Default::default()
    };
    App::new()
        .insert_resource(PlayerAnimations::default())
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugins(
            DefaultPlugins
                .set(
                    RenderPlugin {
                        render_creation: wgpu_settings.into(),
                        ..Default::default()
                    }
                )
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Faces".to_string(),
                        mode: WindowMode::BorderlessFullscreen,
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins((
            MainMenuPlugin,
            DevSettingsPlugin,
            PlayerPlugin,
            NpcPlugin,
            LevelNavPlugin,
            FightPlugin,
            PartyPlugin,
            RpgPlugin,
            FightingScene,
            UiPlugin,
            DialogPlugin,
            DialogScene,
            BaseInteractionPlugin,
            WorldStatePlugin,
            CharacterScreenPlugin,
        ))
        .add_plugins((InventoryAndAbilityScreenPlugin, sound::SoundPlugin))
        .add_systems(Startup, startup::setup)
        .init_state::<GameState>()
        .run();
}
