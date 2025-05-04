use bevy::audio::AudioSource;
use bevy::prelude::{AssetServer, FromWorld, Handle, Resource, World};

#[derive(Resource)]
pub struct ButtonSounds {
    pub hover: Handle<AudioSource>,
    pub click: Handle<AudioSource>,
    pub iron_click: Handle<AudioSource>,
    pub final_click: Handle<AudioSource>,
    pub negative_click: Handle<AudioSource>,
}

impl FromWorld for ButtonSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ButtonSounds {
            hover: asset_server.load("sounds/ui/hover-btn.ogg"),
            click: asset_server.load("sounds/ui/click-btn.ogg"),
            iron_click: asset_server.load("sounds/ui/iron-click-btn.ogg"),
            final_click: asset_server.load("sounds/ui/final-click-btn.ogg"),
            negative_click: asset_server.load("sounds/ui/negative-btn.ogg"),
        }
    }
}

#[derive(Resource)]
pub struct Soundtrack {
    pub menu: Handle<AudioSource>,
    pub house: Handle<AudioSource>,
    pub courthouse_front: Handle<AudioSource>,
    pub courthouse: Handle<AudioSource>,
    pub into_battle: Handle<AudioSource>,
    pub super_speed: Handle<AudioSource>,
    pub warrior_routine: Handle<AudioSource>,
    pub cold_anger: Handle<AudioSource>,
}

impl FromWorld for Soundtrack {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Soundtrack {
            menu: asset_server.load("sounds/track/menu.ogg"),
            house: asset_server.load("sounds/track/house.ogg"),
            courthouse_front: asset_server.load("sounds/track/courthouse_front.ogg"),
            courthouse: asset_server.load("sounds/track/courthouse.ogg"),
            into_battle: asset_server.load("sounds/track/into_battle.ogg"),
            super_speed: asset_server.load("sounds/track/super_speed.ogg"),
            warrior_routine: asset_server.load("sounds/track/warrior_routine.ogg"),
            cold_anger: asset_server.load("sounds/track/cold_anger.ogg"),
        }
    }
}

#[derive(Resource)]
pub struct ChestSounds {
    pub opened: Handle<AudioSource>,
    pub items_picked: Handle<AudioSource>,
}

impl FromWorld for ChestSounds {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        ChestSounds {
            opened: asset_server.load("sounds/world/container_door.ogg"),
            items_picked: asset_server.load("sounds/world/pick_items.ogg"),
        }
    }
}
