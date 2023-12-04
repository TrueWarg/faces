use bevy::ecs::component::Component;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point2D {
    pub x: i32,
    pub y: i32,
}

impl Point2D {
    pub fn new(x: i32, y: i32) -> Point2D {
        return Point2D { x, y };
    }
}

#[derive(Component)]
pub struct MoveAgent {
    pub priority: i32,
    pub square: Square,
    pub route: Vec<Point2D>,
}

#[derive(Component)]
pub struct Blocks {
    pub blocks: Vec<Block>,
}

impl Blocks {
    pub fn from(items: Vec<Block>) -> Self {
        return Blocks { blocks: items };
    }
}

pub struct Block {
    pub half_width: i32,
    pub half_height: i32,
    pub center_position: Point2D,
}

pub struct Square {
    pub half_size: i32,
    pub center_position: Point2D,
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
