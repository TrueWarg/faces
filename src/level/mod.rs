use crate::core::states::GameState;
use crate::level::court::CourtPlugin;
use crate::level::courthouse_front::CourtHouseFrontPlugin;
use crate::level::courthouse_hall::CourtHouseHallPlugin;
use crate::level::house::HousePlugin;
use crate::level::states::Level;
use crate::sound::Soundtrack;
use bevy::app::{App, Plugin, Update};
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::prelude::AppExtStates;
use bevy::prelude::Commands;
use bevy::prelude::DetectChanges;
use bevy::prelude::Entity;
use bevy::prelude::FromWorld;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::Resource;
use bevy::prelude::State;
use bevy_rapier2d::parry::simba::scalar::SupersetOf;
pub use dialogs::*;

mod court;
mod courthouse_front;
mod courthouse_hall;
mod dialogs;
pub mod house;
pub mod objects;
pub mod sprites;
pub(crate) mod states;

pub struct LevelNavPlugin;

impl Plugin for LevelNavPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HousePlugin {
                state: Level::House,
            },
            CourtHouseFrontPlugin {
                state: Level::CourtHouseFront,
            },
            CourtHouseHallPlugin {
                state: Level::CourtHouseHall,
            },
            CourtPlugin {
                state: Level::Court,
            },
        ))
        .init_state::<Level>()
        .init_resource::<CurrentSoundtrack>()
        .add_systems(Update, level_sounds_handle);
    }
}

fn level_sounds_handle(
    mut commands: Commands,
    soundtrack_res: Res<Soundtrack>,
    mut current_soundtrack_res: ResMut<CurrentSoundtrack>,
    game_state: Res<State<GameState>>,
    current_level_state: Res<State<Level>>,
) {
    if current_level_state.is_changed() || game_state.is_changed() {
        let sound = match game_state.get() {
            GameState::MainMenu | GameState::DevSetting => {
                (&soundtrack_res.menu, CurrentSoundtrackType::Menu)
            }
            GameState::Fighting => (&soundtrack_res.into_battle, CurrentSoundtrackType::Fighting),
            GameState::Dialog
            | GameState::Exploration
            | GameState::InventoryAndAbilities
            | GameState::Character => {
                let level = current_level_state.get();
                let sound = match level {
                    Level::None => &soundtrack_res.menu,
                    Level::House => &soundtrack_res.house,
                    Level::CourtHouseFront => &soundtrack_res.courthouse_front,
                    Level::CourtHouseHall => &soundtrack_res.courthouse,
                    Level::Court => &soundtrack_res.courthouse,
                };
                (sound, CurrentSoundtrackType::Exploration(level.clone()))
            }
            GameState::CatScene => (&soundtrack_res.menu, CurrentSoundtrackType::Menu),

            GameState::GameOver => (&soundtrack_res.menu, CurrentSoundtrackType::Menu),
        };

        if let Some(current) = &current_soundtrack_res.1 {
            if current == &sound.1 {
                return;
            }
        }

        if let Some(entity) = current_soundtrack_res.0 {
            commands.entity(entity).despawn();
        }

        let entity = commands
            .spawn(AudioBundle {
                source: sound.0.clone(),
                settings: PlaybackSettings::LOOP,
            })
            .id();

        current_soundtrack_res.0 = Some(entity);
        current_soundtrack_res.1 = Some(sound.1.clone());
    }
}

#[derive(PartialEq, Debug, Clone)]
enum CurrentSoundtrackType {
    Menu,
    Exploration(Level),
    Fighting,
}

#[derive(Resource, Default)]
struct CurrentSoundtrack(Option<Entity>, Option<CurrentSoundtrackType>);
