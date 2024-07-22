use bevy::app::Update;
use bevy::asset::{AssetServer, Handle};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::color::palettes::css::DIM_GREY;
use bevy::color::palettes::css::OLIVE;
use bevy::color::palettes::css::SILVER;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::prelude::{AppExtStates, BackgroundColor, Changed, ChildBuilder, Color, Commands, Component, Entity, Font, in_state, Interaction, IntoSystemConfigs, NextState, OnEnter, OnExit, Plugin, Query, Res, ResMut, SpriteBundle, States, With};
use crate::fight::{Enemy, FightId, FightStorage};

use crate::gui::{Button, ButtonId, Container, Root};
use crate::party::{PartyMember, PartyStateStorage};

pub struct FightingScene<S: States> {
    pub state: S,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ScreenState {
    #[default]
    Main,
    AttackList,
    AbilitiesList,
    ItemsList,
    Selection,
    MoveApply,
}

#[derive(Component)]
struct FontHandle {
    font: Handle<Font>,
}

#[derive(Component)]
struct FightingMainScreen;

#[derive(Component)]
struct AttacksScreen;

#[derive(Component)]
struct AbilitiesScreen;

#[derive(Component)]
struct ItemsScreen;

struct Environment {
    id: String,
}

#[derive(Component)]
struct Enemies {
    items: Vec<Enemy>,
}

#[derive(Component)]
struct Allies {
    items: Vec<PartyMember>,
}

struct Attack {
    id: String,
}

struct Ability {
    id: String,
}

struct Item {
    id: String,
}

impl<S: States> Plugin for FightingScene<S> {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_state::<ScreenState>()
            .add_systems(OnEnter(self.state.clone()),
                         (load_alias,
                          load_fight,
                         ).before(spawn_main),
            )
            .add_systems(OnEnter(self.state.clone()), spawn_main)
            .add_systems(OnExit(self.state.clone()), unspawn_main)
            .add_systems(Update, (mouse_input_handle)
                .run_if(in_state(self.state.clone())),
            );
    }
}

fn mouse_input_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&ButtonId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ButtonId>),
    >,
) {
    for (button_id, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = DIM_GREY.into();
            }
            Interaction::Hovered => {
                *background_color = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {}
        }
    }
}

fn spawn_main(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    enemies_query: Query<(&Enemies)>,
    ally_query: Query<(&Allies)>,
) {
    let mut root = Root::default();

    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load("background/test_bg.png"),
    //     ..Default::default()
    // });

    let mut main_container = Container::default();
    main_container.align_start();

    root.spawn(&mut commands, FightingMainScreen, |parent| {
        main_container.spawn(parent, |parent| {
            spawn_fight_area(parent, 70.0, &asset_server, enemies_query.single());
            spawn_player_menu(parent, 30.0, &asset_server, ally_query.single());
        })
    })
}

fn spawn_fight_area(
    parent: &mut ChildBuilder,
    height_percent: f32,
    asset_server: &Res<AssetServer>,
    enemies: &Enemies,
) {
    let mut main_container = Container::size_percentage(100.0, height_percent);
    main_container.row()
        .justify_around();
    main_container.spawn(parent, |parent| {
        for enemy in &enemies.items {
            spawn_enemy_item(parent, asset_server, enemy);
        }
    });
}

fn spawn_enemy_item(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    enemy: &Enemy,
) {
    let mut main_container = Container::size_percentage(20.0, 80.0);
    main_container
        .background_color(Color::from(OLIVE))
        .margin(25.0);
    main_container.spawn(parent, |parent| {});
}

fn spawn_player_menu(
    parent: &mut ChildBuilder,
    height_percent: f32,
    asset_server: &Res<AssetServer>,
    allies: &Allies,
) {
    let mut main_container = Container::size_percentage(100.0, height_percent);
    main_container.row()
        .align_start()
        .justify_start();

    let mut allies_container = Container::size_percentage(75.0, 100.0);
    allies_container.row()
        .background_color(Color::from(DIM_GREY))
        .align_start()
        .justify_start();

    let mut actions_container = Container::size_percentage(25.0, 100.0);
    actions_container
        .background_color(Color::from(SILVER))
        .justify_between();

    main_container.spawn(parent, |parent| {
        allies_container.spawn(parent, |parent| {
            for item in &allies.items {
                spawn_ally_item(parent, asset_server, item);
            }
        });
        actions_container.spawn(parent, |parent| {
            spawn_actions(parent, asset_server);
        });
    });
}

fn spawn_ally_item(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    enemy: &PartyMember,
) {
    let mut main_container = Container::size_percentage(20.0, 80.0);
    main_container
        .background_color(Color::from(ANTIQUE_WHITE))
        .margin(25.0);
    main_container.spawn(parent, |parent| {

    });
}

fn spawn_actions(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    let font = asset_server.load("fonts/quattrocentoSans-Bold.ttf");

    let mut attacks_button = Button::new("Attack", &font);
    attacks_button.id(ATTACKS_BUTTON_ID)
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut protect_button = Button::new("Protect", &font);
    protect_button.id(PROTECT_BUTTON_ID)
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut abilities_button = Button::new("Abilities", &font);
    abilities_button.id(ABILITIES_BUTTON_ID)
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut items_button = Button::new("Items", &font);
    items_button.id(ITEMS_BUTTON_ID)
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    attacks_button.spawn(parent);
    protect_button.spawn(parent);
    abilities_button.spawn(parent);
    items_button.spawn(parent);
}

fn unspawn_main(
    mut commands: Commands,
    query: Query<Entity, With<FightingMainScreen>>,
) {
    let entity = query.single();
    commands.entity(entity).despawn_recursive();
}

fn load_fight(
    mut commands: Commands,
    query: Query<(&FightId)>,
    fight_storage: Res<FightStorage>,
    asset_server: Res<AssetServer>,
) {
    let fight_id = query.single().0;
    let fight = fight_storage.load(fight_id).expect("");

    commands.spawn(SpriteBundle {
        texture: asset_server.load(&fight.arena_bg_path),
        ..Default::default()
    });

    commands.spawn_empty()
        .insert(FightingMainScreen)
        .insert(
            Enemies {
                items: fight.enemies,
            },
        );
}

fn load_alias(
    mut commands: Commands,
    party_storage: Res<PartyStateStorage>,
) {
    let members = party_storage.get_fight_party_members();
    commands.spawn_empty()
        .insert(FightingMainScreen)
        .insert(
            Allies { items: members },
        );
}

const ATTACKS_BUTTON_ID: ButtonId = ButtonId { value: 0 };
const PROTECT_BUTTON_ID: ButtonId = ButtonId { value: 1 };
const ABILITIES_BUTTON_ID: ButtonId = ButtonId { value: 2 };
const ITEMS_BUTTON_ID: ButtonId = ButtonId { value: 3 };

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgb(0.50, 0.50, 0.50);
