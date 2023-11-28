use bevy::{prelude::Component, time::Timer};

use super::types::MoveDirection;

#[derive(Component, Debug)]
pub struct Player {
    pub speed: f32,
}

#[derive(Component)]
pub struct MoveAnimationComponent {
    pub timer: Timer,
    pub direction: MoveDirection,
}
