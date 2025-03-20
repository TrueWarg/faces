use bevy::color::Color;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::prelude::{AlignItems, JustifyItems};
use bevy::prelude::BackgroundColor;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::JustifyContent;
use bevy::prelude::UiRect;
use bevy::prelude::Val;
use bevy::ui::FocusPolicy;
use sickle_ui::prelude::{SetAlignItemsExt, SetHeightExt, SetJustifyItemsExt};
use sickle_ui::prelude::SetFocusPolicyExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetPaddingExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiRowExt;
use crate::gui::ButtonConfig;
use crate::gui::TextButtonExt;
use crate::gui::TextConfig;
use crate::gui::TextExt;

#[derive(Component)]
pub struct Title;

#[derive(Component)]
pub enum TitleAction {
    Next,
    Back,
}

pub trait TitleExt<'a> {
    fn title(&mut self) -> UiBuilder<Entity>;
}

impl<'a> TitleExt<'a> for UiBuilder<'a, Entity> {
    fn title(&mut self) -> UiBuilder<Entity> {
        let mut item = self.row(|parent| {
            parent
                .configure_text_button(
                    "<",
                    TitleAction::Back,
                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    ButtonConfig {
                        width: Val::Percent(15.0),
                        height: Val::Px(100.0),
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
                    TextConfig::large(Color::from(ANTIQUE_WHITE)),
                )
                .insert(Title)
                .style()
                .padding(UiRect {
                    left: Val::Px(20.0),
                    right: Val::Px(20.0),
                    top: Default::default(),
                    bottom: Default::default(),
                });

            parent
                .configure_text_button(
                    ">",
                    TitleAction::Next,
                    TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    ButtonConfig {
                        width: Val::Percent(15.0),
                        height: Val::Px(100.0),
                        idle: BackgroundColor::from(Color::NONE),
                        hover: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        pressed: BackgroundColor::from(PRESSED_HOVER_BUTTON_COLOR),
                        justify_content: JustifyContent::Center,
                    },
                )
                .style()
                .focus_policy(FocusPolicy::Pass);
        });

        item
            .style()
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center);

        return item;
    }
}

/// <div style="background-color:rgb(60.0%, 44.4%, 25.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.6, 0.444, 0.25, 1.0);