use bevy::{prelude::Component, time::Timer};
use bevy::input::ButtonInput;
use bevy::prelude::{Commands, Entity, KeyCode, Query, Res, Time, Transform, With};

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
    fn transit(&self) -> Self {
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
    fn transit(&self) -> Self {
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
            half_w,
            half_h,
            offset_x: 0.0,
            offset_y: 0.0,
        };
    }

    pub fn create(half_w: f32, half_h: f32, offset_x: f32, offset_y: f32) -> InteractionArea {
        return InteractionArea {
            half_w,
            half_h,
            offset_x,
            offset_y,
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

pub fn detect_active_interaction(
    active: &Query<(&ActiveInteractor, &Transform)>,
    passive: (&PassiveInteractor, &Transform),
) -> bool {
    let (active_interactor, active_transform) = active
        .get_single()
        .expect("One active interactor is expected");

    let active_translation = active_transform.translation;
    let active_area = &active_interactor
        .area
        .to_box(active_translation.x, active_translation.y);

    let delta: f32 = 0.0000001;
    let (interactor, passive_transform) = passive;
    let translation = passive_transform.translation;
    let area = interactor.area.to_box(translation.x, translation.y);
    let intersection = active_area.round_intersection_with(&area);
    return active_translation.z - translation.z >= delta && intersection > 0;
}

pub fn transit_to_next_container_state(
    mut commands: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    mut interactors: Query<
        (Entity, &PassiveInteractor, &Transform, &mut Container),
        With<LimitedInteractor>,
    >,
) {
    if !(keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE)) {
        return;
    }
    for (entity, interactor, transform, mut container) in interactors.iter_mut() {
        let is_interacting = detect_active_interaction(&active, (interactor, transform));
        if is_interacting {
            container.state = container.state.transit();
            if container.state.is_finished() {
                commands.entity(entity).remove::<LimitedInteractor>();
            }
        }
    }
}

pub fn change_switcher_state(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    active: Query<(&ActiveInteractor, &Transform)>,
    mut interactors: Query<(&PassiveInteractor, &Transform, &mut Switcher)>,
) {
    for (interactor, transform, mut switcher) in interactors.iter_mut() {
        if switcher.state.is_in_transition() {
            switcher.timer.tick(time.delta());
            if switcher.timer.finished() {
                switcher.state = switcher.state.transit();
            }
        } else {
            let is_pressed = keyboard.pressed(KeyCode::KeyE) && keyboard.just_pressed(KeyCode::KeyE);
            if is_pressed && detect_active_interaction(&active, (interactor, transform)) {
                switcher.timer.reset();
                switcher.state = switcher.state.transit();
            }
        }
    }
}
