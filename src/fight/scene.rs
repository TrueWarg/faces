use bevy::{
    app::Update,
    asset::{AssetServer, Handle},
    color::palettes::css::ANTIQUE_WHITE,
    color::palettes::css::DIM_GREY,
    color::palettes::css::OLIVE,
    color::palettes::css::SILVER,
    hierarchy::{Children, DespawnRecursiveExt},
    prelude::AppExtStates,
    prelude::BackgroundColor,
    prelude::Changed,
    prelude::ChildBuilder,
    prelude::Color,
    prelude::Commands,
    prelude::Component,
    prelude::Entity,
    prelude::Font,
    prelude::in_state,
    prelude::Interaction,
    prelude::IntoSystemConfigs,
    prelude::NextState,
    prelude::OnEnter,
    prelude::OnExit,
    prelude::Plugin,
    prelude::Query,
    prelude::Res,
    prelude::ResMut,
    prelude::SpriteBundle,
    prelude::States,
    prelude::With,
    text::Text,
};
use bevy::color::palettes::basic::YELLOW;
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, State};
use hashlink::LinkedHashMap;

use crate::core::states::GameState;
use crate::fight::{Enemy, FightId, FightStorage};
use crate::fight::actions_ui::{ActionId, ActionItem};
use crate::fight::party_member_ui::{Health, MemberId, PartyMemberItem};
use crate::fight::selector_ui::{Selector, SelectorItem};
use crate::gui::{Container, Root};
use crate::party::{PartyMember, PartyStateStorage};
use crate::rpg::{Ability, ConsumableItem, DirectionalAttack, TargetProps};

pub struct FightingScene;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum ScreenState {
    #[default]
    Main,
    AttacksList,
    AbilitiesList,
    ItemsList,
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

#[derive(Component)]
pub struct SelectedMemberId(pub String);

#[derive(Component)]
struct Attacks {
    items: LinkedHashMap<String, Vec<DirectionalAttack>>,
}

#[derive(Component)]
struct Abilities {
    items: LinkedHashMap<String, Vec<Ability>>,
}

#[derive(Component)]
struct Consumables {
    items: Vec<ConsumableItem>,
}

impl Plugin for FightingScene {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .init_state::<ScreenState>()

            .add_systems(OnEnter(GameState::Fighting), spawn_main)
            .add_systems(OnExit(GameState::Fighting), unspawn::<FightingMainScreen>)
            .add_systems(Update, keyboard_input_handle.run_if(in_state(GameState::Fighting)))

            .add_systems(Update,
                         (party_state_changes,
                          actions_menu_input_handle,
                          party_member_selection_input_handle,
                          party_member_selection_state_changes,
                         ).run_if(in_state(ScreenState::Main)),
            )

            .add_systems(OnEnter(ScreenState::AttacksList), spawn_attacks_list)
            .add_systems(OnExit(ScreenState::AttacksList), unspawn::<AttacksScreen>)
            .add_systems(Update, attacks_inputs.run_if(in_state(ScreenState::AttacksList)))

            .add_systems(OnEnter(ScreenState::AbilitiesList), spawn_abilities_list)
            .add_systems(OnExit(ScreenState::AbilitiesList), unspawn::<AbilitiesScreen>)
            .add_systems(Update, abilities_inputs.run_if(in_state(ScreenState::AbilitiesList)))

            .add_systems(OnEnter(ScreenState::ItemsList), spawn_items_list)
            .add_systems(OnExit(ScreenState::ItemsList), unspawn::<ItemsScreen>)
            .add_systems(Update, items_inputs.run_if(in_state(ScreenState::ItemsList)));
    }
}

fn actions_menu_input_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&ActionId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<ActionId>),
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
            Interaction::Pressed => {
                if button_id.0 == ATTACKS_BUTTON_ID.0 {
                    next_state.set(ScreenState::AttacksList);
                }


                if button_id.0 == PROTECT_BUTTON_ID.0 {}


                if button_id.0 == ABILITIES_BUTTON_ID.0 {
                    next_state.set(ScreenState::AbilitiesList);
                }


                if button_id.0 == ITEMS_BUTTON_ID.0 {
                    next_state.set(ScreenState::ItemsList);
                }
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

fn party_member_selection_input_handle(
    query: Query<
        (&MemberId, &Interaction),
        (Changed<Interaction>, With<MemberId>),
    >,
    mut selected_member_query: Query<(&mut SelectedMemberId)>,
) {
    for (member_id, interaction) in &query {
        match interaction {
            Interaction::None => {}
            Interaction::Hovered => {}
            Interaction::Pressed => {
                let mut new_id = selected_member_query.single_mut();
                *new_id = SelectedMemberId((*member_id).0.clone());
            }
        }
    }
}

fn party_member_selection_state_changes(
    mut query: Query<(&MemberId, &mut BackgroundColor), With<MemberId>>,
    selected_member_query: Query<(&SelectedMemberId), Changed<SelectedMemberId>>,
) {
    for id in selected_member_query.iter() {
        for (member_id, mut background) in &mut query {
            if member_id.0 == id.0 {
                *background = YELLOW.into();
            } else {
                *background = ANTIQUE_WHITE.into();
            }
        }
        // only one selected member expected here
        break;
    }
}

fn party_state_changes(
    parent_query: Query<(&PartyMember, &Children),
        (Changed<PartyMember>),
    >,
    mut children_query: Query<(&mut Text), (With<Health>),
    >,
) {
    for (member, mut children) in parent_query.iter() {
        for &child in children.iter() {
            let mut health = children_query.get_mut(child).expect("");
            health.sections[1].value = format!("{}", member.target.armor)
        }
    }
}

fn spawn_attacks_list(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    selected_member_query: Query<(&SelectedMemberId)>,
    attacks_query: Query<(&Attacks)>,
) {
    let font = asset_server.load("fonts/quattrocentoSans-Bold.ttf");

    let mut selector = Selector;
    selector.spawn(&mut commands, AttacksScreen, &font, vec![
        SelectorItem {
            name: "Name 1".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 2".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 3".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 4".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 5".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 6".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 7".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 8".to_string(),
            description: "".to_string(),
        },
        SelectorItem {
            name: "Name 9".to_string(),
            description: "".to_string(),
        },
    ]);
}

fn attacks_inputs() {}

fn spawn_abilities_list(
    mut commands: Commands,
    selected_member_query: Query<(&SelectedMemberId)>,
    attacks_query: Query<(&Attacks)>,
) {
    let mut selector = Selector;
    // selector.spawn(&mut commands, AbilitiesScreen);
}

fn abilities_inputs() {}

fn spawn_items_list(
    mut commands: Commands,
    selected_member_query: Query<(&SelectedMemberId)>,
    attacks_query: Query<(&Attacks)>,
) {
    let mut selector = Selector;
    // selector.spawn(&mut commands, ItemsScreen);
}

fn items_inputs() {}

fn spawn_main(
    mut commands: Commands,
    query: Query<(&FightId)>,
    asset_server: Res<AssetServer>,
    fight_storage: Res<FightStorage>,
    party_storage: Res<PartyStateStorage>,
) {
    let mut root = Root::default();

    let fight_id = query.single();
    let fight = fight_storage.load(&fight_id.0).expect("");
    let members = party_storage.get_fight_party_members();
    let items = party_storage.get_consumables();

    let mut main_container = Container::default();
    main_container.align_start();

    commands.spawn(SpriteBundle {
        texture: asset_server.load(&fight.arena_bg_path),
        ..Default::default()
    })
        .insert(FightingMainScreen);

    root.spawn(&mut commands, FightingMainScreen, |parent| {
        let default_selected = members.first().expect("").id.clone();
        parent.spawn(SelectedMemberId(default_selected));
        parent.spawn(Consumables { items });
        main_container.spawn(parent, |parent| {
            spawn_fight_area(parent, 70.0, &asset_server, fight.enemies);
            spawn_player_menu(parent, 30.0, &asset_server, members);
        })
    })
}

fn spawn_fight_area(
    parent: &mut ChildBuilder,
    height_percent: f32,
    asset_server: &Res<AssetServer>,
    enemies: Vec<Enemy>,
) {
    let mut main_container = Container::size_percentage(100.0, height_percent);
    main_container.row().justify_around();
    main_container.spawn(parent, |parent| {
        for enemy in &enemies {
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
    members: Vec<PartyMember>,
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
            let mut attacks = LinkedHashMap::new();
            let mut abilities = LinkedHashMap::new();
            for item in members {
                attacks.insert(item.id.clone(), item.attacks);
                abilities.insert(item.id.clone(), item.abilities);
                spawn_ally_item(parent, asset_server, item.id, item.target);
            }
            parent.spawn(Attacks { items: attacks });
            parent.spawn(Abilities { items: abilities });
        });
        actions_container.spawn(parent, |parent| {
            spawn_actions(parent, asset_server);
        });
    });
}

fn spawn_ally_item(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
    member_id: String,
    props: TargetProps,
) {
    let font = asset_server.load("fonts/quattrocentoSans-Bold.ttf");

    let mut main_container = Container::size_percentage(20.0, 80.0);
    main_container.margin(25.0);
    main_container.spawn_with_payload(parent, props, |parent| {
        let member = PartyMemberItem::new(member_id);
        member.spawn(parent, &font);
    });
}

fn spawn_actions(
    parent: &mut ChildBuilder,
    asset_server: &Res<AssetServer>,
) {
    let font = &asset_server.load("fonts/quattrocentoSans-Bold.ttf");

    let mut attacks_button = ActionItem::new(ATTACKS_BUTTON_ID, "Attacks", font);
    attacks_button
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut protect_button = ActionItem::new(PROTECT_BUTTON_ID, "Protect", font);
    protect_button
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut abilities_button = ActionItem::new(ABILITIES_BUTTON_ID, "Abilities", font);
    abilities_button
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    let mut items_button = ActionItem::new(ITEMS_BUTTON_ID, "Items", font);
    items_button
        .size_percentage(95.0, 20.0)
        .margin(4.0)
        .text_color(Color::from(SILVER));

    attacks_button.spawn(parent);
    protect_button.spawn(parent);
    abilities_button.spawn(parent);
    items_button.spawn(parent);
}

fn unspawn<M: Component>(
    mut commands: Commands,
    query: Query<Entity, With<M>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}


const ATTACKS_BUTTON_ID: ActionId = ActionId(0);
const PROTECT_BUTTON_ID: ActionId = ActionId(1);
const ABILITIES_BUTTON_ID: ActionId = ActionId(2);
const ITEMS_BUTTON_ID: ActionId = ActionId(3);

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_BUTTON_COLOR: Color = Color::srgb(0.50, 0.50, 0.50);
