use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum EscapeFromHouse {
    #[default]
    Courier,
    GoSleep,
    CallDog,
    Escape,
    Completed,
}