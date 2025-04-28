use bevy::app::App;
use bevy::app::Plugin;
use bevy::app::Update;
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::color::palettes::css::DIM_GREY;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::in_state;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::KeyCode;
use bevy::prelude::NextState;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::With;
use bevy::prelude::{AppExtStates, JustifyContent, Val};
use bevy::ui::{AlignItems, FlexDirection};
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetFlexDirectionExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::ui_builder::{UiBuilderExt, UiRoot};

use crate::core::states::GameState;
use crate::dialog::{DialogId, DialogsStorage};
use crate::fight::{FightId, FightStorage};
use crate::gui::{TextButton, TextButtonExt};
use crate::level::states::Level;
use crate::sound::ButtonSounds;

pub struct DevSettingsPlugin;

#[derive(Component)]
struct DevSettingsScreen;

#[derive(Component)]
struct FightsList;

#[derive(Component)]
struct DialogsList;

#[derive(Component)]
struct LevelsList;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ScreenState {
    #[default]
    Main,
    FightsList,
    DialogsList,
    LevelsList,
}

impl Plugin for DevSettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ScreenState>()
            .add_systems(OnEnter(GameState::DevSetting), spawn_main)
            .add_systems(OnExit(GameState::DevSetting), despawn_main)
            .add_systems(OnEnter(ScreenState::FightsList), spawn_fights_list)
            .add_systems(OnExit(ScreenState::FightsList), despawn_fignts_list)
            .add_systems(OnEnter(ScreenState::DialogsList), spawn_dialogs_list)
            .add_systems(OnExit(ScreenState::DialogsList), despawn_dialogs_list)
            .add_systems(OnEnter(ScreenState::LevelsList), spawn_levels_list)
            .add_systems(OnExit(ScreenState::LevelsList), despawn_levels_list)
            .add_systems(
                Update,
                (keyboard_input_handle, mouse_input_handle).run_if(in_state(GameState::DevSetting)),
            );
    }
}

fn mouse_input_handle(
    mut commands: Commands,
    mut fight_id_query: Query<(&mut FightId)>,
    mut dialog_id_query: Query<(&mut DialogId)>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_level_state: ResMut<NextState<Level>>,
    current_screen_state: Res<State<ScreenState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&TextButton<SettingsId>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    audio_res: Res<ButtonSounds>,
) {
    for (button, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = button.config.idle;
            }
            Interaction::Hovered => {
                *background_color = button.config.hover;
            }
            Interaction::Pressed => {
                let default = AudioBundle {
                    source: audio_res.click.clone(),
                    settings: PlaybackSettings::ONCE,
                };
                match current_screen_state.get() {
                    ScreenState::Main => {
                        if button.payload == FIGHT_SAMPLES_BUTTON_ID {
                            next_state.set(ScreenState::FightsList);
                            return;
                        }

                        if button.payload == DIALOGS_SAMPLES_BUTTON_ID {
                            next_state.set(ScreenState::DialogsList);
                            return;
                        }

                        if button.payload == LEVEL_SAMPLES_BUTTON_ID {
                            next_state.set(ScreenState::LevelsList);
                            return;
                        }

                        if button.payload == CHARACTER_SAMPLE_BUTTON_ID {
                            next_game_state.set(GameState::Character);
                            return;
                        }

                        if button.payload == INVENTORY_AND_ABILITIES_SAMPLE_BUTTON_ID {
                            next_game_state.set(GameState::InventoryAndAbilities);
                            return;
                        }
                    }
                    ScreenState::FightsList => {
                        match fight_id_query.get_single_mut() {
                            Ok(mut fight_id) => fight_id.0 = button.payload.0,
                            Err(_) => {
                                commands.spawn(FightId(button.payload.0));
                            }
                        }
                        next_game_state.set(GameState::Fighting);
                        return;
                    }
                    ScreenState::DialogsList => {
                        match dialog_id_query.get_single_mut() {
                            Ok(mut dialog_id) => dialog_id.0 = button.payload.0,
                            Err(_) => {
                                commands.spawn(DialogId(button.payload.0));
                            }
                        }
                        next_game_state.set(GameState::Dialog);
                        return;
                    }
                    ScreenState::LevelsList => {
                        if button.payload == HOUSE_LEVEL_BUTTON_ID {
                            next_level_state.set(Level::House);
                        }
                        if button.payload == COURTHOUSE_FRONT_LEVEL_ID {
                            next_level_state.set(Level::CourtHouseFront);
                        }
                        if button.payload == COURTHOUSE_HALL_LEVEL_ID {
                            next_level_state.set(Level::CourtHouseHall);
                        }
                        if button.payload == COURTHOUSE_COURT_LEVEL_ID {
                            next_level_state.set(Level::Court);
                        }
                        next_game_state.set(GameState::Exploration);
                        return;
                    }
                }
                commands.spawn(default);
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

fn spawn_main(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.text_button("Fights samples", FIGHT_SAMPLES_BUTTON_ID);

            parent.text_button("Dialogs samples", DIALOGS_SAMPLES_BUTTON_ID);

            parent.text_button("Level samples", LEVEL_SAMPLES_BUTTON_ID);

            parent.text_button("Character screen", CHARACTER_SAMPLE_BUTTON_ID);

            parent.text_button(
                "Inv and Abi screen",
                INVENTORY_AND_ABILITIES_SAMPLE_BUTTON_ID,
            );
        })
        .insert(DevSettingsScreen)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
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

fn spawn_fights_list(mut commands: Commands, fight_storage: Res<FightStorage>) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            for fight in fight_storage.get_all() {
                let text = format!("Fight {}", fight.id.0);
                parent.text_button(text, SettingsId(fight.id.0));
            }
        })
        .insert(FightsList)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
}

fn despawn_fignts_list(mut commands: Commands, query: Query<Entity, With<FightsList>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn spawn_dialogs_list(mut commands: Commands, dialogs_storage: Res<DialogsStorage>) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            for (id, dialog) in dialogs_storage.get_all() {
                let text = match dialog.label {
                    None => {
                        format!("Dialog with id {}", id)
                    }
                    Some(value) => value,
                };
                parent.text_button(text, SettingsId(id));
            }
        })
        .insert(DialogsList)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
}

fn despawn_dialogs_list(mut commands: Commands, query: Query<Entity, With<DialogsList>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn spawn_levels_list(mut commands: Commands) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.text_button("House", HOUSE_LEVEL_BUTTON_ID);
            parent.text_button("Courthouse front", COURTHOUSE_FRONT_LEVEL_ID);
            parent.text_button("Courthouse hall", COURTHOUSE_HALL_LEVEL_ID);
            parent.text_button("Court", COURTHOUSE_COURT_LEVEL_ID);
        })
        .insert(LevelsList)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .flex_direction(FlexDirection::Column)
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
}

fn despawn_levels_list(mut commands: Commands, query: Query<Entity, With<LevelsList>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

#[derive(Component, Eq, PartialEq)]
struct SettingsId(pub usize);

const FIGHT_SAMPLES_BUTTON_ID: SettingsId = SettingsId(1);
const DIALOGS_SAMPLES_BUTTON_ID: SettingsId = SettingsId(2);
const LEVEL_SAMPLES_BUTTON_ID: SettingsId = SettingsId(3);
const CHARACTER_SAMPLE_BUTTON_ID: SettingsId = SettingsId(4);
const INVENTORY_AND_ABILITIES_SAMPLE_BUTTON_ID: SettingsId = SettingsId(5);
const HOUSE_LEVEL_BUTTON_ID: SettingsId = SettingsId(11);
const COURTHOUSE_FRONT_LEVEL_ID: SettingsId = SettingsId(12);
const COURTHOUSE_HALL_LEVEL_ID: SettingsId = SettingsId(13);
const COURTHOUSE_COURT_LEVEL_ID: SettingsId = SettingsId(14);
