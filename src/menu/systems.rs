use bevy::app::{App, Plugin};
use bevy::asset::{AssetServer, Handle};
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::AlignItems;
use bevy::prelude::ButtonBundle;
use bevy::prelude::Camera2dBundle;
use bevy::prelude::Changed;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::default;
use bevy::prelude::Entity;
use bevy::prelude::Font;
use bevy::prelude::Image;
use bevy::prelude::in_state;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyText;
use bevy::prelude::KeyCode;
use bevy::prelude::NodeBundle;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::Resource;
use bevy::prelude::SpriteBundle;
use bevy::prelude::Style;
use bevy::prelude::TextBundle;
use bevy::prelude::TextStyle;
use bevy::prelude::Transform;
use bevy::prelude::UiRect;
use bevy::prelude::Update;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::ui::BackgroundColor;

use crate::core::states::GameState;
use crate::core::z_index::FLOOR_Z;
use crate::gui::ScrollableContent;
use crate::gui::RootMarker;
use crate::gui::Root;
use crate::gui::Container;
use crate::gui::ButtonId;
use crate::gui::Button;
use crate::gui::ScrollView;
use crate::gui::Text;
use crate::gui::VerticalScrollPlugin;

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
        app
            .add_plugins(VerticalScrollPlugin)
            .add_systems(OnEnter(GameState::MainMenu), spawn_fight_menu)
            .add_systems(OnExit(GameState::MainMenu), despawn_menu)
            .add_systems(Update, (handle_mouse_input, show_selector).run_if(in_state(GameState::MainMenu)));
    }
}

const CLR_1: Color = Color::rgb(0.168, 0.168, 0.168);
const CLR_2: Color = Color::rgb(0.109, 0.109, 0.109);
const CLR_3: Color = Color::rgb(0.569, 0.592, 0.647);
const CLR_4: Color = Color::rgb(0.902, 0.4, 0.004);

fn prepare(mut commands: Commands, label: SelectorLabel) {
    commands.spawn(Camera2dBundle::default());
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                padding: UiRect::all(Val::Px(15.0)),
                ..default()
            },
            background_color: CLR_1.into(),
            ..default()
        })
        .insert(label)
        .with_children(|p| {
            p.spawn(ButtonBundle {
                style: Style {
                    margin: UiRect::all(Val::Px(15.0)),
                    padding: UiRect::all(Val::Px(15.0)),
                    max_height: Val::Px(100.0),
                    border: UiRect::all(Val::Px(3.0)),
                    align_items: AlignItems::Center,
                    ..default()
                },
                background_color: CLR_2.into(),
                border_color: CLR_4.into(),
                ..default()
            })
                .with_children(|p| {
                    p.spawn(TextBundle::from_section(
                        "Reset scroll",
                        TextStyle {
                            font_size: 25.0,
                            color: CLR_4,
                            ..default()
                        },
                    ));
                });
            p.spawn((
                NodeBundle {
                    style: Style {
                        width: Val::Percent(80.0),
                        margin: UiRect::all(Val::Px(15.0)),
                        ..default()
                    },
                    background_color: CLR_2.into(),
                    ..default()
                },
                ScrollView::default(),
            ))
                .with_children(|p| {
                    p.spawn((
                        NodeBundle {
                            style: Style {
                                flex_direction: bevy::ui::FlexDirection::Column,
                                width: Val::Percent(100.0),
                                ..default()
                            },
                            ..default()
                        },
                        ScrollableContent::default(),
                    ))
                        .with_children(|scroll_area| {
                            for i in 0..21 {
                                scroll_area
                                    .spawn(NodeBundle {
                                        style: Style {
                                            min_width: Val::Px(200.0),
                                            margin: UiRect::all(Val::Px(15.0)),
                                            border: UiRect::all(Val::Px(5.0)),
                                            padding: UiRect::all(Val::Px(30.0)),
                                            ..default()
                                        },
                                        border_color: CLR_3.into(),
                                        ..default()
                                    })
                                    .with_children(|p| {
                                        p.spawn(
                                            TextBundle::from_section(
                                                format!("Nr {}", i),
                                                TextStyle {
                                                    font_size: 25.0,
                                                    color: CLR_3,
                                                    ..default()
                                                },
                                            )
                                                .with_text_justify(JustifyText::Center),
                                        );
                                    });
                            }
                        });
                });
        });
}

fn despawn_menu(mut commands: Commands, root_query: Query<Entity, With<RootMarker>>) {
    let root_entity = root_query.single();
    commands.entity(root_entity).despawn_recursive();
}

#[derive(Component)]
struct SelectorLabel;

fn show_selector(
    mut commands: Commands,
    mut query: Query<(&mut ShowSelector)>,
    label_query: Query<Entity, With<SelectorLabel>>,
) {
    let mut show_selector = query.single_mut();
    if show_selector.display && !show_selector.displayed {
        show_selector.displayed = true;
        prepare(commands, SelectorLabel);
    } else if !show_selector.display && show_selector.displayed {
        show_selector.displayed = false;
        let label = label_query.single();
        commands.entity(label).despawn_recursive();
    }
}

fn spawn_fight_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/quattrocentoSans-Bold.ttf");
    let mut root = Root::default();
    commands.spawn(SpriteBundle {
        texture: asset_server.load("background/test_bg.png"),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, FLOOR_Z),
            ..Default::default()
        },
        ..Default::default()
    });

    commands
        .spawn_empty()
        .insert(ShowSelector { display: false, displayed: false });

    let mut main_container = Container::default();
    main_container.row();
    main_container.align_end();
    let mut fight_menu = Container::size_percentage(100.0, 25.0);
    fight_menu.background_color(Color::WHITE);
    fight_menu.row();
    fight_menu.justify_between();
    fight_menu.content_end();
    let mut actions_menu = Container::size_percentage(25.0, 100.0);
    actions_menu.background_color(Color::DARK_GRAY);
    actions_menu.justify_between();
    // actions_menu.bundle.style.align_self = AlignSelf::End;
    let mut attack = Button::new("Attacks", &font);
    let mut protect = Button::new("Protect", &font);
    let mut abilities = Button::new("Abilities", &font);
    abilities.id(KEK);
    let mut items = Button::new("Items", &font);

    let mut character_menu = Container::size_percentage(75.0, 100.0);
    character_menu.background_color(Color::OLIVE);
    character_menu.row();
    character_menu.justify_start();
    character_menu.content_start();

    let mut char_1 = Container::size_percentage(18.0, 85.0);
    char_1.background_color(Color::BLUE);
    char_1.margin(12.0);

    let mut char_2 = Container::size_percentage(18.0, 85.0);
    char_2.background_color(Color::BLUE);
    char_2.margin(12.0);

    let mut char_3 = Container::size_percentage(18.0, 85.0);
    char_3.background_color(Color::BLUE);
    char_3.margin(12.0);

    let mut char_4 = Container::size_percentage(18.0, 85.0);
    char_4.background_color(Color::BLUE);
    char_4.margin(12.0);

    // let mut enemies = Container::size_percentage(100.0, 75.0);

    root.spawn(&mut commands, |parent| {
        main_container.spawn(parent, |parent| {
            fight_menu.spawn(parent, |parent| {
                character_menu.spawn(parent, |parent| {
                    char_1.spawn(parent, |_| {});
                    char_2.spawn(parent, |_| {});
                    char_3.spawn(parent, |_| {});
                    char_4.spawn(parent, |_| {});
                });
                actions_menu.spawn(parent, |parent| {
                    attack.spawn(parent);
                    protect.spawn(parent);
                    abilities.spawn(parent);
                    items.spawn(parent);
                });
            })
        })
    });
}

#[derive(Component)]
struct ShowSelector {
    display: bool,
    displayed: bool,
}

fn handle_mouse_input(
    mut query: Query<
        (&ButtonId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonId>),
    >,
    mut show_selector_query: Query<(&mut ShowSelector)>,
    keyboard: Res<ButtonInput<KeyCode>>,
) {
    if keyboard.pressed(KeyCode::Escape) && keyboard.just_pressed(KeyCode::Escape) {
        let mut selector = show_selector_query.single_mut();
        selector.display = false;
    }

    for (button_id, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = Color::TEAL.into();
            }
            Interaction::Hovered => {
                *background_color = Color::TURQUOISE.into();
            }
            Interaction::Pressed => {
                if button_id.value == KEK {
                    let mut selector = show_selector_query.single_mut();
                    selector.display = true;
                }
            }
        }
    }
}

static KEK: usize = 2;

