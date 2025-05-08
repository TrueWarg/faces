use bevy::math::Vec3;
use bevy::prelude::{Component, Timer};

#[derive(Component)]
pub struct MovingObject {
    pub start_position: Vec3,
    pub end_position: Vec3,
    pub duration: f32,
    pub timer: Timer,
}