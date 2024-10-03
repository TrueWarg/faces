use bevy::app::{App, Plugin};
use bevy::asset::AssetServer;
use bevy::color::Color;
use bevy::color::palettes::css::{ANTIQUE_WHITE, DIM_GREY, WHITE};
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::AlignItems;
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
use sickle_ui::prelude::{ScrollAxis, SetBorderColorExt, SetBorderExt};
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

use crate::core::states::GameState;
use crate::dialog::{Branching, DialogId, DialogsStorage, DialogStick};
use crate::gui::{ButtonConfig, TextButtonExt, TextConfig, TextExt};

pub struct DialogScene;

#[derive(Component)]
pub struct DialogSceneScreen;

#[derive(Component)]
pub struct DialogOptions;

#[derive(Component)]
pub struct Sticks(Vec<(usize, DialogStick)>);

#[derive(Component)]
pub struct CurrentBranching(Option<Branching>);

#[derive(Component)]
pub struct LastPhrase(String);

#[derive(Component)]
pub struct VariantId(usize);

impl Plugin for DialogScene {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Dialog), spawn_main)
            .add_systems(Update, spawn_dialog_options_panel)
            .add_systems(OnExit(GameState::Dialog), unspawn);
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
    let current_branching = CurrentBranching(dialog.root.get_branching().clone());
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent
                .container(NodeBundle::default(), |_| {})
                .style()
                .background_color(Color::from(WHITE))
                .height(Val::Percent(60.0))
                .width(Val::Percent(100.0));
            parent
                .column(|parent| {
                    parent
                        .configure_text(
                            "NEXT",
                            TextConfig::small(Color::from(ANTIQUE_WHITE)))
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
            dialog,
            current_branching,
        )
        )
        .style()
        .justify_content(JustifyContent::Start)
        .align_items(AlignItems::Start)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center);
}

fn spawn_dialog_options_panel(
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
            VariantId(id),
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
