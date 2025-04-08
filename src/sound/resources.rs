use bevy::audio::AudioSource;
use bevy::prelude::{AssetServer, FromWorld, Handle, Resource, World};

#[derive(Resource)]
pub struct ButtonSounds {
    pub hover: Handle<AudioSource>,
}

impl FromWorld for ButtonSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ButtonSounds {
            hover: asset_server.load("sounds/ui/ball-tap.ogg"),
        }
    }
}
