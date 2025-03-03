use bevy::color::palettes::basic::SILVER;
use bevy::prelude::*;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiContainerExt;
use sickle_ui::ui_builder::UiBuilder;

use crate::gui::{TextConfig, TextExt};

#[derive(Component)]
pub struct TextButton<P: Bundle> {
    pub config: ButtonConfig,
    pub payload: P,
}

#[derive(Component)]
pub struct ButtonConfig {
    pub width: Val,
    pub height: Val,
    pub idle: BackgroundColor,
    pub hover: BackgroundColor,
    pub pressed: BackgroundColor,
    pub justify_content: JustifyContent,
}

impl Default for ButtonConfig {
    fn default() -> Self {
        return ButtonConfig {
            width: Val::Px(400.0),
            height: Val::Px(60.0),
            idle: BackgroundColor::from(SILVER),
            hover: BackgroundColor::from(HOVER_PRESSED_BUTTON_COLOR),
            pressed: BackgroundColor::from(HOVER_PRESSED_BUTTON_COLOR),
            justify_content: JustifyContent::Center,
        };
    }
}

pub trait TextButtonExt<'a> {
    fn text_button<S: Into<String> + Clone, P: Bundle>(
        &mut self, text: S,
        payload: P,
    ) -> UiBuilder<Entity> {
        return self.configure_text_button(
            text,
            payload,
            TextConfig::default(),
            ButtonConfig::default(),
        );
    }
    fn configure_text_button<S: Into<String> + Clone, P: Bundle>(
        &mut self,
        text: S,
        payload: P,
        text_config: TextConfig,
        button_config: ButtonConfig,
    ) -> UiBuilder<Entity>;
}

impl<'a> TextButtonExt<'a> for UiBuilder<'a, Entity> {
    fn configure_text_button<S: Into<String> + Clone, P: Bundle>(
        &mut self,
        text: S,
        payload: P,
        text_config: TextConfig,
        button_config: ButtonConfig,
    ) -> UiBuilder<Entity> {
        let width = button_config.width;
        let height = button_config.height;
        let color = button_config.idle;
        let justify_content = button_config.justify_content;
        let mut button = self.container(
            (ButtonBundle::default(), TextButton { config: button_config, payload }),
            |parent| {
                parent
                    .configure_text(text, text_config)
                    .style()
                    .margin(
                        UiRect {
                            left: Val::Px(20.0),
                            right: Val::Px(20.0),
                            top: Val::Px(20.0),
                            bottom: Val::Px(20.0),
                        }
                    );
            });
        button
            .style()
            .width(width)
            .height(height)
            .align_items(AlignItems::Center)
            .justify_content(justify_content)
            .background_color(color.0);
        return button;
    }
}

/// <div style="background-color:rgb(90%, 90%, 90%); width: 10px; padding: 10px; border: 1px solid;"></div>
const HOVER_PRESSED_BUTTON_COLOR: Color = Color::srgb(0.9, 0.9, 0.9);
const BUTTON_TEXT_SIZE_MEDIUM: f32 = 36.0;
