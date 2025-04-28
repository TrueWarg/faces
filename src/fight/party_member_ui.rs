use bevy::color::palettes::basic::MAROON;
use bevy::color::palettes::css::{ANTIQUE_WHITE, GREEN, YELLOW};
use bevy::prelude::AlignItems;
use bevy::prelude::ButtonBundle;
use bevy::prelude::Color;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::JustifyContent;
use bevy::prelude::NodeBundle;
use bevy::prelude::Val;
use sickle_ui::prelude::SetAlignItemsExt;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetHeightExt;
use sickle_ui::prelude::SetJustifyContentExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::SetWidthExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiColumnExt;
use sickle_ui::prelude::UiContainerExt;

pub struct PartyMemberItem {
    id: MemberId,
}

#[derive(Component)]
pub struct MemberId(pub usize);

#[derive(Component)]
pub struct Health;

#[derive(Component)]
pub struct Energy;

pub trait PartyMemberItemExt<'a> {
    fn party_member_item(&mut self, id: MemberId) -> UiBuilder<Entity>;
}

impl<'a> PartyMemberItemExt<'a> for UiBuilder<'a, Entity> {
    fn party_member_item(&mut self, id: MemberId) -> UiBuilder<Entity> {
        let mut item = self.container((ButtonBundle::default(), id), |parent| {
            parent
                .column(|parent| {
                    parent
                        .container(NodeBundle::default(), |_| {})
                        .style()
                        .background_color(Color::from(MAROON))
                        .width(Val::Percent(100.0))
                        .height(Val::Percent(80.0));
                    parent
                        .container((NodeBundle::default(), Health), |_| {})
                        .style()
                        .background_color(Color::from(GREEN))
                        .width(Val::Percent(100.0))
                        .height(Val::Percent(10.0));
                    parent
                        .container((NodeBundle::default(), Energy), |_| {})
                        .style()
                        .background_color(Color::from(YELLOW))
                        .width(Val::Percent(100.0))
                        .height(Val::Percent(10.0));
                })
                .style()
                .size(Val::Percent(90.0));
        });

        item.style()
            .align_items(AlignItems::Center)
            .justify_content(JustifyContent::Center)
            .background_color(Color::from(ANTIQUE_WHITE));

        item
    }
}
