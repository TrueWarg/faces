use std::fmt::format;
use std::ops::Deref;

use bevy::app::{Plugin, Update};
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::hierarchy::{Children, DespawnRecursiveExt};
use bevy::log::warn;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::in_state;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyContent;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::ui::UiRect;
use bevy::utils::hashbrown::HashMap;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetPaddingExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilderExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiRoot;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::ui_commands::UpdateTextExt;

use crate::core::states::GameState;
use crate::gui::{TextButton, TextConfig, TextExt};
use crate::rpg::characteristic_item_ui::{Characteristic, CharacteristicValue};
use crate::rpg::characteristic_item_ui::CharacteristicAction;
use crate::rpg::characteristic_item_ui::CharacteristicItemExt;
use crate::rpg::characteristic_item_ui::CharacteristicValues;
use crate::rpg::characteristic_item_ui::Description;
use crate::rpg::characteristic_item_ui::select_item_handle;
use crate::rpg::RangedProp;
use crate::rpg::stat_item_ui::{Stat, StatItemExt, StatsValues, StatValue};

pub struct LevelScreenScreenPlugin;

#[derive(Component)]
struct LevelUpScreen;

#[derive(Component)]
struct Scores(pub RangedProp);

impl Plugin for LevelScreenScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_systems(OnEnter(GameState::LevelUp), spawn_main)
            .add_systems(OnExit(GameState::LevelUp), despawn_main)
            .add_systems(Update, (change_characteristic_handle,
                                  update_characteristics_value_labels_handle,
                                  update_stats_value_labels_handle,
                                  update_scores_handle,
                                  select_item_handle::<Characteristic>,
                                  select_item_handle::<Stat>,
            ).run_if(in_state(GameState::LevelUp)))

        ;
    }
}

fn spawn_main(
    mut commands: Commands,
) {
    let mut char_values = HashMap::new();
    let prop = RangedProp {
        min: 1,
        current: 1,
        max: 5,
    };
    char_values.insert(Characteristic::Strength, prop.clone());
    char_values.insert(Characteristic::Agility, prop.clone());
    char_values.insert(Characteristic::Stamina, prop.clone());
    char_values.insert(Characteristic::Fortitude, prop.clone());
    char_values.insert(Characteristic::Charisma, prop);

    let mut stat_values = HashMap::new();
    stat_values.insert(Stat::BaseAttack, 0);
    stat_values.insert(Stat::Health, 0);
    stat_values.insert(Stat::Energy, 0);
    stat_values.insert(Stat::Armor, 0);
    stat_values.insert(Stat::Evasion, 0);

    recalculate_stats(&char_values, &mut stat_values);

    let scores = Scores(RangedProp {
        min: 0,
        current: 15,
        max: 15,
    });

    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.row(|parent| {
                parent
                    .column(|parent| {
                        parent
                            .configure_text("Грозный", TextConfig::large(Color::from(ANTIQUE_WHITE)))
                            .style()
                            .height(Val::Percent(10.0));
                        parent
                            .configure_text(
                                format!("Доступные очки: {}", scores.0.current),
                                TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                            .insert(scores)
                            .style()
                            .height(Val::Percent(10.0));
                        parent
                            .column(|parent| {
                                parent.characteristic(Characteristic::Strength);
                                parent.characteristic(Characteristic::Agility);
                                parent.characteristic(Characteristic::Stamina);
                                parent.characteristic(Characteristic::Fortitude);
                                parent.characteristic(Characteristic::Charisma);
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
                            parent.stat(Stat::BaseAttack);
                            parent.stat(Stat::Health);
                            parent.stat(Stat::Energy);
                            parent.stat(Stat::Armor);
                            parent.stat(Stat::Evasion);
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
        .insert((
            LevelUpScreen,
            CharacteristicValues(char_values),
            StatsValues(stat_values),
        ))
        .style()
        .justify_content(JustifyContent::Start)
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .background_color(Color::from(SCREEN_BG));
}


pub fn change_characteristic_handle(
    mut query: Query<
        (&TextButton<(CharacteristicAction, Characteristic)>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>>,
    mut characteristic_values_query: Query<(&mut CharacteristicValues)>,
    mut stats_values_query: Query<(&mut StatsValues)>,
    mut scores_query: Query<(&mut Scores)>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => {
                *background_color = item.config.hover
            }
            Interaction::Pressed => {
                let chars = &mut characteristic_values_query.single_mut().0;
                let value = chars.get_mut(&item.payload.1).expect(
                    &format!("No value with key = {:?}", &item.payload.1)
                );
                let scores = &mut scores_query.single_mut().0;
                match item.payload.0 {
                    CharacteristicAction::Increase => {
                        if scores.current > scores.min {
                            if value.increase(1) {
                                scores.decrease(1);
                            }
                        }
                    }
                    CharacteristicAction::Decrease => {
                        if value.decrease(1) {
                            scores.increase(1);
                        }
                    }
                }
                let stats = &mut stats_values_query.single_mut().0;
                recalculate_stats(&chars, stats);
            }
        }
    }
}

fn recalculate_stats(
    chars: &HashMap<Characteristic, RangedProp>,
    stats: &mut HashMap<Stat, i32>,
) {
    let strength = chars.get(&Characteristic::Strength).expect("");
    let agility = chars.get(&Characteristic::Agility).expect("");
    let stamina = chars.get(&Characteristic::Stamina).expect("");
    let fortitude = chars.get(&Characteristic::Fortitude).expect("");

    let attack = stats.get_mut(&Stat::BaseAttack).expect("");
    *attack = 10 + 3 * strength.current + 2 * agility.current;

    let health = stats.get_mut(&Stat::Health).expect("");
    *health = 100 + 10 * fortitude.current + 3 * stamina.current;

    let energy = stats.get_mut(&Stat::Energy).expect("");
    *energy = 20 + 4 * stamina.current;

    let armor = stats.get_mut(&Stat::Armor).expect("");
    *armor = 4 * fortitude.current;

    let evasion = stats.get_mut(&Stat::Evasion).expect("");
    *evasion = 4 * agility.current;
}

fn update_characteristics_value_labels_handle(
    mut commands: Commands,
    mut characteristic_value_query: Query<(&Children, &Characteristic), With<CharacteristicValue>>,
    characteristic_values_query: Query<(&CharacteristicValues), Changed<CharacteristicValues>>,
) {
    for values in characteristic_values_query.iter() {
        for (mut children, char) in characteristic_value_query.iter() {
            for &child in children.iter() {
                match commands.get_entity(child) {
                    None => { warn!("CharacteristicValue is not found") }
                    Some(mut entity_commands) => {
                        let value = values.0.get(char)
                            .expect(
                                &format!("No value with key = {:?}", char)
                            );
                        ;
                        entity_commands.update_text(format!("{}", value.current));
                    }
                }
            }
        }
    }
}

fn update_stats_value_labels_handle(
    mut commands: Commands,
    mut stat_value_query: Query<(&Children, &Stat), With<StatValue>>,
    stat_values_query: Query<(&StatsValues), Changed<StatsValues>>,
) {
    for values in stat_values_query.iter() {
        for (mut children, stat) in stat_value_query.iter() {
            for &child in children.iter() {
                match commands.get_entity(child) {
                    None => { warn!("StatValue is not found") }
                    Some(mut entity_commands) => {
                        let value = values.0.get(stat)
                            .expect(
                                &format!("No value with key = {:?}", stat)
                            );
                        ;
                        entity_commands.update_text(format!("{}", value));
                    }
                }
            }
        }
    }
}

fn update_scores_handle(
    mut commands: Commands,
    mut scores_query: Query<(&Children, &Scores), Changed<Scores>>,
) {
    for (mut children, scores) in scores_query.iter() {
        for &child in children.iter() {
            match commands.get_entity(child) {
                None => { warn!("Scores is not found") }
                Some(mut entity_commands) => {
                    entity_commands.update_text(format!("Доступные очки: {}", scores.0.current));
                }
            }
        }
    }
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
