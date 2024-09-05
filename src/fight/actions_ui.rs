use bevy::color::Color;
use bevy::color::palettes::basic::SILVER;
use bevy::color::palettes::css::DIM_GREY;
use bevy::prelude::{Component, Entity, UiRect};
use bevy::ui::{BackgroundColor, Val};
use sickle_ui::prelude::{SetMarginExt, UiBuilder};

use crate::gui::{ButtonConfig, TextButtonExt, TextConfig};

#[derive(Component)]
pub struct ActionId(pub usize);

pub trait ActionItemExt<'a> {
    fn action_item<S: Into<String> + Clone>(
        &mut self,
        id: ActionId,
        text: S,
    ) -> UiBuilder<Entity>;
}

impl<'a> ActionItemExt<'a> for UiBuilder<'a, Entity> {
    fn action_item<S: Into<String> + Clone>(&mut self, id: ActionId, text: S) -> UiBuilder<Entity> {
        let mut item = self
            .configure_text_button(
                text,
                id,
                TextConfig::from_color(Color::from(SILVER)),
                ButtonConfig {
                    width: Val::Percent(95.0),
                    height: Val::Percent(20.0),
                    idle: BackgroundColor::from(DIM_GREY),
                    hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                    pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                },
            );
        item
            .style()
            .margin(
                UiRect {
                    left: Val::Px(4.0),
                    right: Val::Px(4.0),
                    top: Val::Px(4.0),
                    bottom: Val::Px(4.0),
                }
            );
        return item;
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgb(0.50, 0.50, 0.50);