use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::color::palettes::css::{ANTIQUE_WHITE, DIM_GREY, WHITE};
use bevy::hierarchy::{Children, DespawnRecursiveExt};
use bevy::log::warn;
use bevy::prelude::{AlignItems, default, ImageBundle, Interaction, NextState, PositionType, ResMut, State, UiImage};
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::JustifyContent;
use bevy::prelude::NodeBundle;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::UiRect;
use bevy::prelude::Update;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::ui::{AlignSelf, FocusPolicy};
use bevy::utils::HashMap;
use sickle_ui::prelude::{ScrollAxis, SetBorderColorExt, SetBorderExt, SetLeftExt, SetPositionTypeExt, SetTopExt};
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetAlignSelfExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetFocusPolicyExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiContainerExt;
use sickle_ui::prelude::UiRoot;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::prelude::UiScrollViewExt;
use sickle_ui::ui_builder::UiBuilderExt;
use sickle_ui::ui_commands::UpdateTextExt;

use crate::core::states::GameState;
use crate::dialog::{Branching, Dialog, DialogEffect, DialogId, DialogsStorage, DialogStick, SelectedVariantsSource};
use crate::fight::FightId;
use crate::gui::{ButtonConfig, TextButton, TextButtonExt, TextConfig, TextExt};
use crate::world_state::EscapeFromHouse;

pub struct DialogScene;

#[derive(Component)]
struct DialogSceneScreen;

#[derive(Component)]
struct DialogOptions;

#[derive(Component)]
struct Sticks(Vec<(usize, usize)>);

#[derive(Component)]
struct CurrentReplica(String);

#[derive(Component)]
struct CurrentBranching(Option<Branching>);

#[derive(Component)]
struct LastPhrase(String);

#[derive(Component)]
struct OptionId(usize);

impl Plugin for DialogScene {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Dialog), spawn_main)
            .add_systems(OnExit(GameState::Dialog), unspawn)
            .add_systems(Update, dialog_options_panel_respawns)
            .add_systems(Update, (
                option_input_handle,
                dialog_options_updates,
                current_replica_updates,
            ),
            );
    }
}

fn spawn_main(
    mut commands: Commands,
    query: Query<(&DialogId)>,
    asset_server: Res<AssetServer>,
    dialogs_storage: Res<DialogsStorage>,
) {
    let dialog_id = query.single();
    let dialog = dialogs_storage
        .get_by_id(&dialog_id.0)
        .expect(&format!("No dialog with id {}", &dialog_id.0));
    let root_stick = dialog.get_root_stick();
    let current_replica = if root_stick.replicas_size() > 0 {
        CurrentReplica(root_stick.first_replica().text.clone())
    } else {
        CurrentReplica("".to_string())
    };
    let current_branching = if root_stick.replicas_size() > 0 {
        CurrentBranching(None)
    } else {
        CurrentBranching(root_stick.get_branching().clone())
    };

    let id_to_replica_position = vec![(root_stick.id, 0)];

    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.container(ImageBundle {
                image: UiImage {
                    texture: asset_server.load(&dialog.bg_path),
                    ..default()
                },
                ..default()
            }, |parent| {
                parent.container(ImageBundle {
                    image: UiImage { texture: asset_server.load(&dialog.character_path), ..default() },
                    ..default()
                }, |parent| {},
                )
                    .style()
                    .width(Val::Auto)
                    .height(Val::Percent(70.0))
                    .position_type(PositionType::Relative)
                    .top(Val::Percent(30.0))
                    .left(Val::Percent(40.0));
            })
                .style()
                .height(Val::Percent(60.0))
                .width(Val::Percent(100.0));

            parent
                .column(|parent| {
                    parent
                        .configure_text("", TextConfig::small(Color::from(ANTIQUE_WHITE)))
                        .insert(current_replica)
                        .style()
                        .margin(
                            UiRect {
                                left: Val::Px(20.0),
                                right: Val::Px(20.0),
                                top: Val::Px(20.0),
                                bottom: Val::Px(20.0),
                            }
                        );
                })
                .style()
                .justify_content(JustifyContent::Start)
                .align_items(AlignItems::Start)
                .border(UiRect {
                    left: Val::Px(5.0),
                    right: Val::Px(5.0),
                    top: Val::Px(5.0),
                    bottom: Val::Px(5.0),
                })
                .border_color(Color::from(ANTIQUE_WHITE))
                .background_color(Color::from(DIM_GREY))
                .height(Val::Percent(15.0))
                .width(Val::Percent(100.0));
        })
        .insert((
            DialogSceneScreen,
            current_branching,
            dialog,
            Sticks(id_to_replica_position),
        )
        )
        .style()
        .justify_content(JustifyContent::Start)
        .align_items(AlignItems::Start)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center);
}

fn dialog_options_panel_respawns(
    mut commands: Commands,
    options_query: Query<Entity, With<DialogOptions>>,
    branching_query: Query<(&CurrentBranching), Changed<CurrentBranching>>,
) {
    for branching in branching_query.iter() {
        for entity in options_query.iter() {
            commands.entity(entity).despawn_recursive();
        }
        commands
            .ui_builder(UiRoot)
            .row(|parent| {
                parent.scroll_view(Some(ScrollAxis::Vertical), |parent| {
                    match &branching.0 {
                        None => {
                            option_button(parent, BTN_NEXT_ID, "NEXT".to_string());
                        }
                        Some(value) => {
                            let variants = &value.variants;
                            for (pos, item) in variants.iter().enumerate() {
                                option_button(parent, pos, format!("{}. {}", pos + 1, item.label));
                            }
                        }
                    }
                })
                    .style()
                    .width(Val::Percent(100.0))
                    .height(Val::Percent(100.0));
            })
            .insert(DialogOptions)
            .style()
            .border(UiRect {
                left: Val::Px(5.0),
                right: Val::Px(5.0),
                top: Val::Px(0.0),
                bottom: Val::Px(5.0),
            })
            .border_color(Color::from(ANTIQUE_WHITE))
            .background_color(Color::from(DIM_GREY))
            .justify_content(JustifyContent::End)
            .align_items(AlignItems::End)
            .height(Val::Percent(25.0))
            .width(Val::Percent(100.0))
            .align_self(AlignSelf::End);
    }
}

fn option_button(
    parent: &mut UiBuilder<Entity>,
    id: usize,
    label: String,
) {
    parent
        .configure_text_button(
            label,
            OptionId(id),
            TextConfig::small(Color::from(ANTIQUE_WHITE)),
            ButtonConfig {
                width: Val::Percent(100.0),
                height: Val::Px(40.0),
                idle: BackgroundColor::from(Color::NONE),
                hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
            },
        )
        .style()
        .focus_policy(FocusPolicy::Pass)
        .justify_content(JustifyContent::FlexStart);
}

fn option_input_handle(
    mut dialog_variant_source: ResMut<SelectedVariantsSource>,
    query: Query<(&DialogId)>,
    mut game_state: ResMut<NextState<GameState>>,
    mut sticks_query: Query<&mut Sticks>,
    dialog_query: Query<&Dialog>,
    mut replica_query: Query<&mut CurrentReplica>,
    branching_query: Query<&CurrentBranching>,
    mut button_query: Query<
        (&TextButton<OptionId>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (button, interaction, mut background_color) in &mut button_query {
        match *interaction {
            Interaction::None => {
                *background_color = button.config.idle;
            }
            Interaction::Hovered => {
                *background_color = button.config.hover;
            }
            Interaction::Pressed => {
                let mut stack = &mut sticks_query.single_mut().0;
                if stack.is_empty() {
                    return;
                }
                let last_idx = stack.len() - 1;
                if button.payload.0 == BTN_NEXT_ID {
                    let (id, mut pos) = stack.last().expect("No value in stack");
                    let stick = dialog_query.single().get_stick_at(*id);
                    if pos + 1 < stick.replicas_size() {
                        pos += 1;
                        stack[last_idx].1 = pos;

                        replica_query.single_mut().0 = stick.get_replica_at(pos).text.clone();
                    }

                    if pos == stick.replicas_size() - 1 {
                        let branching = stick.get_branching().clone();
                        if branching.is_none() {
                            stack.pop();
                        }
                    }
                    if stack.is_empty() {
                        game_state.set(GameState::Exploration);
                    }
                    return;
                }
                match &branching_query.single().0 {
                    None => {}
                    Some(branching) => {
                        let selected = &branching.variants[button.payload.0];
                        match &selected.effect {
                            None => {}
                            Some(effect) => {
                                match effect {
                                    DialogEffect::ReplaceDialog => {
                                        stack.pop();
                                    }
                                    DialogEffect::EndDialog(end_id) => {
                                        let dialog_id = query.single();
                                        if let Some(id) = end_id {
                                            dialog_variant_source.produce(dialog_id.0, *id);
                                        }
                                        stack.clear();
                                    }
                                }
                            }
                        }
                        let stick = dialog_query.single().get_stick_at(selected.stick_id);
                        replica_query.single_mut().0 = stick.first_replica().text.clone();
                        stack.push((selected.stick_id, 0));
                    }
                }
            }
        }
    }
}

fn dialog_options_updates(
    sticks_query: Query<&Sticks, Changed<Sticks>>,
    dialog_query: Query<&Dialog>,
    mut branching_query: Query<&mut CurrentBranching>,
) {
    for sticks in sticks_query.iter() {
        let stack = &sticks.0;
        if stack.is_empty() {
            return;
        }
        let (id, pos) = stack.last().expect("No value in stack");
        let stick = dialog_query.single().get_stick_at(*id);

        let new_branching = if *pos == stick.replicas_size() - 1 {
            stick.get_branching().clone()
        } else {
            None
        };

        branching_query.single_mut().0 = new_branching
    }
}

fn current_replica_updates(
    mut commands: Commands,
    mut replica_query: Query<(&Children, &CurrentReplica), Changed<CurrentReplica>>,
) {
    for (children, replica) in replica_query.iter_mut() {
        for &child in children.iter() {
            match commands.get_entity(child) {
                None => { warn!("Current replica component is not found") }
                Some(mut entity_commands) => {
                    entity_commands.update_text(replica.0.clone());
                }
            }
        }
    }
}

fn unspawn(
    mut commands: Commands,
    options_query: Query<Entity, With<DialogOptions>>,
    panel_query: Query<Entity, With<DialogSceneScreen>>,
) {
    for entity in options_query.iter() {
        commands.entity(entity).despawn_recursive();
    }

    for entity in panel_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.50, 0.50, 0.50, 0.7);
const BTN_NEXT_ID: usize = 999;
