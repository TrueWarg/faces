use bevy::app::{Plugin, Update};
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{Commands, Component, Entity, in_state, IntoSystemConfigs, JustifyContent, OnEnter, OnExit, Query, Val, With};
use bevy::ui::UiRect;
use sickle_ui::prelude::{SetBackgroundColorExt, SetHeightExt, SetJustifyContentExt, SetPaddingExt, SetWidthExt, UiBuilderExt, UiColumnExt, UiRoot, UiRowExt};

use crate::core::states::GameState;
use crate::gui::{TextConfig, TextExt};
use crate::rpg::characteristic_item_ui::{change_value_handle, CharacteristicId, CharacteristicItemExt, Description, select_item_handle};

pub struct LevelScreenScreenPlugin;

#[derive(Component)]
struct LevelUpScreen;

#[derive(Component)]
struct Scores(pub usize);

impl Plugin for LevelScreenScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(GameState::LevelUp), spawn_main)
            .add_systems(OnExit(GameState::LevelUp), despawn_main)
            .add_systems(Update, (change_value_handle, select_item_handle).run_if(in_state(GameState::LevelUp)))

        ;
    }
}

fn spawn_main(
    mut commands: Commands,
) {
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.row(|parent| {
                parent
                    .column(|parent| {
                        parent
                            .configure_text("Formidable", TextConfig::large(Color::from(ANTIQUE_WHITE)))
                            .style()
                            .height(Val::Percent(10.0));
                        parent
                            .configure_text("Доступные очки: 5", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                            .style()
                            .height(Val::Percent(10.0));
                        parent
                            .column(|parent| {
                                parent.characteristic(CharacteristicId(0), "Сила", 1, 1, 5);
                                parent.characteristic(CharacteristicId(1), "Выкрутасность", 1, 1, 5);
                                parent.characteristic(CharacteristicId(2), "Стамина", 1, 1, 5);
                                parent.characteristic(CharacteristicId(3), "Стойкость", 1, 1, 5);
                                parent.characteristic(CharacteristicId(4), "Языкастость", 1, 1, 5);
                            })
                            .style()
                            .justify_content(JustifyContent::SpaceAround)
                            .width(Val::Percent(100.0))
                            .height(Val::Percent(50.0));
                    })
                    .style()
                    .justify_content(JustifyContent::SpaceAround)
                    .width(Val::Percent(50.0))
                    .height(Val::Percent(100.0))
                    .padding(UiRect {
                        left: Val::Percent(5.0),
                        right: Val::Percent(5.0),
                        top: Val::Percent(5.0),
                        bottom: Val::Percent(0.0),
                    });

                parent.column(|parent| {
                    parent
                        .configure_text("Level 1", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                        .style()
                        .height(Val::Percent(10.0));

                    parent
                        .configure_text("Exp: 0 / 2000", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                        .style()
                        .height(Val::Percent(10.0));

                    parent
                        .column(|parent| {
                            parent.characteristic(CharacteristicId(0), "Базовый урон", 1, 1, 5);
                            parent.characteristic(CharacteristicId(1), "Здоровье", 1, 1, 5);
                            parent.characteristic(CharacteristicId(2), "Энергия", 1, 1, 5);
                            parent.characteristic(CharacteristicId(3), "Броня", 1, 1, 5);
                            parent.characteristic(CharacteristicId(4), "Уклонение", 1, 1, 5);
                        })
                        .style()
                        .justify_content(JustifyContent::SpaceAround)
                        .width(Val::Percent(100.0))
                        .height(Val::Percent(50.0));
                })
                    .style()
                    .justify_content(JustifyContent::SpaceAround)
                    .width(Val::Percent(50.0))
                    .height(Val::Percent(100.0))
                    .padding(UiRect {
                        left: Val::Percent(5.0),
                        right: Val::Percent(5.0),
                        top: Val::Percent(5.0),
                        bottom: Val::Percent(0.0),
                    });
            })
                .style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(70.0));

            parent
                .configure_text("", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                .insert(Description)
                .style()
                .height(Val::Percent(30.0))
                .padding(UiRect {
                    left: Val::Percent(5.0),
                    right: Val::Percent(5.0),
                    top: Val::Percent(5.0),
                    bottom: Val::Percent(5.0),
                });
        })
        .insert(LevelUpScreen)
        .style()
        .justify_content(JustifyContent::Start)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .background_color(Color::from(SCREEN_BG));
    ;
}

fn despawn_main(
    mut commands: Commands,
    query: Query<Entity, With<LevelUpScreen>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

/// <div style="background-color:rgb(50.0%, 39.4%, 21.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const SCREEN_BG: Srgba = Srgba::new(0.5, 0.394, 0.21, 1.0);
