use bevy::app::{App, AppExit, Plugin, Update};
use bevy::color::Color;
use bevy::color::palettes::css::DIM_GREY;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::AlignItems;
use bevy::prelude::AppExtStates;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Entity;
use bevy::prelude::EventWriter;
use bevy::prelude::Interaction;
use bevy::prelude::NextState;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::With;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::in_state;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyContent;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Val;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::UiBuilderExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiRoot;

use crate::core::states::GameState;
use crate::gui::{TextButton, TextButtonExt};

pub struct MainMenuPlugin;

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
struct Options;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ScreenState {
    #[default]
    Main,
    Options,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<ScreenState>()
            .add_systems(OnEnter(GameState::MainMenu), spawn_main)
            .add_systems(OnExit(GameState::MainMenu), despawn_main)
            .add_systems(OnEnter(ScreenState::Options), spawn_options)
            .add_systems(OnExit(ScreenState::Options), despawn_options)
            .add_systems(Update, mouse_input_handle.run_if(in_state(GameState::MainMenu)));
    }
}

#[derive(Component, Eq, PartialEq)]
struct MenuItemId(usize);

fn spawn_main(
    mut commands: Commands,
) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent
                .text_button("Continue", CONTINUE_MENU_ITEM_ID);
            parent
                .text_button("New", NEW_MENU_ITEM_ID);
            parent
                .text_button("Options", OPTIONS_MENU_ITEM_ID);
            parent
                .text_button("Exit", EXIT_MENU_ITEM_ID);
        })
        .insert(MainMenu)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
}

fn despawn_main(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut commands: Commands,
    query: Query<Entity, With<MainMenu>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
    next_state.set(ScreenState::Main);
}

fn spawn_options(
    mut commands: Commands,
) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent
                .text_button("Dev settings", DEV_SETTINGS_OPTION_ITEM_ID);
        })
        .insert(Options)
        .style()
        .justify_content(JustifyContent::SpaceAround)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center)
        .background_color(Color::from(DIM_GREY));
}

fn despawn_options(
    mut commands: Commands,
    query: Query<Entity, With<Options>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn mouse_input_handle(
    mut next_game_state: ResMut<NextState<GameState>>,
    current_screen_state: Res<State<ScreenState>>,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut exit: EventWriter<AppExit>,
    mut query: Query<
        (&TextButton<MenuItemId>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
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
                match current_screen_state.get() {
                    ScreenState::Main => {
                        if button.payload == CONTINUE_MENU_ITEM_ID {
                            return;
                        }

                        if button.payload == NEW_MENU_ITEM_ID {
                            return;
                        }

                        if button.payload == OPTIONS_MENU_ITEM_ID {
                            next_state.set(ScreenState::Options);
                            return;
                        }

                        if button.payload == EXIT_MENU_ITEM_ID {
                            exit.send(AppExit::Success);
                            return;
                        }
                    }
                    ScreenState::Options => {
                        if button.payload == DEV_SETTINGS_OPTION_ITEM_ID {
                            next_game_state.set(GameState::DevSetting);
                            return;
                        }
                        return;
                    }
                }
            }
        }
    }
}

const CONTINUE_MENU_ITEM_ID: MenuItemId = MenuItemId(1);
const NEW_MENU_ITEM_ID: MenuItemId = MenuItemId(2);
const OPTIONS_MENU_ITEM_ID: MenuItemId = MenuItemId(3);
const EXIT_MENU_ITEM_ID: MenuItemId = MenuItemId(4);
const DEV_SETTINGS_OPTION_ITEM_ID: MenuItemId = MenuItemId(11);


