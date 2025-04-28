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
use bevy::log::warn;
use bevy::prelude::default;
use bevy::prelude::in_state;
use bevy::prelude::AppExtStates;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Color;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::ImageBundle;
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
use bevy::prelude::{AlignItems, PositionType};
use bevy::text::Text;
use bevy::ui::{UiRect, Val};
use bevy::utils::{warn, HashMap, HashSet};
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiContainerExt;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::prelude::{SetAlignItemsExt, SetLeftExt, SetPositionTypeExt, SetTopExt};
use sickle_ui::ui_builder::{UiBuilder, UiBuilderExt, UiRoot};
use std::fmt::format;

use crate::core::states::GameState;
use crate::fight::actions_ui::{ActionId, ActionItemExt};
use crate::fight::enemy_ui::{EnemyId, EnemyItemExt};
use crate::fight::party_member_ui::{Health, MemberId, PartyMemberItemExt};
use crate::fight::selector_ui::{pick_item_handle, SelectedItemPosHolder, SelectorExt};
use crate::fight::step::decide_next_step;
use crate::fight::{ActionTarget, Enemy, Fight, FightId, FightStorage, GetActionTarget};
use crate::gui::{GetSelectorItem, TextButton};
use crate::party::{PartyMember, PartyStateStorage};
use crate::rpg::{Ability, AttackResult, ConsumableItem, DirectionalAttack, TargetProps};

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
    EnemyStepApply,
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
struct SelectedMemberId(Option<usize>);

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
    all: HashSet<usize>,
    remaining: HashSet<usize>,
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
struct EnemyAttacks {
    items: HashMap<usize, Vec<DirectionalAttack>>,
}

#[derive(Component, Debug)]
struct CurrentAllyStep(Option<AllyStep>);

impl CurrentAllyStep {
    fn set_target_id(&mut self, id: usize) {
        match &mut self.0 {
            None => {}
            Some(step) => match step {
                AllyStep::OnEnemy {
                    action,
                    member_id,
                    target_id,
                } => {
                    *target_id = Some(id);
                }
                AllyStep::OnAlly {
                    action,
                    member_id,
                    target_id,
                } => *target_id = Some(id),
                AllyStep::Guard => {}
            },
        }
    }
}

#[derive(Component, Debug)]
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

#[derive(Component, Debug)]
enum StepAction {
    Attack(DirectionalAttack),
    Ability(Ability),
    Consumable(ConsumableItem),
}

#[derive(Debug)]
enum StepActionResult {
    AttackHit,
    AttackMiss,
    AbilitySuccess,
    ConsumableSuccess,
    TargetDefeated(usize),
}

impl Plugin for FightingScene {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_state::<ScreenState>()
            .add_systems(OnEnter(GameState::Fighting), spawn_main)
            .add_systems(OnExit(GameState::Fighting), unspawn::<FightingMainScreen>)
            .add_systems(
                Update,
                keyboard_input_handle.run_if(in_state(GameState::Fighting)),
            )
            .add_systems(
                Update,
                (
                    party_state_changes,
                    actions_menu_input_handle,
                    party_member_selection_input_handle,
                    party_member_selection_state_changes,
                )
                    .run_if(in_state(ScreenState::Main)),
            )
            .add_systems(OnEnter(ScreenState::AttacksList), spawn_attacks_list)
            .add_systems(OnExit(ScreenState::AttacksList), unspawn::<AttacksScreen>)
            .add_systems(
                Update,
                (selected_attacks_handle, pick_item_handle::<AttacksScreen>)
                    .run_if(in_state(ScreenState::AttacksList)),
            )
            .add_systems(OnEnter(ScreenState::AbilitiesList), spawn_abilities_list)
            .add_systems(
                OnExit(ScreenState::AbilitiesList),
                unspawn::<AbilitiesScreen>,
            )
            .add_systems(
                Update,
                (selected_ability_handle, pick_item_handle::<AbilitiesScreen>)
                    .run_if(in_state(ScreenState::AbilitiesList)),
            )
            .add_systems(OnEnter(ScreenState::ItemsList), spawn_items_list)
            .add_systems(OnExit(ScreenState::ItemsList), unspawn::<ItemsScreen>)
            .add_systems(
                Update,
                (selected_consumable_handle, pick_item_handle::<ItemsScreen>)
                    .run_if(in_state(ScreenState::ItemsList)),
            )
            .add_systems(
                Update,
                ally_step_handle.run_if(in_state(ScreenState::PlayerStepApply)),
            )
            .add_systems(
                Update,
                enemy_step_handle.run_if(in_state(ScreenState::EnemyStep)),
            )
            .add_systems(
                Update,
                target_enemy_selection_input_handle
                    .run_if(in_state(ScreenState::SelectEnemyTarget)),
            )
            .add_systems(
                Update,
                target_ally_selection_input_handle.run_if(in_state(ScreenState::SelectAllyTarget)),
            );
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
            Interaction::None => *background_color = button.config.idle,
            Interaction::Hovered => *background_color = button.config.hover,
            Interaction::Pressed => {
                *background_color = button.config.idle;
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
    query: Query<(&MemberId, &Interaction), (Changed<Interaction>, With<MemberId>)>,
    mut selected_member_query: Query<(&mut SelectedMemberId)>,
    available_members_query: Query<(&AvailableMembers)>,
) {
    for (member_id, interaction) in &query {
        let members = available_members_query.single();
        if !members.remaining.contains(&member_id.0) {
            return;
        }
        match interaction {
            Interaction::None => {}
            Interaction::Hovered => {}
            Interaction::Pressed => {
                let mut new_id = selected_member_query.single_mut();
                *new_id = SelectedMemberId(Some((*member_id).0));
            }
        }
    }
}

fn party_member_selection_state_changes(
    mut query: Query<(&MemberId, &mut BackgroundColor), With<MemberId>>,
    selected_member_query: Query<(&SelectedMemberId), Changed<SelectedMemberId>>,
) {
    for selected_member_id in selected_member_query.iter() {
        for (member_id, mut background) in &mut query {
            if let Some(id) = selected_member_id.0 {
                if member_id.0 == id {
                    *background = YELLOW.into();
                } else {
                    *background = ANTIQUE_WHITE.into();
                }
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
                *background = ANTIQUE_WHITE.into();
                let mut step = current_step_query.single_mut();
                step.set_target_id(id.0);
                next_state.set(ScreenState::PlayerStepApply);
            }
        }
    }
}

fn ally_step_handle(
    mut commands: Commands,
    mut next_state: ResMut<NextState<ScreenState>>,
    mut selected_member_query: Query<(&mut SelectedMemberId)>,
    mut current_step_query: Query<(&mut CurrentAllyStep)>,
    mut available_members_query: Query<(&mut AvailableMembers)>,
    mut allies_targets_query: Query<(&mut AllyTargets)>,
    mut enemies_targets_query: Query<(&mut EnemyTargets)>,
    mut enemies_attacks_query: Query<(&mut EnemyAttacks)>,
    enemies_query: Query<(Entity, &EnemyId)>,
) {
    for curr_step in current_step_query.iter() {
        match &curr_step.0 {
            None => {
                warn!("Step is empty!")
            }
            Some(step) => {
                println!("!!! step = {:?}", step);
                let mut selected_member = selected_member_query.single_mut();
                if selected_member.0.is_none() {
                    panic!("No selected member found")
                }
                let mut allies = allies_targets_query.single_mut();
                let mut enemies = enemies_targets_query.single_mut();
                let mut attacks = enemies_attacks_query.single_mut();
                let result = apply_step(step, &mut allies.items, &mut enemies.items);
                handle_ally_action_result(
                    &result,
                    &mut commands,
                    &enemies_query,
                    &mut enemies.items,
                    &mut attacks.items,
                );
                println!("!!! action step result = {:?}", result);
                println!("!!! Current allies = {:?}", &allies.items);
                println!("!!! Current enemies = {:?}", &enemies.items);

                let mut available_members = available_members_query.single_mut();
                available_members
                    .remaining
                    .remove(&selected_member.0.unwrap());
                *selected_member = SelectedMemberId(None);
                if available_members.remaining.is_empty() {
                    next_state.set(ScreenState::EnemyStep);
                } else {
                    next_state.set(ScreenState::Main);
                };
            }
        }
    }
}

fn handle_ally_action_result(
    step_action_result: &StepActionResult,
    commands: &mut Commands,
    enemies_query: &Query<(Entity, &EnemyId)>,
    enemies_targets: &mut HashMap<usize, TargetProps>,
    enemies_attacks: &mut HashMap<usize, Vec<DirectionalAttack>>,
) {
    match step_action_result {
        StepActionResult::AttackHit => {}
        StepActionResult::AttackMiss => {}
        StepActionResult::AbilitySuccess => {}
        StepActionResult::ConsumableSuccess => {}
        StepActionResult::TargetDefeated(target_id) => {
            for (entity, enemy_id) in enemies_query {
                if enemy_id.0 == *target_id {
                    commands.entity(entity).despawn_recursive();
                    enemies_targets.remove(target_id);
                    enemies_attacks.remove(target_id);
                }
            }
        }
    }
}

fn apply_step(
    step: &AllyStep,
    allies: &mut HashMap<usize, TargetProps>,
    enemies: &mut HashMap<usize, TargetProps>,
) -> StepActionResult {
    match step {
        AllyStep::OnEnemy {
            action,
            member_id,
            target_id,
        } => {
            let id = target_id.expect("target_id must be set");
            let target = enemies
                .get_mut(&id)
                .expect(&format!("No target with {:?} found", id));
            let result = apply_action(action, target);
            if target.is_defeated() {
                StepActionResult::TargetDefeated(id)
            } else {
                result
            }
        }
        AllyStep::OnAlly {
            action,
            member_id,
            target_id,
        } => {
            let id = target_id.expect("target_id must be set");
            let actor = allies
                .get_mut(member_id)
                .expect(&format!("No ally with {:?} found", member_id));
            apply_cost(action, actor);
            let target = allies
                .get_mut(&id)
                .expect(&format!("No target with {:?} found", id));
            apply_action(action, target)
        }
        AllyStep::Guard => {
            panic!("Not implemented yet")
        }
    }
}

fn apply_action(action: &StepAction, target: &mut TargetProps) -> StepActionResult {
    match action {
        StepAction::Attack(attack) => apply_attack(attack, target),
        StepAction::Ability(ability) => {
            ability.apply(target);
            StepActionResult::AbilitySuccess
        }
        StepAction::Consumable(consumable) => {
            consumable.apply(target);
            StepActionResult::ConsumableSuccess
        }
    }
}

fn apply_attack(attack: &DirectionalAttack, target: &mut TargetProps) -> StepActionResult {
    return match attack.apply(target) {
        AttackResult::Hit => StepActionResult::AttackHit,
        AttackResult::Miss => StepActionResult::AttackMiss,
    };
}

fn apply_cost(action: &StepAction, target: &mut TargetProps) {
    match action {
        StepAction::Ability(ability) => ability.apply_cost(target),
        _ => {}
    }
}

fn enemy_step_handle(
    mut next_state: ResMut<NextState<ScreenState>>,
    mut available_members_query: Query<(&mut AvailableMembers)>,
    mut allies_targets_query: Query<(&mut AllyTargets)>,
    mut enemy_attacks_query: Query<(&mut EnemyAttacks)>,
) {
    let mut targets = allies_targets_query.single_mut();
    let mut available_members = available_members_query.single_mut();
    let ids_to_attacks = enemy_attacks_query.single();
    for (id, attacks) in &ids_to_attacks.items {
        println!("Enemy {:?} step", id);
        println!("Enemy {:?} attacks {:?}", id, &attacks);

        let decision = decide_next_step(&attacks, &targets.items);
        println!("Enemy {:?} decision {:?}", id, decision);

        let mut target = targets.items.get_mut(&decision.target_id).expect(&format!(
            "No target with id = {:?} found",
            decision.target_id
        ));
        let attack = &attacks[decision.attack_id];
        let mut result = apply_attack(attack, target);
        if target.is_defeated() {
            result = StepActionResult::TargetDefeated(decision.target_id);
        }
        handle_enemy_action_result(&result, &mut available_members, &mut targets.items);
        println!("!!! result {:?}", result)
    }

    for (_, target) in &targets.items {
        println!("!!! target now: {:?}", target)
    }

    let all = &available_members.all;
    available_members.remaining = all.clone();
    next_state.set(ScreenState::Main);
}

fn handle_enemy_action_result(
    result: &StepActionResult,
    available_members: &mut AvailableMembers,
    allies_targets: &mut HashMap<usize, TargetProps>,
) {
    match result {
        StepActionResult::AttackHit => {}
        StepActionResult::AttackMiss => {}
        StepActionResult::AbilitySuccess => {}
        StepActionResult::ConsumableSuccess => {}
        StepActionResult::TargetDefeated(target_id) => {
            allies_targets.remove(target_id);
            available_members.all.remove(target_id);
        }
    }
}

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
            Interaction::None => *background = Color::NONE.into(),
            Interaction::Hovered => {
                *background = HOVER_BUTTON_COLOR.into();
            }
            Interaction::Pressed => {
                *background = Color::NONE.into();
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
    for selected_member in selected_member_query.iter() {
        if let Some(id) = selected_member.0 {
            let attacks = &attacks_query.single().items[&id];
            let items = attacks
                .iter()
                .map(|attack| attack.selector_item())
                .collect();

            commands
                .ui_builder(UiRoot)
                .selector(items)
                .insert(AttacksScreen)
                .style()
                .size(Val::Percent(100.0));
        }
    }
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
                for selected_member in selected_member_query.iter() {
                    if selected_member.0.is_none() {
                        return;
                    }
                    let selected_member_id = selected_member.0.unwrap();
                    let attacks = &attacks_query.single().items[&selected_member_id];
                    let attack = &attacks[value];
                    let mut current_step = current_step_query.single_mut();
                    current_step.0 = Some(AllyStep::OnEnemy {
                        action: StepAction::Attack(attack.clone()),
                        member_id: selected_member_id,
                        target_id: None,
                    });
                    next_state.set(ScreenState::SelectEnemyTarget);
                }
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
                for selected_member in selected_member_query.iter() {
                    if selected_member.0.is_none() {
                        return;
                    }
                    let selected_member_id = selected_member.0.unwrap();
                    let abilities = &abilities_query.single().items[&selected_member_id];
                    let ability = &abilities[value];
                    let mut current_step = current_step_query.single_mut();
                    let target_direction = ability.action_target();
                    match target_direction {
                        ActionTarget::Enemy => {
                            current_step.0 = Some(AllyStep::OnEnemy {
                                action: StepAction::Ability(ability.clone()),
                                member_id: selected_member_id,
                                target_id: None,
                            });
                            next_state.set(ScreenState::SelectEnemyTarget);
                        }
                        ActionTarget::Ally => {
                            current_step.0 = Some(AllyStep::OnAlly {
                                action: StepAction::Ability(ability.clone()),
                                member_id: selected_member_id,
                                target_id: None,
                            });
                            next_state.set(ScreenState::SelectAllyTarget);
                        }
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
                for selected_member in selected_member_query.iter() {
                    if selected_member.0.is_none() {
                        return;
                    }
                    let selected_member_id = selected_member.0.unwrap();
                    let items = &consumables_query.single().items;
                    let item = &items[value];
                    let mut current_step = current_step_query.single_mut();
                    current_step.0 = Some(AllyStep::OnAlly {
                        action: StepAction::Consumable(item.clone()),
                        member_id: selected_member_id,
                        target_id: None,
                    });
                    next_state.set(ScreenState::SelectAllyTarget);
                }
            }
        }
    }
}

fn spawn_abilities_list(
    mut commands: Commands,
    selected_member_query: Query<(&SelectedMemberId)>,
    abilities_query: Query<(&Abilities)>,
) {
    for selected_member in selected_member_query.iter() {
        if selected_member.0.is_none() {
            return;
        }
        let selected_member_id = selected_member.0.unwrap();
        let abilities = &abilities_query.single().items[&selected_member_id];
        let items = abilities
            .iter()
            .map(|ability| ability.selector_item())
            .collect();
        commands
            .ui_builder(UiRoot)
            .selector(items)
            .insert(AbilitiesScreen)
            .style()
            .size(Val::Percent(100.0));
    }
}

fn spawn_items_list(mut commands: Commands, consumables_query: Query<(&Consumables)>) {
    let consumables = &consumables_query.single().items;
    let items = consumables
        .iter()
        .map(|consumable| consumable.selector_item())
        .collect();
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
    let members = party_storage.get_party_members();
    let ids: HashSet<usize> = members.iter().map(|m| m.id).collect();
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
            SelectedMemberId(Some(default_selected)),
            Consumables { items },
            SelectedItemPosHolder::new(),
            CurrentAllyStep(None),
            AvailableMembers {
                all: ids.clone(),
                remaining: ids,
            },
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
    let mut enemy_attacks = HashMap::new();

    parent
        .container(
            ImageBundle {
                image: UiImage {
                    texture: asset_server.load(&fight.arena_bg_path),
                    ..default()
                },
                ..default()
            },
            |parent| {
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
                    enemy_attacks.insert(enemy.id, enemy.attacks);
                }
            },
        )
        .insert(EnemyTargets {
            items: enemy_targets,
        })
        .insert(EnemyAttacks {
            items: enemy_attacks,
        })
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
            parent
                .row(|parent| {
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

fn spawn_actions(parent: &mut UiBuilder<Entity>, width_percent: f32, height_percent: f32) {
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

fn unspawn<M: Component>(mut commands: Commands, query: Query<Entity, With<M>>) {
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
