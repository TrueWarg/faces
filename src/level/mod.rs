use bevy::app::{App, Plugin, Update};
use bevy::prelude::{AppExtStates, DetectChanges, NextState, Res, ResMut, State};

use crate::core::states::GameState;
use crate::level::house::HousePlugin;
use crate::level::states::Level;

pub mod house;
pub mod objects;
pub mod sprites;
pub(crate) mod states;
mod dialogs;
mod courthouse_front;

pub use dialogs::*;
use crate::level::courthouse_front::CourtHouseFrontPlugin;

pub struct LevelNavPlugin;

impl Plugin for LevelNavPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                HousePlugin { state: Level::House },
                CourtHouseFrontPlugin { state: Level::CourtHouseFront }
            )
            )
            .init_state::<Level>();
    }
}