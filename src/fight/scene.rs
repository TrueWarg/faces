use bevy::app::Update;
use bevy::asset::AssetServer;
use bevy::color::palettes::basic::YELLOW;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::color::palettes::css::BLUE;
use bevy::color::palettes::css::DIM_GREY;
use bevy::color::palettes::css::SILVER;
use bevy::color::Srgba;
use bevy::hierarchy::Children;
use bevy::hierarchy::DespawnRecursiveExt;
use bevy::input::ButtonInput;
use bevy::prelude::{AlignItems, PositionType};
use bevy::prelude::AppExtStates;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::default;
use bevy::prelude::Entity;
use bevy::prelude::ImageBundle;
use bevy::prelude::in_state;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyContent;
use bevy::prelude::KeyCode;
use bevy::prelude::NextState;
use bevy::prelude::OnEnter;
use bevy::prelude::OnExit;
use bevy::prelude::Plugin;
use bevy::prelude::Query;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::UiImage;
use bevy::prelude::With;
use bevy::text::Text;
use bevy::ui::{UiRect, Val};
use bevy::utils::{HashMap, HashSet};
use sickle_ui::prelude::{SetAlignItemsExt, SetLeftExt, SetPositionTypeExt, SetTopExt};
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiContainerExt;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::ui_builder::{UiBuilder, UiBuilderExt, UiRoot};

use crate::core::states::GameState;
use crate::fight::{ActionTarget, Fight, FightId, FightStorage};
use crate::fight::actions_ui::{ActionId, ActionItemExt};
use crate::fight::enemy_ui::{EnemyId, EnemyItemExt};
use crate::fight::mappers::{GetActionTarget, GetSelectorItem};
use crate::fight::party_member_ui::{Health, MemberId, PartyMemberItemExt};
use crate::fight::selector_ui::{pick_item_handle, SelectedItemPosHolder, SelectorExt};
use crate::gui::TextButton;
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
    SelectEnemyTarget,
    SelectAllyTarget,
    PlayerStepApply,
    EnemyStep,
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
pub struct SelectedMemberId(pub usize);

#[derive(Component)]
struct Attacks {
    items: HashMap<usize, Vec<DirectionalAttack>>,
}

#[derive(Component)]
struct Abilities {
    items: HashMap<usize, Vec<Ability>>,
}

#[derive(Component)]
struct Consumables {
    items: Vec<ConsumableItem>,
}

#[derive(Component)]
struct AvailableMembers {
    ids: HashSet<usize>,
}

#[derive(Component)]
struct AllyTargets {
    items: HashMap<usize, TargetProps>,
}

#[derive(Component)]
struct EnemyTargets {
    items: HashMap<usize, TargetProps>,
}

#[derive(Component)]
struct CurrentAllyStep(Option<AllyStep>);

impl CurrentAllyStep {
    fn set_target_id(&mut self, id: usize) {
        match &mut self.0 {
            None => {}
            Some(step) => match step {
                AllyStep::OnEnemy { action, member_id, mut target_id } => {
                    target_id = Some(id);
                }
                AllyStep::OnAlly { action, member_id, mut target_id } => {
                    target_id = Some(id)
                }
                AllyStep::Guard => {}
            },
        }
    }
}

#[derive(Component)]
enum AllyStep {
    OnEnemy {
        action: StepAction,
        member_id: usize,
        target_id: Option<usize>,
    },
    OnAlly {
        action: StepAction,
        member_id: usize,
        target_id: Option<usize>,
    },
    Guard,
}

#[derive(Component)]
enum StepAction {
    Attack(DirectionalAttack),
    Ability(Ability),
    Consumable(ConsumableItem),
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
            .add_systems(Update, (selected_attacks_handle, pick_item_handle::<AttacksScreen>)
                .run_if(in_state(ScreenState::AttacksList)))

            .add_systems(OnEnter(ScreenState::AbilitiesList), spawn_abilities_list)
            .add_systems(OnExit(ScreenState::AbilitiesList), unspawn::<AbilitiesScreen>)
            .add_systems(Update, (selected_ability_handle, pick_item_handle::<AbilitiesScreen>)
                .run_if(in_state(ScreenState::AbilitiesList)))

            .add_systems(OnEnter(ScreenState::ItemsList), spawn_items_list)
            .add_systems(OnExit(ScreenState::ItemsList), unspawn::<ItemsScreen>)
            .add_systems(Update, (selected_consumable_handle, pick_item_handle::<ItemsScreen>)
                .run_if(in_state(ScreenState::ItemsList)))

            .add_systems(Update, target_enemy_selection_input_handle.run_if(in_state(ScreenState::SelectEnemyTarget)))

            .add_systems(Update, target_ally_selection_input_handle.run_if(in_state(ScreenState::SelectAllyTarget)));
    }
}

fn actions_menu_input_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&TextButton<ActionId>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
) {
    for (button, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = button.config.idle
            }
            Interaction::Hovered => {
                *background_color = button.config.hover
            }
            Interaction::Pressed => {
                if button.payload.0 == ATTACKS_BUTTON_ID.0 {
                    next_state.set(ScreenState::AttacksList);
                }

                if button.payload.0 == PROTECT_BUTTON_ID.0 {}

                if button.payload.0 == ABILITIES_BUTTON_ID.0 {
                    next_state.set(ScreenState::AbilitiesList);
                }

                if button.payload.0 == ITEMS_BUTTON_ID.0 {
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
                *new_id = SelectedMemberId((*member_id).0);
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

fn target_ally_selection_input_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&MemberId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MemberId>),
    >,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
) {
    for (id, interaction, mut background) in &mut query {
        match interaction {
            Interaction::None => {
                *background = ANTIQUE_WHITE.into();
            }
            Interaction::Hovered => {
                *background = BLUE.into();
            }
            Interaction::Pressed => {
                let mut step = current_step_query.single_mut();
                step.set_target_id(id.0);
                next_state.set(ScreenState::PlayerStepApply);
            }
        }
    }
}

fn ally_step_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
) {}

fn party_state_changes(
    parent_query: Query<(&PartyMember, &Children), Changed<PartyMember>>,
    mut children_query: Query<(&mut Text), With<Health>>,
) {
    for (member, mut children) in parent_query.iter() {
        for &child in children.iter() {
            let mut health = children_query.get_mut(child).expect("");
            health.sections[1].value = format!("{}", member.target.armor)
        }
    }
}

fn target_enemy_selection_input_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut query: Query<
        (&EnemyId, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EnemyId>),
    >,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
) {
    for (id, interaction, mut background) in &mut query {
        match interaction {
            Interaction::None => {
                *background = Color::NONE.into()
            }
            Interaction::Hovered => {
                *background = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background = HOVER_BUTTON_COLOR.into();
                let mut step = current_step_query.single_mut();
                step.set_target_id(id.0);
                next_state.set(ScreenState::PlayerStepApply);
            }
        }
    }
}

fn spawn_attacks_list(
    mut commands: Commands,
    selected_member_query: Query<(&SelectedMemberId)>,
    attacks_query: Query<(&Attacks)>,
) {
    let selected_member = selected_member_query.single();
    let attacks = &attacks_query.single().items[&selected_member.0];
    let items = attacks.iter().map(|attack| { attack.selector_item() }).collect();

    commands
        .ui_builder(UiRoot)
        .selector(items)
        .insert(AttacksScreen)
        .style()
        .size(Val::Percent(100.0));
}

fn selected_attacks_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    selected_member_query: Query<(&SelectedMemberId)>,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
    attacks_query: Query<(&Attacks)>,
    mut holder_query: Query<(&mut SelectedItemPosHolder), Changed<SelectedItemPosHolder>>,
) {
    for mut holder in holder_query.iter_mut() {
        match holder.take_away() {
            None => {}
            Some(value) => {
                let selected_member = selected_member_query.single();
                let attacks = &attacks_query.single().items[&selected_member.0];
                let attack = &attacks[value];
                let mut current_step = current_step_query.single_mut();
                current_step.0 = Some(AllyStep::OnEnemy {
                    action: StepAction::Attack(attack.clone()),
                    member_id: selected_member.0,
                    target_id: None,
                });
                next_state.set(ScreenState::SelectEnemyTarget);
            }
        }
    }
}

fn selected_ability_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    selected_member_query: Query<(&SelectedMemberId)>,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
    abilities_query: Query<(&Abilities)>,
    mut holder_query: Query<(&mut SelectedItemPosHolder), Changed<SelectedItemPosHolder>>,
) {
    for mut holder in holder_query.iter_mut() {
        match holder.take_away() {
            None => {}
            Some(value) => {
                let selected_member = selected_member_query.single();
                let abilities = &abilities_query.single().items[&selected_member.0];
                let ability = &abilities[value];
                let mut current_step = current_step_query.single_mut();
                let target_direction = ability.action_target();
                match target_direction {
                    ActionTarget::Enemy => {
                        current_step.0 = Some(AllyStep::OnEnemy {
                            action: StepAction::Ability(ability.clone()),
                            member_id: selected_member.0,
                            target_id: None,
                        });
                        next_state.set(ScreenState::SelectEnemyTarget);
                    }
                    ActionTarget::Ally => {
                        current_step.0 = Some(AllyStep::OnAlly {
                            action: StepAction::Ability(ability.clone()),
                            member_id: selected_member.0,
                            target_id: None,
                        });
                        next_state.set(ScreenState::SelectAllyTarget);
                    }
                }
            }
        }
    }
}

fn selected_consumable_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    selected_member_query: Query<(&SelectedMemberId)>,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
    consumables_query: Query<(&Consumables)>,
    mut holder_query: Query<(&mut SelectedItemPosHolder), Changed<SelectedItemPosHolder>>,
) {
    for mut holder in holder_query.iter_mut() {
        match holder.take_away() {
            None => {}
            Some(value) => {
                let selected_member = selected_member_query.single();
                let items = &consumables_query.single().items;
                let item = &items[value];
                let mut current_step = current_step_query.single_mut();
                current_step.0 = Some(AllyStep::OnAlly {
                    action: StepAction::Consumable(item.clone()),
                    member_id: selected_member.0,
                    target_id: None,
                });
                next_state.set(ScreenState::SelectAllyTarget);
            }
        }
    }
}

fn spawn_abilities_list(
    mut commands: Commands,
    selected_member_query: Query<(&SelectedMemberId)>,
    abilities_query: Query<(&Abilities)>,
) {
    let selected_member = selected_member_query.single();
    let abilities = &abilities_query.single().items[&selected_member.0];
    let items = abilities.iter().map(|ability| { ability.selector_item() }).collect();
    commands
        .ui_builder(UiRoot)
        .selector(items)
        .insert(AbilitiesScreen)
        .style()
        .size(Val::Percent(100.0));
}

fn spawn_items_list(
    mut commands: Commands,
    consumables_query: Query<(&Consumables)>,
) {
    let consumables = &consumables_query.single().items;
    let items = consumables.iter().map(|consumable| { consumable.selector_item() }).collect();
    commands
        .ui_builder(UiRoot)
        .selector(items)
        .insert(ItemsScreen)
        .style()
        .size(Val::Percent(100.0));
}

fn spawn_main(
    mut commands: Commands,
    query: Query<(&FightId)>,
    asset_server: Res<AssetServer>,
    fight_storage: Res<FightStorage>,
    party_storage: Res<PartyStateStorage>,
) {
    let fight_id = query.single();
    let fight = fight_storage.load(&fight_id.0).expect("");
    let members = party_storage.get_fight_party_members();
    let items = party_storage.get_consumables();

    let default_selected = members.first().expect("Members should not be empty").id;
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            spawn_fight_area(parent, 70.0, &asset_server, fight);
            spawn_player_menu(parent, 30.0, &asset_server, members);
        })
        .insert((
            FightingMainScreen,
            SelectedMemberId(default_selected),
            Consumables { items },
            SelectedItemPosHolder::new(),
            CurrentAllyStep(None),
            // AvailableMembers { ids: HashSet::new() },
        ))
        .style()
        .justify_content(JustifyContent::Center)
        .size(Val::Percent(100.0))
        .align_items(AlignItems::Center);
}

fn spawn_fight_area(
    parent: &mut UiBuilder<Entity>,
    height_percent: f32,
    asset_server: &Res<AssetServer>,
    fight: Fight,
) {
    let mut enemy_targets = HashMap::new();

    parent.container(ImageBundle {
        image: UiImage {
            texture: asset_server.load(&fight.arena_bg_path),
            ..default()
        },
        ..default()
    }, |parent| {
        for enemy in fight.enemies {
            parent
                .enemy_item(EnemyId(enemy.id), asset_server.load(enemy.asset_path))
                .style()
                .width(Val::Auto)
                .height(Val::Percent(enemy.relative_height))
                .position_type(PositionType::Absolute)
                .left(Val::Percent(enemy.relative_x))
                .top(Val::Percent(enemy.relative_y));
            enemy_targets.insert(enemy.id, enemy.target);
        }
    })
        .insert(EnemyTargets { items: enemy_targets })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(height_percent));
}

fn spawn_player_menu(
    parent: &mut UiBuilder<Entity>,
    height_percent: f32,
    asset_server: &Res<AssetServer>,
    members: Vec<PartyMember>,
) {
    parent
        .row(|parent| {
            let mut attacks = HashMap::new();
            let mut abilities = HashMap::new();
            let mut targets = HashMap::new();
            parent.row(|parent| {
                for item in members {
                    attacks.insert(item.id, item.attacks);
                    abilities.insert(item.id, item.abilities);
                    targets.insert(item.id, item.target);
                    parent
                        .party_member_item(MemberId(item.id))
                        .style()
                        .margin(UiRect {
                            left: Val::Px(25.0),
                            right: Val::Px(25.0),
                            top: Val::Px(25.0),
                            bottom: Val::Px(25.0),
                        })
                        .width(Val::Percent(20.0))
                        .height(Val::Percent(80.0));
                }
            })
                .insert(Attacks { items: attacks })
                .insert(Abilities { items: abilities })
                .insert(AllyTargets { items: targets })
                .style()
                .background_color(Color::from(DIM_GREY))
                .justify_content(JustifyContent::FlexStart)
                .align_items(AlignItems::FlexStart)
                .width(Val::Percent(75.0))
                .height(Val::Percent(100.0));

            spawn_actions(parent, 25.0, 100.0);
        })
        .style()
        .justify_content(JustifyContent::FlexStart)
        .align_items(AlignItems::FlexStart)
        .width(Val::Percent(100.0))
        .height(Val::Percent(height_percent));
}

fn spawn_actions(
    parent: &mut UiBuilder<Entity>,
    width_percent: f32,
    height_percent: f32,
) {
    parent
        .column(|parent| {
            parent.action_item(ATTACKS_BUTTON_ID, "Attacks");
            parent.action_item(PROTECT_BUTTON_ID, "Protect");
            parent.action_item(ABILITIES_BUTTON_ID, "Abilities");
            parent.action_item(ITEMS_BUTTON_ID, "Items");
        })
        .style()
        .width(Val::Percent(width_percent))
        .height(Val::Percent(height_percent))
        .background_color(Color::from(SILVER))
        .justify_content(JustifyContent::SpaceBetween)
        .align_items(AlignItems::Center);
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
const HOVER_BUTTON_COLOR: Srgba = Srgba::new(0.302, 0.302, 0.302, 0.7);
