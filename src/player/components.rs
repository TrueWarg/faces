use bevy::{asset::Handle, prelude::Component, sprite::TextureAtlasLayout, time::Timer};

use crate::animation::entities::{FightDirection, MoveDirection};

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
    pub is_fights: bool,
}

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: Timer,
    pub direction: MoveDirection,
    // todo: remove it, find more convential method to get particular sheets
    pub sheet_handle: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct FightAnimation {
    pub timer: Timer,
    pub direction: FightDirection,
    // todo: remove it, find more convential method to get particular sheets
    pub sheet_handle: Handle<TextureAtlasLayout>,
}
