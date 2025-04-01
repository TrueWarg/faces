use bevy::color::{Color, Srgba};
use bevy::color::palettes::css::ANTIQUE_WHITE;
use bevy::ecs::system::EntityCommands;
use bevy::hierarchy::Children;
use bevy::log::warn;
use bevy::prelude::AlignItems;
use bevy::prelude::BackgroundColor;
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
use sickle_ui::prelude::{LabelConfig, SetFocusPolicyExt};
use sickle_ui::prelude::ScrollAxis;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetMarginExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiRoot;
use sickle_ui::prelude::UiRowExt;
use sickle_ui::prelude::UiScrollViewExt;
use sickle_ui::ui_commands::UpdateTextExt;
use sickle_ui::widgets::layout::label::SetLabelTextExt;

use crate::fight::party_member_ui::Health;
use crate::gui::{ButtonConfig, SelectorItem, Text, TextButton, TextButtonExt, TextConfig, TextExt};

pub struct Selector;

#[derive(Component)]
pub struct PosAndDescr(pub usize, pub String);

#[derive(Component)]
pub struct SelectedItemPosHolder {
    value: Option<usize>,
}

#[derive(Component)]
pub struct Description;

impl SelectedItemPosHolder {
    pub fn new() -> Self {
        return SelectedItemPosHolder { value: None };
    }

    fn store(&mut self, value: usize) {
        self.value = Some(value);
    }

    pub fn take_away_unsafe(&mut self) -> usize {
        let value = self.value.expect("Value was not be stored");
        self.value = None;
        return value;
    }

    pub fn take_away(&mut self) -> Option<usize> {
        let value = self.value;
        self.value = None;
        return value;
    }
}

pub trait SelectorExt<'a> {
    fn selector(
        &mut self,
        items: Vec<SelectorItem>,
    ) -> UiBuilder<Entity>;
}

impl<'a> SelectorExt<'a> for UiBuilder<'a, UiRoot> {
    fn selector(&mut self, items: Vec<SelectorItem>) -> UiBuilder<Entity> {
        let mut selector = self.row(|parent| {
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
        });

        selector
            .style()
            .justify_content(JustifyContent::Center)
            .align_items(AlignItems::Center)
            .background_color(Color::from(Srgba::new(0.302, 0.302, 0.302, 0.7)));

        return selector;
    }
}

pub fn pick_item_handle<S>(
    mut commands: Commands,
    mut query: Query<
        (&TextButton<PosAndDescr>, &Interaction, &mut BackgroundColor),
        Changed<Interaction>,
    >,
    mut description_query: Query<(&Children), With<Description>>,
    mut holder_query: Query<(&mut SelectedItemPosHolder)>,
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
                let mut holder = holder_query.single_mut();
                holder.store(item.payload.0);
            }
        }
    }
}

/// <div style="background-color:rgb(30%, 30%, 30%); width: 10px; padding: 10px; border: 1px solid;"></div>
const PRESSED_HOVER_BUTTON_COLOR: Color = Color::srgba(0.50, 0.50, 0.50, 0.7);