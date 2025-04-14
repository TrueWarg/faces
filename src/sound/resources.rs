use bevy::audio::AudioSource;
use bevy::prelude::{AssetServer, FromWorld, Handle, Resource, World};

#[derive(Resource)]
pub struct ButtonSounds {
    pub hover: Handle<AudioSource>,
    pub click: Handle<AudioSource>,
    pub iron_click: Handle<AudioSource>,
    pub final_click: Handle<AudioSource>,
}

impl FromWorld for ButtonSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ButtonSounds {
            hover: asset_server.load("sounds/ui/hover-btn.ogg"),
            click: asset_server.load("sounds/ui/click-btn.ogg"),
            iron_click: asset_server.load("sounds/ui/iron-click-btn.ogg"),
            final_click: asset_server.load("sounds/ui/final-click-btn.ogg"),
        }
    }
}
