use bevy::ecs::component::Component;

use crate::movement::entities::MoveDirection;

#[derive(Component)]
pub struct Npc {
    pub speed: f32,
    pub move_direction: MoveDirection,
}
