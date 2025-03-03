use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::prelude::{AlignItems, BackgroundColor, Component, Entity, JustifyContent, UiRect, Val};
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::{SetAlignItemsExt, SetBackgroundColorExt, SetFocusPolicyExt, SetJustifyContentExt, SetMarginExt, SetWidthExt, UiBuilder, UiRowExt};

use crate::gui::{ButtonConfig, TextButtonExt, TextConfig, TextExt};

#[derive(Component, Copy, Clone)]
pub struct CharacteristicId(pub usize);

#[derive(Component)]
pub enum CharacteristicAction {
    Increase,
    Decrease,
}

#[derive(Component)]
pub struct CharacteristicValue;

pub trait CharacteristicItemExt<'a> {
    fn characteristic(
        &mut self,
        id: CharacteristicId,
        name: &str,
    ) -> UiBuilder<Entity>;
}

impl<'a> CharacteristicItemExt<'a> for UiBuilder<'a, Entity> {
    fn characteristic(&mut self, id: CharacteristicId, name: &str) -> UiBuilder<Entity> {
        let mut item = self.row(|parent| {
            parent
                .configure_text(name, TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                .style()
                .width(Val::Percent(70.0));

            parent.row(|parent| {
                parent
                    .configure_text_button(
                        "-",
                        (CharacteristicAction::Decrease, id),
                        TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                        ButtonConfig {
                            width: Val::Percent(50.0),
                            height: Val::Px(50.0),
                            idle: BackgroundColor::from(Color::NONE),
                            hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                            pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        },
                    )
                    .style()
                    .focus_policy(FocusPolicy::Pass);

                parent
                    .configure_text("0", TextConfig::from_color(Color::from(ANTIQUE_WHITE)))
                    .insert(CharacteristicValue);

                parent
                    .configure_text_button(
                        "+",
                        (CharacteristicAction::Increase, id),
                        TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                        ButtonConfig {
                            width: Val::Percent(50.0),
                            height: Val::Px(50.0),
                            idle: BackgroundColor::from(Color::NONE),
                            hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                            pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        },
                    )
                    .style()
                    .focus_policy(FocusPolicy::Pass);
            })
                .style()
                .justify_content(JustifyContent::Center)
                .align_items(AlignItems::Center)
                .width(Val::Percent(30.0));;
        });

        item
            .style()
            .justify_content(JustifyContent::FlexStart)
            .align_items(AlignItems::Center);
        return item;
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.50, 0.50, 0.50, 0.7);

