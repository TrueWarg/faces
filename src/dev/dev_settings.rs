use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Update;
use bevy::asset::AssetServer;
use bevy::color::palettes::css::DIM_GREY;
use bevy::color::palettes::css::SILVER;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::AppExtStates;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Color;
use bevy::prelude::Font;
use bevy::prelude::Handle;
use bevy::prelude::in_state;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::KeyCode;
use bevy::prelude::NextState;
use bevy::prelude::ResMut;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::With;

use crate::core::states::GameState;
use crate::fight::{FightId, FightStorage};
use crate::gui::{Button, Root, Text};

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
    mut commands: Commands,
    mut fight_id_query: Query<(&mut FightId)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&SettingsId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<SettingsId>),
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

                match fight_id_query.get_single_mut() {
                    Ok(mut fight_id) => fight_id.0 = (*button_id).0,
                    Err(_) => {
                        commands.spawn(FightId((*button_id).0));
                    }
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

    let mut fights_button = Button::default();
    let mut levels_button = Button::default();


    root.spawn(&mut commands, DevSettingsScreen, |parent| {
        fights_button.spawn(parent, FIGHT_SAMPLES_BUTTON_ID, |parent| {
            let mut text = Text::medium("Fights samples", &font);
            text.set_color(Color::from(DIM_GREY));
            text.spawn(parent);
        });
        levels_button.spawn(parent, LEVEL_SAMPLES_BUTTON_ID, |parent| {
            let mut text = Text::medium("Level samples", &font);
            text.set_color(Color::from(DIM_GREY));
            text.spawn(parent);
        });
    });

    commands
        .spawn_empty()
        .insert(FontHandle { font });
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
    fight_storage: Res<FightStorage>,
    query: Query<&FontHandle>,
) {
    let font = &query.single().font;
    let mut root = Root::default();
    root.background_color(Color::from(DIM_GREY))
        .justify_around();

    root.spawn(&mut commands, FightsList, |parent| {
        for fight in fight_storage.get_all() {
            let mut button = Button::default();
            button.background_color(Color::from(SILVER));
            let text = format!("Fight {}", fight.id.0);
            button.spawn(parent, SettingsId(fight.id.0), |parent| {
                let mut text = Text::medium(text, &font);
                text.set_color(Color::from(DIM_GREY));
                text.spawn(parent);
            })
        }
    })
}

fn despawn_fignts_list(
    mut commands: Commands,
    query: Query<Entity, With<FightsList>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

#[derive(Component, Eq, PartialEq)]
struct SettingsId(pub usize);

const FIGHT_SAMPLES_BUTTON_ID: SettingsId = SettingsId(1);
const LEVEL_SAMPLES_BUTTON_ID: SettingsId = SettingsId(2);


/// <div style="background-color:rgb(90%, 90%, 90%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
