use std::fmt::format;
use std::ops::Deref;

use bevy::app::{Plugin, Update};
use bevy::audio::{AudioBundle, PlaybackSettings};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::color::{Color, Srgba};
use bevy::hierarchy::{Children, DespawnRecursiveExt};
use bevy::log::{error, warn};
use bevy::prelude::in_state;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyContent;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::prelude::{BackgroundColor, Res, ResMut, Style, Visibility};
use bevy::render::render_resource::encase::private::RuntimeSizedArray;
use bevy::ui::{FocusPolicy, UiRect};
use bevy::utils::hashbrown::HashMap;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetPaddingExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilderExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiRoot;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::prelude::{
    PseudoState, SetBackgroundColorExt, SetFocusPolicyExt, SetSizeExt, SetVisibilityExt,
};
use sickle_ui::ui_commands::{ManagePseudoStateExt, UpdateTextExt};

use crate::core::states::GameState;
use crate::gui::{ButtonConfig, TextButton, TextButtonExt, TextConfig, TextExt};
use crate::party::PartyStateStorage;
use crate::rpg::character::{Character, Class};
use crate::rpg::characteristic_item_ui::select_item_handle;
use crate::rpg::characteristic_item_ui::CharacteristicAction;
use crate::rpg::characteristic_item_ui::CharacteristicItemExt;
use crate::rpg::characteristic_item_ui::CharacteristicValues;
use crate::rpg::characteristic_item_ui::Description;
use crate::rpg::characteristic_item_ui::{Characteristic, CharacteristicValue};
use crate::rpg::stat_item_ui::{Stat, StatItemExt, StatValue, StatsValues};
use crate::rpg::storages::CharacterStorage;
use crate::rpg::title_ui::{Title, TitleAction, TitleExt};
use crate::rpg::{Ability, DirectionalAttack, RangedProp, TargetProps};
use crate::sound::ButtonSounds;

pub struct CharacterScreenPlugin;

#[derive(Component)]
struct CharacterScreen;

#[derive(Component)]
struct Scores(RangedProp);

#[derive(Component)]
struct Level;

#[derive(Component)]
struct Exp;

#[derive(Component)]
struct Charisma;

#[derive(Component)]
struct Characters {
    items: Vec<Character>,
    current: usize,
}

impl Characters {
    fn next(&mut self) {
        if self.current == self.items.len() - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
    }

    fn back(&mut self) {
        if self.current == 0 {
            self.current = self.items.len() - 1;
        } else {
            self.current -= 1;
        }
    }

    fn current(&self) -> &Character {
        &self.items[self.current]
    }
}

#[derive(Component)]
struct ConfirmButton;

impl Plugin for CharacterScreenPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(OnEnter(GameState::Character), spawn_main)
            .add_systems(OnExit(GameState::Character), despawn_main)
            .add_systems(
                Update,
                (
                    change_characteristic_handle,
                    update_characteristics_value_labels_handle,
                    update_stats_value_labels_handle,
                    update_scores_handle,
                    update_character_name_handle,
                    update_character_level_handle,
                    update_character_exp_handle,
                    change_character_handle,
                    confirm_handle,
                    select_item_handle::<Characteristic>,
                    select_item_handle::<Stat>,
                )
                    .run_if(in_state(GameState::Character)),
            );
    }
}

fn spawn_main(mut commands: Commands, character_storage: Res<CharacterStorage>) {
    let characters_items = character_storage.get();

    let character = characters_items
        .get(0)
        .expect("characters must not be empty");

    let char_values = to_screen_values(character);

    let mut stat_values = HashMap::new();
    stat_values.insert(Stat::BaseAttack, 0);
    stat_values.insert(Stat::Health, 0);
    stat_values.insert(Stat::Energy, 0);
    stat_values.insert(Stat::Armor, 0);
    stat_values.insert(Stat::Evasion, 0);

    recalculate_stats(&char_values, &mut stat_values);

    let scores = Scores(RangedProp {
        min: 0,
        current: character.level.available_points,
        max: character.level.available_points,
    });

    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent
                .row(|parent| {
                    parent
                        .column(|parent| {
                            parent.title().style().height(Val::Percent(10.0));
                            parent
                                .configure_text(
                                    "",
                                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                                )
                                .insert(scores)
                                .style()
                                .height(Val::Percent(10.0));
                            parent
                                .column(|parent| {
                                    parent.characteristic(Characteristic::Strength);
                                    parent.characteristic(Characteristic::Agility);
                                    parent.characteristic(Characteristic::Stamina);
                                    parent.characteristic(Characteristic::Fortitude);
                                    parent
                                        .characteristic(Characteristic::Charisma)
                                        .insert(Charisma)
                                        .style()
                                        .visibility(Visibility::Visible);
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

                    parent
                        .column(|parent| {
                            parent
                                .configure_text(
                                    "",
                                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                                )
                                .insert(Level)
                                .style()
                                .height(Val::Percent(10.0));

                            parent
                                .configure_text(
                                    "",
                                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                                )
                                .insert(Exp)
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
                .row(|parent| {
                    parent
                        .configure_text("", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                        .insert(Description)
                        .style()
                        .width(Val::Percent(70.0));

                    parent
                        .configure_text_button(
                            "Подтвердить",
                            ConfirmButton,
                            TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                            ButtonConfig {
                                width: Val::Px(50.0),
                                height: Val::Px(50.0),
                                idle: BackgroundColor::from(Color::NONE),
                                hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                                pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                                justify_content: JustifyContent::Center,
                            },
                        )
                        .style()
                        .width(Val::Percent(30.0))
                        .focus_policy(FocusPolicy::Pass);
                })
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
            CharacterScreen,
            CharacteristicValues(char_values),
            StatsValues(stat_values),
            Characters {
                items: characters_items,
                current: 0,
            },
        ))
        .style()
        .justify_content(JustifyContent::Start)
        .size(Val::Percent(100.0))
        .background_color(Color::from(SCREEN_BG));
}

fn to_screen_values(character: &Character) -> HashMap<Characteristic, RangedProp> {
    let mut char_values = HashMap::new();
    match character.class {
        Class::FormidableFace {
            strength,
            agility,
            stamina,
            fortitude,
            charisma,
        } => {
            let strength = RangedProp {
                min: strength,
                current: strength,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Strength, strength);
            let agility = RangedProp {
                min: agility,
                current: agility,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Agility, agility);
            let stamina = RangedProp {
                min: stamina,
                current: stamina,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Stamina, stamina);

            let fortitude = RangedProp {
                min: fortitude,
                current: fortitude,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Fortitude, fortitude);

            let charisma = RangedProp {
                min: charisma,
                current: charisma,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Charisma, charisma);
        }
        Class::FormidableDog {
            strength,
            agility,
            stamina,
            fortitude,
        } => {
            let strength = RangedProp {
                min: strength,
                current: strength,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Strength, strength);
            let agility = RangedProp {
                min: agility,
                current: agility,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Agility, agility);
            let stamina = RangedProp {
                min: stamina,
                current: stamina,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Stamina, stamina);

            let fortitude = RangedProp {
                min: fortitude,
                current: fortitude,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Fortitude, fortitude);

            let charisma = RangedProp {
                min: 0,
                current: 0,
                max: character.level.characteristic_max_value,
            };
            char_values.insert(Characteristic::Charisma, charisma);
        }
    }
    char_values
}

pub fn change_characteristic_handle(
    mut query: Query<
        (
            &TextButton<(CharacteristicAction, Characteristic)>,
            &Interaction,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    mut characteristic_values_query: Query<(&mut CharacteristicValues)>,
    mut stats_values_query: Query<(&mut StatsValues)>,
    mut scores_query: Query<(&mut Scores)>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => *background_color = item.config.hover,
            Interaction::Pressed => {
                let chars = &mut characteristic_values_query.single_mut().0;
                let value = chars
                    .get_mut(&item.payload.1)
                    .expect(&format!("No value with key = {:?}", &item.payload.1));
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

fn change_character_handle(
    mut query: Query<
        (
            &TextButton<(TitleAction)>,
            &Interaction,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    mut characters_value_query: Query<(&mut Characters)>,
    mut scores_value_query: Query<(&mut Scores)>,
    mut characteristic_values_query: Query<(&mut CharacteristicValues)>,
    mut stats_values_query: Query<(&mut StatsValues)>,
    mut visibility_query: Query<(&mut Visibility), With<Charisma>>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => *background_color = item.config.hover,
            Interaction::Pressed => {
                let value = &mut characters_value_query.single_mut();
                match item.payload {
                    TitleAction::Next => {
                        value.next();
                    }
                    TitleAction::Back => {
                        value.back();
                    }
                }
                let scores = &mut scores_value_query.single_mut();
                scores.0 = RangedProp {
                    min: 0,
                    current: value.current().level.available_points,
                    max: value.current().level.available_points,
                };

                let characteristics = &mut characteristic_values_query.single_mut();
                let stats = &mut stats_values_query.single_mut();
                let char_values = to_screen_values(value.current());
                recalculate_stats(&char_values, &mut stats.0);
                characteristics.0 = char_values;
                for mut visibility in visibility_query.iter_mut() {
                    match value.current().class {
                        Class::FormidableFace { .. } => {
                            *visibility = Visibility::Visible;
                        }
                        Class::FormidableDog { .. } => {
                            *visibility = Visibility::Hidden;
                        }
                    };
                }
            }
        }
    }
}

fn confirm_handle(
    mut commands: Commands,
    mut character_storage: ResMut<CharacterStorage>,
    mut party_state_storage: ResMut<PartyStateStorage>,
    mut scores_query: Query<(&mut Scores)>,
    mut query: Query<
        (
            &TextButton<(ConfirmButton)>,
            &Interaction,
            &mut BackgroundColor,
        ),
        Changed<Interaction>,
    >,
    audio_res: Res<ButtonSounds>,
    mut characters_value_query: Query<(&mut Characters)>,
    mut characteristic_values_query: Query<(&mut CharacteristicValues)>,
    stats_values_query: Query<(&StatsValues)>,
) {
    for (mut item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => *background_color = item.config.hover,
            Interaction::Pressed => {
                let scores = scores_query.single();
                if scores.0.current == scores.0.min && scores.0.max != 0 {
                    let char_id = characters_value_query.single().current;
                    let mut characteristics = characteristic_values_query.single_mut();
                    let class =
                        map_to_class(&characteristics, character_storage.get_class_by_id(char_id));
                    for (_, mut props) in &mut characteristics.0 {
                        props.min = props.current;
                    }

                    let stats = stats_values_query.single();
                    let targets = map_to_target_props(stats);
                    let base_attack = stats
                        .0
                        .get(&Stat::BaseAttack)
                        .expect("No value set")
                        .clone();
                    party_state_storage.update_base_attack_by_id(char_id, base_attack);
                    party_state_storage.update_target_props_by_id(char_id, targets);
                    let new_level = character_storage.get_level_by_id(char_id).up_level();
                    party_state_storage.update_attacks_by_id(
                        char_id,
                        map_to_attacks(&new_level, &class, base_attack),
                    );
                    party_state_storage
                        .update_abilities_by_id(char_id, map_to_abilities(&new_level, &class));
                    character_storage.update_class_by_id(char_id, class);
                    character_storage.update_level_by_id(char_id, new_level);
                    scores_query.single_mut().0 = RangedProp::default();
                    let mut characters = characters_value_query.single_mut();
                    characters.items = character_storage.get();
                } else {
                    commands.spawn(AudioBundle {
                        source: audio_res.negative_click.clone(),
                        settings: PlaybackSettings::ONCE,
                    });
                }
            }
        }
    }
}

fn map_to_class(chars: &CharacteristicValues, class: &Class) -> Class {
    match class {
        Class::FormidableFace { .. } => Class::FormidableFace {
            strength: chars
                .0
                .get(&Characteristic::Strength)
                .expect("No value set")
                .current,
            agility: chars
                .0
                .get(&Characteristic::Agility)
                .expect("No value set")
                .current,
            stamina: chars
                .0
                .get(&Characteristic::Stamina)
                .expect("No value set")
                .current,
            fortitude: chars
                .0
                .get(&Characteristic::Fortitude)
                .expect("No value set")
                .current,
            charisma: chars
                .0
                .get(&Characteristic::Charisma)
                .expect("No value set")
                .current,
        },
        Class::FormidableDog { .. } => Class::FormidableDog {
            strength: chars
                .0
                .get(&Characteristic::Strength)
                .expect("No value set")
                .current,
            agility: chars
                .0
                .get(&Characteristic::Agility)
                .expect("No value set")
                .current,
            stamina: chars
                .0
                .get(&Characteristic::Stamina)
                .expect("No value set")
                .current,
            fortitude: chars
                .0
                .get(&Characteristic::Fortitude)
                .expect("No value set")
                .current,
        },
    }
}

fn map_to_attacks(
    level: &crate::rpg::character::Level,
    class: &Class,
    base_attack: i32,
) -> Vec<DirectionalAttack> {
    match class {
        Class::FormidableFace { .. } => match level.current {
            1 => vec![DirectionalAttack::Punch {
                damage: base_attack + 5,
            }],
            2 => vec![
                DirectionalAttack::Punch {
                    damage: base_attack + 5,
                },
                DirectionalAttack::Kick {
                    damage: base_attack + 15,
                },
            ],
            _otherwise => vec![
                DirectionalAttack::Punch {
                    damage: base_attack + 6,
                },
                DirectionalAttack::Kick {
                    damage: base_attack + 20,
                },
            ],
        },
        Class::FormidableDog { .. } => match level.current {
            1 => vec![DirectionalAttack::Bite {
                damage: base_attack + 7,
            }],
            2 => vec![
                DirectionalAttack::Bite {
                    damage: base_attack + 7,
                },
                DirectionalAttack::PawStrike {
                    damage: base_attack + 15,
                },
            ],
            _otherwise => vec![
                DirectionalAttack::Bite {
                    damage: base_attack + 10,
                },
                DirectionalAttack::PawStrike {
                    damage: base_attack + 20,
                },
            ],
        },
    }
}

fn map_to_abilities(level: &crate::rpg::character::Level, class: &Class) -> Vec<Ability> {
    match class {
        Class::FormidableFace { .. } => match level.current {
            1 => vec![Ability::NeckTwist {
                damage: 30,
                cost: 10,
            }],
            2 => vec![
                Ability::NeckTwist {
                    damage: 40,
                    cost: 15,
                },
                Ability::SuperPunch {
                    damage: 60,
                    cost: 25,
                },
            ],
            _otherwise => vec![
                Ability::NeckTwist {
                    damage: 50,
                    cost: 20,
                },
                Ability::SuperPunch {
                    damage: 70,
                    cost: 30,
                },
            ],
        },
        Class::FormidableDog { .. } => match level.current {
            1 => vec![Ability::WoundsLicking {
                health: 20,
                cost: 15,
            }],
            2 => vec![Ability::WoundsLicking {
                health: 30,
                cost: 15,
            }],
            _otherwise => vec![
                Ability::WoundsLicking {
                    health: 50,
                    cost: 25,
                },
                Ability::NeckGnawing {
                    damage: 50,
                    cost: 20,
                },
            ],
        },
    }
}

fn map_to_target_props(stats: &StatsValues) -> TargetProps {
    let health = stats.0.get(&Stat::Health).expect("No value set");
    let energy = stats.0.get(&Stat::Energy).expect("No value set");
    TargetProps {
        health: RangedProp {
            min: 0,
            current: health.clone(),
            max: health.clone(),
        },
        energy: RangedProp {
            min: 0,
            current: energy.clone(),
            max: energy.clone(),
        },
        armor: stats.0.get(&Stat::Armor).expect("No value sat").clone(),
        evasion: stats.0.get(&Stat::Evasion).expect("No value sat").clone(),
    }
}

fn recalculate_stats(chars: &HashMap<Characteristic, RangedProp>, stats: &mut HashMap<Stat, i32>) {
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
                    None => {
                        warn!("CharacteristicValue is not found")
                    }
                    Some(mut entity_commands) => {
                        let value = values
                            .0
                            .get(char)
                            .expect(&format!("No value with key = {:?}", char));
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
                    None => {
                        warn!("StatValue is not found")
                    }
                    Some(mut entity_commands) => {
                        let value = values
                            .0
                            .get(stat)
                            .expect(&format!("No value with key = {:?}", stat));
                        entity_commands.update_text(format!("{}", value));
                    }
                }
            }
        }
    }
}

fn update_character_name_handle(
    mut commands: Commands,
    mut children_query: Query<(&Children), With<Title>>,
    characters_value_query: Query<(&Characters), Changed<Characters>>,
) {
    for value in characters_value_query.iter() {
        for mut children in children_query.iter() {
            for &child in children.iter() {
                match commands.get_entity(child) {
                    None => {
                        warn!("Title is not found")
                    }
                    Some(mut entity_commands) => {
                        let current = value.current();
                        entity_commands.update_text(format!("{}", current.name));
                    }
                }
            }
        }
    }
}

fn update_character_level_handle(
    mut commands: Commands,
    mut children_query: Query<(&Children), With<Level>>,
    characters_value_query: Query<(&Characters), Changed<Characters>>,
) {
    for value in characters_value_query.iter() {
        for mut children in children_query.iter() {
            for &child in children.iter() {
                match commands.get_entity(child) {
                    None => {
                        warn!("Level is not found")
                    }
                    Some(mut entity_commands) => {
                        let current = value.current();
                        entity_commands.update_text(format!("Level {}", current.level.current));
                    }
                }
            }
        }
    }
}

fn update_character_exp_handle(
    mut commands: Commands,
    mut children_query: Query<(&Children), With<Exp>>,
    characters_value_query: Query<(&Characters), Changed<Characters>>,
) {
    for value in characters_value_query.iter() {
        for mut children in children_query.iter() {
            for &child in children.iter() {
                match commands.get_entity(child) {
                    None => {
                        warn!("Exp. is not found")
                    }
                    Some(mut entity_commands) => {
                        let current = value.current();
                        entity_commands.update_text(format!(
                            "Exp. {} / {}",
                            current.level.current_experience, current.level.experience_for_the_next,
                        ));
                    }
                }
            }
        }
    }
}

fn update_scores_handle(
    mut commands: Commands,
    mut scores_query: Query<(&Children, &Scores), Changed<Scores>>,
    mut confirm_button_style_query: Query<(&mut BackgroundColor, &mut TextButton<(ConfirmButton)>)>,
) {
    for (mut children, scores) in scores_query.iter() {
        let (mut background, mut button) = confirm_button_style_query.single_mut();
        if scores.0.current > scores.0.min || scores.0.max == 0 {
            button.config.hover = BackgroundColor(DISABLED_BUTTON_COLOR);
            button.config.idle = BackgroundColor(DISABLED_BUTTON_COLOR);
            *background = button.config.idle;
        } else {
            button.config.hover = BackgroundColor(PRESSED_HOVER_BUTTON_COLOR);
            button.config.idle = BackgroundColor(Color::NONE);
            *background = button.config.idle;
        }
        for &child in children.iter() {
            match commands.get_entity(child) {
                None => {
                    warn!("Scores is not found")
                }
                Some(mut entity_commands) => {
                    entity_commands.update_text(format!("Доступные очки: {}", scores.0.current));
                }
            }
        }
    }
}

fn despawn_main(mut commands: Commands, query: Query<Entity, With<CharacterScreen>>) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

/// <div style="background-color:rgb(50.0%, 39.4%, 21.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const SCREEN_BG: Srgba = Srgba::new(0.5, 0.394, 0.21, 1.0);
/// <div style="background-color:rgb(60.0%, 44.4%, 25.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.6, 0.444, 0.25, 1.0);
/// <div style="background-color:rgb(80.0%, 80.0%, 80.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const DISABLED_BUTTON_COLOR: Color = Color::srgba(0.8, 0.8, 0.8, 0.8);
