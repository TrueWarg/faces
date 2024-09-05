use bevy::color::palettes::css::{ANTIQUE_WHITE, GREEN};
use bevy::prelude::ButtonBundle;
use bevy::prelude::Color;
use bevy::prelude::Component;
use bevy::prelude::Entity;
use bevy::prelude::NodeBundle;
use bevy::prelude::Val;
use sickle_ui::prelude::SetBackgroundColorExt;
use sickle_ui::prelude::SetSizeExt;
use sickle_ui::prelude::UiBuilder;
use sickle_ui::prelude::UiContainerExt;

#[derive(Component)]
pub struct EnemyId(pub usize);

pub trait EnemyItemExt<'a> {
    fn enemy_item(
        &mut self,
        id: EnemyId,
    ) -> UiBuilder<Entity>;
}

impl<'a> EnemyItemExt<'a> for UiBuilder<'a, Entity> {
    fn enemy_item(&mut self, id: EnemyId) -> UiBuilder<Entity> {
        let mut item = self.container(
            (ButtonBundle::default(), id),
            |parent| {
                parent
                    .container(NodeBundle::default(), |_| {})
                    .style()
                    .size(Val::Percent(90.0))
                    .background_color(Color::from(GREEN))
                ;
            });

        item
            .style()
            .background_color(Color::from(ANTIQUE_WHITE));

        return item;
    }
}