use bevy::app::{App, Plugin};
use bevy::asset::{Assets, AssetServer, Handle};
use bevy::hierarchy::{BuildChildren, Children, DespawnRecursiveExt};
use bevy::math::Vec2;
use bevy::prelude::{AlignItems, AlignSelf, Button, ButtonBundle, Camera2dBundle, Changed, Color, Commands, Component, Entity, Font, Image, ImageBundle, in_state, Interaction, IntoSystemConfigs, JustifyContent, NextState, OnEnter, OnExit, Query, Res, ResMut, Resource, Style, Text, TextBundle, TextStyle, UiImage, Update, Val, With};
use bevy::sprite::TextureAtlasLayout;
use bevy::ui::FocusPolicy;

use crate::core::states::GameState;

pub struct MainMenuPlugin;

#[derive(Component)]
pub struct ButtonActive(bool);

#[derive(Resource)]
struct UiAssets {
    font: Handle<Font>,
    button: Handle<Image>,
    button_pressed: Handle<Image>,
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::MainMenu), setup_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
            .add_systems(Update, handle_start_button.run_if(in_state(GameState::MainMenu)));
    }
}

fn despawn_menu(mut commands: Commands, button_query: Query<Entity, With<Button>>) {
    for ent in button_query.iter() {
        commands.entity(ent).despawn_recursive();
    }
}

fn handle_start_button(
    mut interaction_query: Query<
        (&Children, &mut ButtonActive, &Interaction),
        Changed<Interaction>,
    >,
    mut image_query: Query<&mut UiImage>,
    ui_assets: Res<UiAssets>,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (children, mut active, interaction) in interaction_query.iter_mut() {
        let child = children.iter().next().unwrap();
        let mut image = image_query.get_mut(*child).unwrap();

        match interaction {
            Interaction::Pressed => {
                if active.0 {
                    image.texture = ui_assets.button_pressed.clone();
                    active.0 = false;
                    next_game_state.set(GameState::Exporation);
                }
            }
            Interaction::Hovered | Interaction::None => {
                image.texture = ui_assets.button.clone();
            }
        }
    }
}

fn setup_menu(mut commands: Commands, assets: Res<AssetServer>) {
    let ui_assets = UiAssets {
        font: assets.load("fonts/quattrocentoSans-Bold.ttf"),
        button: assets.load("menu/button.png"),
        button_pressed: assets.load("menu/button_pressed.png"),
    };

    // commands.spawn(Camera2dBundle::default());
    commands
        .spawn(ButtonBundle {
            style: Style {
                align_self: AlignSelf::Center,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                min_height: Val::Percent(10.0),
                min_width: Val::Percent(20.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ButtonActive(true))
        .with_children(|parent| {
            parent
                .spawn(ImageBundle {
                    style: Style {
                        min_height: Val::Percent(100.0),
                        min_width: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    image: ui_assets.button.clone().into(),
                    ..Default::default()
                })
                .insert(FocusPolicy::Pass)
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(
                            "Start Game",
                            TextStyle {
                                font: ui_assets.font.clone(),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        ),
                        focus_policy: FocusPolicy::Pass,
                        ..Default::default()
                    });
                });
        });
    commands.insert_resource(ui_assets);
}

