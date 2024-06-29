use bevy::app::{App, Plugin};
use bevy::prelude::{NextState, ResMut, Startup};
use crate::level::house::HousePlugin;
use crate::level::states::Level;
use crate::level::test_level::TestLevel;

pub mod house;
pub mod component;
pub mod resources;
pub(crate) mod states;
mod test_level;

pub struct LevelNavPlugin;

impl Plugin for LevelNavPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, load_level)
            .add_plugins((
                HousePlugin { state: Level::House },
                TestLevel { state: Level::Test },
            )
            )
            .init_state::<Level>();
    }
}

fn load_level(
    mut next_state: ResMut<NextState<Level>>,
) {
    next_state.set(Level::Test);
}