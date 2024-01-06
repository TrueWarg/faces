use bevy::{prelude::Component, time::Timer, asset::Handle, sprite::TextureAtlas};

use crate::animation::entities::{MoveDirection, FightDirection};

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
    pub sheet_handle: Handle<TextureAtlas>,
}

#[derive(Component)]
pub struct FightAnimation {
    pub timer: Timer,
    pub direction: FightDirection,
    // todo: remove it, find more convential method to get particular sheets
    pub sheet_handle: Handle<TextureAtlas>,
}
