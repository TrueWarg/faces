use bevy::app::{App, Plugin};
use bevy::prelude::{AppExtStates, DetectChanges};

pub use dialogs::*;
use crate::level::court::CourtPlugin;

use crate::level::courthouse_front::CourtHouseFrontPlugin;
use crate::level::courthouse_hall::CourtHouseHallPlugin;
use crate::level::house::HousePlugin;
use crate::level::states::Level;

pub mod house;
pub mod objects;
pub mod sprites;
pub(crate) mod states;
mod dialogs;
mod courthouse_front;
mod courthouse_hall;
mod court;

pub struct LevelNavPlugin;

impl Plugin for LevelNavPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HousePlugin { state: Level::House },
                CourtHouseFrontPlugin { state: Level::CourtHouseFront },
                CourtHouseHallPlugin { state: Level::CourtHouseHall },
                CourtPlugin { state: Level::Court },
            )
            )
            .init_state::<Level>();
    }
}