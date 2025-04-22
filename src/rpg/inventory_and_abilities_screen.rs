use bevy::app::{App, Plugin, Update};
use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::hierarchy::{Children, DespawnRecursiveExt};
use bevy::log::warn;
use bevy::prelude::{AppExtStates, OnExit};
use bevy::prelude::AlignItems;
use bevy::prelude::DetectChanges;
use bevy::prelude::Entity;
use bevy::prelude::NextState;
use bevy::prelude::Res;
use bevy::prelude::ResMut;
use bevy::prelude::State;
use bevy::prelude::States;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::in_state;
use bevy::prelude::Interaction;
use bevy::prelude::IntoSystemConfigs;
use bevy::prelude::JustifyContent;
use bevy::prelude::OnEnter;
use bevy::prelude::Query;
use bevy::prelude::UiRect;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::ui::FocusPolicy;
use bevy_rapier2d::na::DimRange;
use sickle_ui::prelude::ScrollAxis;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetFocusPolicyExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiRoot;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::prelude::UiScrollViewExt;
use sickle_ui::ui_builder::UiBuilderExt;
use sickle_ui::ui_commands::UpdateTextExt;
use crate::core::states::GameState;
use crate::gui::ButtonConfig;
use crate::gui::GetSelectorItem;
use crate::gui::SelectorItem;
use crate::gui::TextButton;
use crate::gui::TextButtonExt;
use crate::gui::TextConfig;
use crate::gui::TextExt;
use crate::party::PartyStateStorage;
use crate::rpg::{ConsumableItem, DirectionalAttack};

pub struct InventoryAndAbilityScreenPlugin;

#[derive(Component)]
struct InventoryAndAbilityScreen;

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum Tab {
    #[default]
    Inventory,
    Abilities,
    Attacks,
}

#[derive(Component)]
struct PosAndDescr(pub usize, pub String);

#[derive(Component, Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
struct SelectedMember(pub usize);

#[derive(Component)]
struct SelectedItemPosHolder {
    value: Option<usize>,
}

impl SelectedItemPosHolder {
    fn new() -> Self {
        return SelectedItemPosHolder { value: None };
    }

    fn store(&mut self, value: usize) {
        self.value = Some(value);
    }

    fn take_away_unsafe(&mut self) -> usize {
        let value = self.value.expect("Value was not be stored");
        self.value = None;
        return value;
    }

    fn take_away(&mut self) -> Option<usize> {
        let value = self.value;
        self.value = None;
        return value;
    }
}

#[derive(Component)]
pub struct Description;

impl Plugin for InventoryAndAbilityScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<Tab>()
            .init_state::<SelectedMember>()
            .add_systems(OnEnter(GameState::InventoryAndAbilities), spawn_main)
            .add_systems(OnExit(GameState::InventoryAndAbilities), despawn_main)
            .add_systems(Update, main_respawns.run_if(in_state(GameState::InventoryAndAbilities)))
            .add_systems(Update,
                         (pick_item_handle,
                          pick_tab_handle,
                          pick_member_handle,
                         )
                             .run_if(in_state(GameState::InventoryAndAbilities)),
            );
    }
}

fn spawn_main(
    mut commands: Commands,
) {
    commands
        .spawn_empty()
        .insert(SelectedItemPosHolder::new());
}

fn main_respawns(
    mut commands: Commands,
    party_storage: Res<PartyStateStorage>,
    tab_state: Res<State<Tab>>,
    selected_member_state: Res<State<SelectedMember>>,
    screen_query: Query<Entity, With<InventoryAndAbilityScreen>>,
) {
    if !tab_state.is_changed() && !selected_member_state.is_changed() {
        return;
    }

    for entity in screen_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
    
    let members = party_storage.get_party_members();
    let current_member_id = selected_member_state.get().0;
    let member = &members[current_member_id];
    let items = match tab_state.get() {
        Tab::Inventory => {
            let items = party_storage.get_consumables();
            to_selector_items(&items)
        }
        Tab::Abilities => {
            let items = &member.abilities;
            to_selector_items(&items)
        }
        Tab::Attacks => {
            let items = &member.attacks;
            to_selector_items(&items)
        }
    };
    commands
        .ui_builder(UiRoot)
        .column(|parent| {
            parent.column(|parent| {
                parent.row(|parent| {
                    let config = ButtonConfig {
                        width: Val::Percent(100.0 / 3.0),
                        height: Val::Percent(100.0),
                        idle: BackgroundColor::from(Color::NONE),
                        hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        justify_content: JustifyContent::Center,
                    };
                    parent
                        .configure_text_button(
                            "Лут".to_string(),
                            Tab::Inventory,
                            TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                            config.clone(),
                        )
                        .style()
                        .focus_policy(FocusPolicy::Pass);
                    parent
                        .configure_text_button(
                            "Способности".to_string(),
                            Tab::Abilities,
                            TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                            config.clone(),
                        )
                        .style()
                        .focus_policy(FocusPolicy::Pass);
                    parent
                        .configure_text_button(
                            "Атаки".to_string(),
                            Tab::Attacks,
                            TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                            config,
                        )
                        .style()
                        .focus_policy(FocusPolicy::Pass);
                })
                    .style()
                    .width(Val::Percent(100.0))
                    .height(Val::Percent(50.0));

                parent.row(|parent| {
                    for (idx, member) in members.iter().enumerate() {
                        let config = ButtonConfig {
                            width: Val::Percent(100.0 / (members.len() as f32)),
                            height: Val::Percent(100.0),
                            idle: BackgroundColor::from(Color::NONE),
                            hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                            pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                            justify_content: JustifyContent::Center,
                        };
                        parent
                            .configure_text_button(
                                member.name.clone(),
                                SelectedMember(idx),
                                TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                                config,
                            )
                            .style()
                            .focus_policy(FocusPolicy::Pass);
                    };
                })
                    .style()
                    .width(Val::Percent(100.0))
                    .height(Val::Percent(50.0));
            })
                .style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(30.0));

            parent.row(|parent| {
                parent.column(|parent| {
                    parent.scroll_view(Some(ScrollAxis::Vertical), |parent| {
                        for (pos, item) in items.iter().enumerate() {
                            parent
                                .configure_text_button(
                                    &item.name,
                                    PosAndDescr(pos, item.description.clone()),
                                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                                    ButtonConfig {
                                        width: Val::Percent(100.0),
                                        height: Val::Px(70.0),
                                        idle: BackgroundColor::from(Color::NONE),
                                        hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                                        pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                                        justify_content: JustifyContent::Center,
                                    },
                                )
                                .style()
                                .focus_policy(FocusPolicy::Pass)
                                .justify_content(JustifyContent::FlexStart);
                        }
                    })
                        .style()
                        .width(Val::Percent(100.0))
                        .height(Val::Percent(100.0));
                })
                    .style()
                    .width(Val::Percent(50.0))
                    .height(Val::Percent(100.0));

                parent
                    .column(|parent| {
                        parent
                            .configure_text("".to_string(), TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                            .insert(Description)
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
                    .justify_content(JustifyContent::FlexStart)
                    .width(Val::Percent(50.0))
                    .height(Val::Percent(100.0));
            })

                .style()
                .width(Val::Percent(100.0))
                .height(Val::Percent(70.0))
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center);
        })
        .insert(InventoryAndAbilityScreen)
        .style()
        .justify_content(JustifyContent::Start)
        .size(Val::Percent(100.0))
        .background_color(Color::from(SCREEN_BG));
}

fn to_selector_items<T: GetSelectorItem>(items: &Vec<T>) -> Vec<SelectorItem> {
    let mut result = vec![];
    for item in items {
        result.push(item.selector_item());
    }
    return result;
}

fn pick_item_handle(
    mut commands: Commands,
    mut query: Query<
        (&TextButton<PosAndDescr>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut description_query: Query<(&Children), With<Description>>,
    mut holder_query: Query<(&mut SelectedItemPosHolder)>,
    mut tab_state: Res<State<Tab>>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => {
                for mut children in description_query.iter() {
                    for &child in children.iter() {
                        match commands.get_entity(child) {
                            None => { warn!("Description is not found") }
                            Some(mut entity_commands) => {
                                entity_commands.update_text(item.payload.1.clone());
                            }
                        }
                    }
                }
                *background_color = item.config.hover
            }
            Interaction::Pressed => {
                let tab = tab_state.get();
                match tab {
                    Tab::Inventory => {
                        let mut holder = holder_query.single_mut();
                        holder.store(item.payload.0);
                    }
                    _otherwise => {}
                }
            }
        }
    }
}

fn pick_tab_handle(
    mut query: Query<(&TextButton<Tab>, &Interaction, &mut BackgroundColor), Changed<Interaction>>,
    mut next_tab_state: ResMut<NextState<Tab>>,
    mut tab_state: Res<State<Tab>>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                if &item.payload == tab_state.get() {
                    *background_color = item.config.pressed
                } else {
                    *background_color = item.config.idle;
                }
            }
            Interaction::Hovered => {
                *background_color = item.config.hover
            }
            Interaction::Pressed => {
                next_tab_state.set(item.payload);
                *background_color = item.config.pressed;
            }
        }
    }
}

fn pick_member_handle(
    mut query: Query<(
        &TextButton<SelectedMember>,
        &Interaction,
        &mut BackgroundColor),
        Changed<Interaction>>
    ,
    mut next_member_state: ResMut<NextState<SelectedMember>>,
    mut member_state: Res<State<SelectedMember>>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                if item.payload.0 == member_state.get().0 {
                    *background_color = item.config.pressed
                } else {
                    *background_color = item.config.idle;
                }
            }
            Interaction::Hovered => {
                *background_color = item.config.hover
            }
            Interaction::Pressed => {
                next_member_state.set(item.payload);
                *background_color = item.config.pressed;
            }
        }
    }
}

fn despawn_main(
    mut commands: Commands,
    query: Query<Entity, With<InventoryAndAbilityScreen>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// <div style="background-color:rgb(60.0%, 44.4%, 25.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.6, 0.444, 0.25, 1.0);
/// <div style="background-color:rgb(50.0%, 39.4%, 21.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const SCREEN_BG: Srgba = Srgba::new(0.5, 0.394, 0.21, 1.0);