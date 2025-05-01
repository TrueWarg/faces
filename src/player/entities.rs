use bevy::asset::Handle;
use bevy::prelude::Component;
use bevy::sprite::TextureAtlasLayout;
use bevy::time::Timer;

use crate::animation::entities::MoveDirection;

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct FormidableDog;

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: Timer,
    pub direction: MoveDirection,
    // todo: remove it, find more conventional method to get particular sheets
    pub sheet_handle: Handle<TextureAtlasLayout>,
}

#[derive(Component, Debug)]
pub struct PlayerPosition {
    pub x: f32,
    pub y: f32,
}
