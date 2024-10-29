use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::prelude::{TextureAtlasLayout, Timer};
use crate::animation::entities::MoveDirection;

#[derive(Component)]
pub struct Npc {
    pub speed: f32,
}

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: Timer,
    pub direction: MoveDirection,
    // todo: remove it, find more convential method to get particular sheets
    pub sheet_handle: Handle<TextureAtlasLayout>,
}
