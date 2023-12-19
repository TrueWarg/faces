use bevy::ecs::component::Component;

use crate::core::geometry::{Point2D, Rectangle, Square};

#[derive(Component)]
pub struct MoveAgent {
    pub priority: i32,
    pub square: Square,
    pub route: Vec<Point2D>,
}

#[derive(Component)]
pub struct Blocks {
    pub blocks: Vec<Rectangle>,
}

impl Blocks {
    pub fn from(items: Vec<Rectangle>) -> Self {
        return Blocks { blocks: items };
    }
}

#[derive(Component)]
pub struct Target {
    pub square: Square,
}

#[derive(Component)]
pub struct MoveArea {
    pub half_width: i32,
    pub half_height: i32,
    pub center_position: Point2D,
}
