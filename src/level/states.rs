use bevy::prelude::States;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Level {
    #[default]
    None,
    House,
    CourtHouseFront,
    CourtHouseHall,
    Court,
}
