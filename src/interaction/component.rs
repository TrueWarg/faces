use bevy::prelude::Component;

use crate::core::geometry::BBox;

#[derive(Component)]
pub struct ActiveInteractor {
    pub area: InteractionArea,
    pub side: InteractionSide,
}

#[derive(Component)]
pub struct PassiveInteractor {
    pub area: InteractionArea,
    pub side: InteractionSide,
}

#[derive(Component)]
pub struct OneTimeInteractor;

#[derive(Debug)]
pub enum InteractionSide {
    Left,
    Top,
    Right,
    Bottom,
}

#[derive(Debug)]
pub struct InteractionArea {
    pub half_w: f32,
    pub half_h: f32,
    pub offset_x: f32,
    pub offset_y: f32,
}

impl InteractionArea {
    pub fn from_sizes(half_w: f32, half_h: f32) -> InteractionArea {
        return InteractionArea {
            half_w: half_w,
            half_h: half_h,
            offset_x: 0.0,
            offset_y: 0.0,
        };
    }

    pub fn create(half_w: f32, half_h: f32, offset_x: f32, offset_y: f32) -> InteractionArea {
        return InteractionArea {
            half_w: half_w,
            half_h: half_h,
            offset_x: offset_x,
            offset_y: offset_y,
        };
    }

    pub fn to_box(&self, x: f32, y: f32) -> BBox {
        return BBox {
            left: x - self.half_w + self.offset_x,
            top: y + self.half_h + self.offset_y,
            right: x + self.half_w + self.offset_x,
            bottom: y - self.half_h + self.offset_y,
        };
    }
}
