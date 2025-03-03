use bevy::color::Color;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::hierarchy::Children;
use bevy::log::warn;
use bevy::prelude::{AlignItems, BackgroundColor, Changed, Commands, Component, Entity, Interaction, JustifyContent, Query, Val, With};
use bevy::ui::{FocusPolicy, UiRect};
use sickle_ui::prelude::{SetAlignItemsExt, SetFocusPolicyExt, SetJustifyContentExt, SetPaddingExt, SetWidthExt, UiBuilder, UiRowExt};
use sickle_ui::ui_commands::UpdateTextExt;

use crate::gui::{ButtonConfig, TextButton, TextButtonExt, TextConfig, TextExt};
use crate::rpg::RangedProp;

#[derive(Component, Copy, Clone, Eq, PartialEq)]
pub struct CharacteristicId(pub usize);

#[derive(Component)]
pub enum CharacteristicAction {
    Increase,
    Decrease,
}

#[derive(Component)]
pub struct CharacteristicValue(pub RangedProp);

#[derive(Component)]
pub struct Description;

pub trait CharacteristicItemExt<'a> {
    fn characteristic(
        &mut self,
        id: CharacteristicId,
        name: &str,
        min: i32,
        current: i32,
        max: i32,
    ) -> UiBuilder<Entity>;
}

impl<'a> CharacteristicItemExt<'a> for UiBuilder<'a, Entity> {
    fn characteristic(
        &mut self,
        id: CharacteristicId,
        name: &str,
        min: i32,
        current: i32,
        max: i32,
    ) -> UiBuilder<Entity> {
        let mut item = self.row(|parent| {
            parent
                .configure_text_button(
                    name,
                    id,
                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    ButtonConfig {
                        width: Val::Percent(70.0),
                        height: Val::Px(50.0),
                        idle: BackgroundColor::from(Color::NONE),
                        hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        justify_content: JustifyContent::Start,
                    },
                )
                .style()
                .focus_policy(FocusPolicy::Pass);

            parent.row(|parent| {
                parent
                    .configure_text_button(
                        "-",
                        (CharacteristicAction::Decrease, id),
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
                    .focus_policy(FocusPolicy::Pass);

                parent
                    .configure_text(
                        format!("{current}"),
                        TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    )
                    .insert((id, CharacteristicValue(RangedProp {
                        min,
                        current,
                        max,
                    })))
                    .style()
                    .padding(UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        top: Default::default(),
                        bottom: Default::default(),
                    });

                parent
                    .configure_text_button(
                        "+",
                        (CharacteristicAction::Increase, id),
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
                    .focus_policy(FocusPolicy::Pass);
            })
                .style()
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center)
                .width(Val::Percent(30.0));
        });

        item
            .style()
            .justify_content(JustifyContent::FlexStart)
            .align_items(AlignItems::Center);
        return item;
    }
}

pub fn change_value_handle(
    mut commands: Commands,
    mut query: Query<
        (&TextButton<(CharacteristicAction, CharacteristicId)>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>>,
    mut values_query: Query<(&Children, &mut CharacteristicValue, &CharacteristicId)>,
) {
    for (item, interaction, mut background_color) in &mut query {
        match *interaction {
            Interaction::None => {
                *background_color = item.config.idle;
            }
            Interaction::Hovered => {
                *background_color = item.config.hover
            }
            Interaction::Pressed => {
                for (mut children, mut value, id) in values_query.iter_mut() {
                    if *id != item.payload.1 {
                        continue;
                    }
                    match item.payload.0 {
                        CharacteristicAction::Increase => {
                            value.0.increase(1);
                        }
                        CharacteristicAction::Decrease => {
                            value.0.decrease(1)
                        }
                    };

                    for &child in children.iter() {
                        match commands.get_entity(child) {
                            None => { warn!("CharacteristicValue is not found") }
                            Some(mut entity_commands) => {
                                entity_commands.update_text(format!("{}", value.0.current));
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn select_item_handle(
    mut commands: Commands,
    mut query: Query<
        (&TextButton<CharacteristicId>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut description_query: Query<(&Children), With<Description>>,
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
                                entity_commands.update_text(format!("{}", item.payload.0));
                            }
                        }
                    }
                }
                *background_color = item.config.hover
            }
            Interaction::Pressed => {}
        }
    }
}


/// <div style="background-color:rgb(60.0%, 44.4%, 25.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.6, 0.444, 0.25, 1.0);

