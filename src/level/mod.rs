use bevy::app::{App, Plugin, Update};
use bevy::prelude::{AppExtStates, DetectChanges, NextState, Res, ResMut, State};

use crate::core::states::GameState;
use crate::level::house::HousePlugin;
use crate::level::states::Level;

pub mod house;
pub mod component;
pub mod resources;
pub(crate) mod states;

pub struct LevelNavPlugin;

impl Plugin for LevelNavPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, load_level)
            .add_plugins((
                HousePlugin { state: Level::House },
            )
            )
            .init_state::<Level>();
    }
}

fn load_level(
    game_state: Res<State<GameState>>,
    mut next_state: ResMut<NextState<Level>>,
) {
    if game_state.get() == &GameState::Exporation {
        if game_state.is_changed() {
            next_state.set(Level::House);
        }
    }
}