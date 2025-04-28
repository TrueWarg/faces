use bevy::ecs::component::Component;

use crate::core::geometry::{BBox, Point2D};

#[derive(Component)]
pub struct MoveAgent {
    pub priority: i32,
    pub half_size: i32,
    pub route: Vec<Point2D>,
}

#[derive(Component)]
pub struct Blocks {
    pub blocks: Vec<BBox>,
}

impl Blocks {
    pub fn from(items: Vec<BBox>) -> Self {
        Blocks { blocks: items }
    }
}

#[derive(Component)]
pub struct Target {
    pub half_size: i32,
}

#[derive(Component)]
pub struct MoveArea {
    pub half_width: i32,
    pub half_height: i32,
    pub center_position: Point2D,
}

pub enum MoveDirection {
    Top,
    LeftTop,
    Left,
    LeftBottom,
    Bottom,
    RightBottom,
    Right,
    RightTop,
    TopIdle,
    LeftTopIdle,
    LeftIdle,
    LeftBottomIdle,
    BottomIdle,
    RightBottomIdle,
    RightIdle,
    RightTopIdle,
}
