use bevy::ecs::component::Component;

use crate::movement::component::MoveDirection;

#[derive(Component)]
pub struct Npc {
    pub speed: f32,
    pub move_direction: MoveDirection,
}
