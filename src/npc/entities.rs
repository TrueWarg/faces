use crate::animation::entities::MoveDirection;
use crate::movement;
use bevy::asset::Handle;
use bevy::ecs::component::Component;
use bevy::prelude::{TextureAtlasLayout, Timer};

#[derive(Component)]
pub struct Npc {
    pub speed: f32,
    pub move_direction: movement::entities::MoveDirection,
}

#[derive(Component)]
pub struct MoveAnimation {
    pub timer: Timer,
    pub direction: MoveDirection,
    // todo: remove it, find more conventional method to get particular sheets
    pub sheet_handle: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct IdleAnimation {
    pub timer: Timer,
    pub frames_count: usize,
}
