use bevy::app::{App, Plugin, Update};
use bevy::asset::AssetServer;
use bevy::color::palettes::css::DIM_GREY;
use bevy::color::palettes::css::SILVER;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::{AppExtStates, BackgroundColor, Changed, Color, Font, Handle, in_state, Interaction, IntoSystemConfigs, KeyCode, NextState, ResMut, State, States};
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::With;

use crate::core::states::GameState;
use crate::gui::{Button, ButtonId, Root};

pub struct DevSettingsPlugin;

#[derive(Component)]
struct DevSettingsScreen;

#[derive(Component)]
struct FightsList;

#[derive(Component)]
struct LevelsList;

#[derive(Component)]
struct FontHandle {
    font: Handle<Font>,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ScreenState {
    #[default]
    Main,
    FightsList,
    LevelsList,
}

impl Plugin for DevSettingsPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ScreenState>()
            .add_systems(OnEnter(GameState::DevSetting), spawn_main)
            .add_systems(OnExit(GameState::DevSetting), despawn_main)
            .add_systems(OnEnter(ScreenState::FightsList), spawn_fights_list)
            .add_systems(OnExit(ScreenState::FightsList), despawn_fignts_list)
            .add_systems(Update, (keyboard_input_handle, mouse_input_handle)
                .run_if(in_state(GameState::DevSetting)),
            );
    }
}

fn mouse_input_handle(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&ButtonId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonId>),
    >,
) {
    for (button_id, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = SILVER.into();
            }
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                if *button_id == FIGHT_SAMPLES_BUTTON_ID {
                    next_state.set(ScreenState::FightsList);
                    return;
                }

                if *button_id == LEVEL_SAMPLES_BUTTON_ID {
                    return;
                }

                next_game_state.set(GameState::Fighting)
            }
        }
    }
}

fn keyboard_input_handle(
    current_state: Res<State<ScreenState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if *current_state.get() == ScreenState::Main {
        return;
    }

    if keyboard.pressed(KeyCode::Escape) && keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(ScreenState::Main);
    }
}

fn spawn_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/quattrocentoSans-Bold.ttf");
    let mut root = Root::default();
    root.background_color(Color::from(DIM_GREY))
        .justify_around();

    let mut fights_button = Button::new("Fights samples", &font);
    fights_button.id(FIGHT_SAMPLES_BUTTON_ID)
        .text_color(Color::from(DIM_GREY));

    let mut levels_button = Button::new("Level samples", &font);
    levels_button.id(LEVEL_SAMPLES_BUTTON_ID)
        .text_color(Color::from(DIM_GREY));

    commands
        .spawn_empty()
        .insert(FontHandle { font });

    root.spawn(&mut commands, DevSettingsScreen, |parent| {
        fights_button.spawn(parent);
        levels_button.spawn(parent);
    })
}

fn despawn_main(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut commands: Commands,
    query: Query<Entity, With<DevSettingsScreen>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
    next_state.set(ScreenState::Main);
}

fn spawn_fights_list(
    mut commands: Commands,
    query: Query<&FontHandle>,
) {
    let font = &query.single().font;
    let mut root = Root::default();
    root.background_color(Color::from(DIM_GREY))
        .justify_around();

    let mut fight_1 = Button::new("Fight_1", font);
    fight_1
        .id(ButtonId { value: 333 })
        .background_color(Color::from(SILVER))
        .text_color(Color::from(DIM_GREY));

    let mut fight_2 = Button::new("Fight_2", font);
    fight_2
        .background_color(Color::from(SILVER))
        .text_color(Color::from(DIM_GREY));

    let mut fight_3 = Button::new("Fight_3", font);
    fight_3
        .background_color(Color::from(SILVER))
        .text_color(Color::from(DIM_GREY));

    let mut fight_4 = Button::new("Fight_4", font);
    fight_4
        .background_color(Color::from(SILVER))
        .text_color(Color::from(DIM_GREY));

    root.spawn(&mut commands, FightsList, |parent| {
        fight_1.spawn(parent);
        fight_2.spawn(parent);
        fight_3.spawn(parent);
        fight_4.spawn(parent);
    })
}

fn despawn_fignts_list(
    mut commands: Commands,
    query: Query<Entity, With<FightsList>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

const FIGHT_SAMPLES_BUTTON_ID: ButtonId = ButtonId { value: 1 };
const LEVEL_SAMPLES_BUTTON_ID: ButtonId = ButtonId { value: 2 };
/// <div style="background-color:rgb(90%, 90%, 90%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
