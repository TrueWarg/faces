use bevy::{prelude::Component, time::Timer};

use crate::core::{
    geometry::BBox,
    state_machines::{CycleLinearTransition, FiniteLinearTransition},
};

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
pub struct LimitedInteractor;

#[derive(Component)]
pub struct Container {
    pub state: ContainerState,
}
#[derive(PartialEq)]
pub enum ContainerState {
    Closed,
    Full,
    Empty,
}

impl FiniteLinearTransition for ContainerState {
    fn transite(&self) -> Self {
        match self {
            ContainerState::Closed => ContainerState::Full,
            _ => ContainerState::Empty,
        }
    }

    fn initial_state() -> Self {
        ContainerState::Closed
    }

    fn final_state() -> Self {
        ContainerState::Empty
    }

    fn is_finished(&self) -> bool {
        *self == Self::final_state()
    }
}

#[derive(Component)]
pub struct Switcher {
    pub timer: Timer,
    pub state: SwitcherState,
}

pub enum SwitcherState {
    On,
    ToOff,
    Off,
    ToOn,
}

impl SwitcherState {
    pub fn is_in_transition(&self) -> bool {
        return match self {
            SwitcherState::ToOff => true,
            SwitcherState::ToOn => true,
            _ => false,
        };
    }
}

impl CycleLinearTransition for SwitcherState {
    fn transite(&self) -> Self {
        match self {
            SwitcherState::On => Self::ToOff,
            SwitcherState::Off => Self::ToOn,
            SwitcherState::ToOff => Self::Off,
            SwitcherState::ToOn => Self::On,
        }
    }

    fn initial_state() -> Self {
        Self::Off
    }
}

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
