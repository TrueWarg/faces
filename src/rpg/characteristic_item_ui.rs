use bevy::color::Color;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::hierarchy::Children;
use bevy::log::warn;
use bevy::prelude::AlignItems;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Bundle;
use bevy::prelude::Changed;
use bevy::prelude::Commands;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::Interaction;
use bevy::prelude::JustifyContent;
use bevy::prelude::Query;
use bevy::prelude::Val;
use bevy::prelude::With;
use bevy::ui::{FocusPolicy, UiRect};
use bevy::utils::HashMap;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetFocusPolicyExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetPaddingExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::ui_commands::UpdateTextExt;

use crate::gui::TextButton;
use crate::gui::ButtonConfig;
use crate::gui::TextButtonExt;
use crate::gui::TextConfig;
use crate::gui::TextExt;
use crate::rpg::RangedProp;

#[derive(Component, Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Characteristic {
    Strength,
    Agility,
    Stamina,
    Fortitude,
    Charisma,
}

impl Characteristic {
    fn name(&self) -> String {
        let str = match self {
            Characteristic::Strength => { "Сила" }
            Characteristic::Agility => { "Выкрутасность" }
            Characteristic::Stamina => { "Стамина" }
            Characteristic::Fortitude => { "Стойкость" }
            Characteristic::Charisma => { "Языкастость" }
        };

        return str.to_string();
    }
}

impl HasDescription for Characteristic {
    fn description(&self) -> String {
        return format!("{:?}", self);
    }
}

#[derive(Component)]
pub enum CharacteristicAction {
    Increase,
    Decrease,
}

#[derive(Component)]
pub struct CharacteristicValues(
    pub HashMap<Characteristic, RangedProp>
);

#[derive(Component)]
pub struct CharacteristicValue;

#[derive(Component)]
pub struct Description;

pub trait HasDescription {
    fn description(&self) -> String;
}

pub trait CharacteristicItemExt<'a> {
    fn characteristic(
        &mut self,
        typ: Characteristic,
    ) -> UiBuilder<Entity>;
}

impl<'a> CharacteristicItemExt<'a> for UiBuilder<'a, Entity> {
    fn characteristic(
        &mut self,
        typ: Characteristic,
    ) -> UiBuilder<Entity> {
        let mut item = self.row(|parent| {
            parent
                .configure_text_button(
                    typ.name(),
                    typ,
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
                        (CharacteristicAction::Decrease, typ),
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
                        "",
                        TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    )
                    .insert((typ, CharacteristicValue))
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
                        (CharacteristicAction::Increase, typ),
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


pub fn select_item_handle<T: HasDescription + Bundle>(
    mut commands: Commands,
    mut query: Query<
        (&TextButton<T>, &Interaction, &mut BackgroundColor),
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
                                entity_commands
                                    .update_text(format!("{}", item.payload.description()));
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

