use bevy::color::Color;
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::prelude::AlignItems;
use bevy::prelude::BackgroundColor;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::JustifyContent;
use bevy::prelude::Val;
use bevy::ui::{FocusPolicy, UiRect};
use bevy::utils::HashMap;
use sickle_ui::prelude::SetAlignItemsExt;
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
use crate::rpg::characteristic_item_ui::HasDescription;

#[derive(Component, Hash, Copy, Clone, Eq, PartialEq, Debug)]
pub enum Stat {
    Health,
    Energy,
    Armor,
    Evasion,
    BaseAttack,
}

impl Stat {
    fn name(&self) -> String {
        let str = match self {
            Stat::Health => { "Здоровье" }
            Stat::Energy => { "Энергия" }
            Stat::Armor => { "Броня" }
            Stat::Evasion => { "Уклонение" }
            Stat::BaseAttack => { "Базовый урон" }
        };

        return str.to_string();
    }
}

impl HasDescription for Stat {
    fn description(&self) -> String {
        return format!("{:?}", self);
    }
}

#[derive(Component)]
pub struct StatsValues(
    pub HashMap<Stat, i32>
);

#[derive(Component)]
pub struct StatValue;

pub trait StatItemExt<'a> {
    fn stat(
        &mut self,
        typ: Stat,
    ) -> UiBuilder<Entity>;
}

impl<'a> StatItemExt<'a> for UiBuilder<'a, Entity> {
    fn stat(
        &mut self,
        typ: Stat,
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
                    .configure_text(
                        "",
                        TextConfig::from_color(Color::from(ANTIQUE_WHITE)),
                    )
                    .insert((typ, StatValue))
                    .style()
                    .padding(UiRect {
                        left: Val::Px(20.0),
                        right: Val::Px(20.0),
                        top: Default::default(),
                        bottom: Default::default(),
                    });
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


/// <div style="background-color:rgb(60.0%, 44.4%, 25.0%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.6, 0.444, 0.25, 1.0);

